use std::collections::HashMap;
// use inkwell::{
//     context::Context,
//     builder::Builder,
//     passes::PassManager,
//     module::Module,
//     values::{FunctionValue, PointerValue}};


use crate::{Types, Span};
use crate::ast::*;
use crate::errors::*;
use crate::symtab::SymbolTable;


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

        self.build_symbols(self.program)?;

        self.check_semantics();

        println!("{}", self.symbol_table);

        Ok(vec![])
    }

    fn check_semantics(&self) -> Result<(), OSLCompilerError> {


        Ok(())
    }

    fn build_symbols(&mut self, stmts: &Vec<Stmt>) -> Result<(), OSLCompilerError> {

        for stmt in stmts {
            match &stmt.statement {
                Stmt_::VariableDeclaration{var_type, name,..} => {
                    self.symbol_table.add_variable(get_var_type_value(var_type).unwrap(),
                                                   get_ident_value(name).unwrap(),
                                                   stmt.span,
                                                   false)?;
                },

                Stmt_::ShaderDeclaration{name, shader_type, params, body, ..} => {
                    self.symbol_table.add_shader(get_shader_type_value(shader_type).unwrap(),
                                                   get_ident_value(name).unwrap(),
                                                   stmt.span);

                    self.symbol_table.up_scope(stmt.span);

                    for param in params {
                        match param.clone().node {
                            Expr_::Parameter {par_type, name, out, ..} => {
                                self.symbol_table.add_variable(get_var_type_value(&par_type).unwrap(),
                                   get_ident_value(&name).unwrap(),
                                   param.span,
                                   out);
                            }
                            _ => {}
                        }
                    }

                    match body.clone().statement {
                        Stmt_::BlockStatement(block_stmts) => {
                            self.build_symbols(&block_stmts);
                        }
                        _ => {}
                    }

                    self.symbol_table.down_scope();
                },

                Stmt_::FunctionDeclaration{name, ret_type, params, body} => {
                    self.symbol_table.add_function(get_var_type_value(ret_type).unwrap(),
                        get_ident_value(name).unwrap(),
                        stmt.span,
                        false);

                    self.symbol_table.up_scope(stmt.span);

                    for param in params {
                        match param.clone().node {
                            Expr_::Parameter {par_type, name, out, ..} => {
                                self.symbol_table.add_variable(get_var_type_value(&par_type).unwrap(),
                                    get_ident_value(&name).unwrap(),
                                    param.span,
                                    out);
                            }
                            _ => {}
                        }
                    }

                    match body.clone().statement {
                        Stmt_::BlockStatement(block_stmts) => {
                            self.build_symbols(&block_stmts);
                        }
                        _ => {}
                    }

                    self.symbol_table.down_scope();
                },

                Stmt_::BlockStatement(block_stmts) => {
                    self.symbol_table.up_scope(stmt.span);
                    self.build_symbols(block_stmts);
                    self.symbol_table.down_scope();
                },
                _ => {}
            }
        }

        Ok(())
    }

}