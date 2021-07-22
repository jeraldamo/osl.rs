use std::collections::HashMap;
use inkwell::{
    context::Context,
    builder::Builder,
    passes::PassManager,
    module::Module,
    values::{FunctionValue, PointerValue}};


use crate::{Types, Span};
use crate::ast::*;
use crate::errors::*;
use std::thread::current;




#[derive(Debug, Clone)]
struct Variable {
    name: String,
    var_type: Types,
    span: Span,
    scope: u64,
}

pub struct Compiler<'a> {//, 'ctx> {
    //pub context: &'ctx Context,
    //pub builder: &'a Builder<'ctx>,
    //pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    //pub module: &'a Module<'ctx>,
    pub program: &'a Vec<Stmt>,

    variables: HashMap<String, Vec<Variable>>,
}

impl<'a> Compiler<'a> {
    pub fn new(//context: &'ctx Context,
        //builder: &'a Builder<'ctx>,
        //module: &'a Module<'ctx>,
        program: &'a Vec<Stmt>) -> Self {

        Self {
            //context,
            //builder,
            //module,
            program,
            variables: HashMap::new(),
        }
    }

    pub fn compile(&mut self) -> Result<Vec<u8>, OSLCompilerError> {

        self.build_variables()?;

        println!("\nVariables:");
        for (key, val) in self.variables.iter() {
            println!("{}: {:?}", key, val);
        }

        Ok(vec![])
    }

    pub fn check_semantics(&self) -> Result<(), OSLCompilerError> {


        Ok(())
    }

    fn build_variables(&mut self) -> Result<(), OSLCompilerError> {

        let scope_stack: Vec<u64> = vec![1];
        let cur_scope: u64 = 1;
        let next_scope: u64 = cur_scope << 1;

        for stmt in self.program {
            match &stmt.statement {
                Stmt_::VariableDeclaration{var_type, name,..} => {

                    let var_name = get_ident_value(name).unwrap();
                    let var = Variable {
                        name: var_name.clone(),
                        var_type: get_var_type_value(var_type).unwrap(),
                        span: stmt.span,
                        scope: cur_scope,
                    };

                    if self.variables.contains_key(var_name.clone().as_str()) {
                        let vars = self.variables.get_mut(var_name.clone().as_str()).unwrap();
                        for v in vars.clone() {
                            if v.scope == var.scope {
                                return Err(OSLCompilerError::ExistingVariable (
                                    Item::new(v.span, v.name),
                                    Item::new(var.span, var.name),
                                ));
                            }
                        }
                        vars.push(var);
                    }
                    else {
                        self.variables.insert(var_name.clone(), vec![var]);
                    }
                },
                Stmt_::ExpressionStatement(expr) => {

                },
                Stmt_::BlockStatement(stmts) => {

                },
                Stmt_::ShaderDeclaration{body, ..} => {

                },
                Stmt_::FunctionDeclaration{body, ..} => {

                },
                _ => {}
            }
        }

        Ok(())
    }

    fn build_variables_recursive(&self, cur_scope: Vec<u64>, next_scope: u64) -> u64 {
        0
    }

}