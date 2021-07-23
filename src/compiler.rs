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
use crate::symtab::SymbolTable;
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

    symbol_table: SymbolTable,
}

impl<'a> Compiler<'a> {
    pub fn new(//context: &'ctx Context,
        //builder: &'a Builder<'ctx>,
        //module: &'a Module<'ctx>,
        program: &'a Vec<Stmt>,
        program_size: usize) -> Self {

        Self {
            //context,
            //builder,
            //module,
            program,
            symbol_table: SymbolTable::new(program_size),
        }
    }

    pub fn compile(&mut self) -> Result<Vec<u8>, OSLCompilerError> {

        self.build_variables()?;

        // println!("\nVariables:");
        // for (key, val) in self.variables.iter() {
        //     println!("{}: {:?}", key, val);
        // }

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
                    self.symbol_table.add_variable(get_var_type_value(var_type).unwrap(),
                                                   get_ident_value(name).unwrap(),
                                                   stmt.span,
                                                   false)?;
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