use std::collections::HashMap;
// use inkwell::{
//     context::Context,
//     builder::Builder,
//     passes::PassManager,
//     module::Module,
//     values::{FunctionValue, PointerValue}};


use crate::compiler::{Types, Span, Token};
use crate::compiler::ast::*;
use crate::compiler::symtab::SymbolTable;
use crate::errors::*;

pub struct Compiler<'a> {//, 'ctx> {
    //pub context: &'ctx Context,
    //pub builder: &'a Builder<'ctx>,
    //pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    //pub module: &'a Module<'ctx>,
    pub tokens: Vec<(Token, Span)>,
    pub program: &'a Vec<Stmt>,

    symbol_table: SymbolTable,
}

impl<'a> Compiler<'a> {
    pub fn new(//context: &'ctx Context,
        //builder: &'a Builder<'ctx>,
        //module: &'a Module<'ctx>,
        tokens: Vec<(Token, Span)>,
        program: &'a Vec<Stmt>,
        program_size: usize) -> Self {

        Self {
            //context,
            //builder,
            //module,
            tokens,
            program,
            symbol_table: SymbolTable::new(program_size),
        }
    }

    pub fn compile(&mut self) -> Result<Vec<u8>, OSLCompilerError> {

        self.build_symbols(self.program)?;

        self.check_semantics()?;

        println!("{}", self.symbol_table);

        Ok(vec![])
    }

    fn check_semantics(&mut self) -> Result<(), OSLCompilerError> {
        // Make sure that the program has one and only one shader function
        if self.symbol_table.n_shaders == 0 {
            return Err(OSLCompilerError::MissingShader);
        }
        else if self.symbol_table.n_shaders > 1 {
            return Err(OSLCompilerError::MultipleShaders);
        }

        // Cycle through tokens and make sure that any ident token references a symbol in the
        // symbol table and has access to it.
        for (token, span) in self.tokens.clone() {
            match token {
                Token::Ident(s) => {
                    self.symbol_table.check_access(span, s.clone())?;
                },
                _ => {}
            }
        }

        self.check_types(&self.program.clone())?;

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
                                                   stmt.span)?;

                    self.symbol_table.up_scope(Span{lo: name.span.hi, hi: stmt.span.hi, line: 0});

                    for param in params {
                        match param.clone().node {
                            Expr_::Parameter {par_type, name, out, ..} => {
                                self.symbol_table.add_variable(get_var_type_value(&par_type).unwrap(),
                                   get_ident_value(&name).unwrap(),
                                   param.span,
                                   out)?;
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
                        Vec::new(),
                        stmt.span,
                        false)?;


                    //                                         VV Bug??
                    self.symbol_table.up_scope(Span{lo: name.span.hi, hi: stmt.span.hi, line: 0});

                    for param in params {
                        match param.clone().node {
                            Expr_::Parameter {par_type, name, out, ..} => {
                                self.symbol_table.add_variable(get_var_type_value(&par_type).unwrap(),
                                    get_ident_value(&name).unwrap(),
                                    param.span,
                                    out)?;
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
                    if self.symbol_table.cur_scope == 1 {
                        return Err(OSLCompilerError::GlobalScopeBlock {
                            block : Item::new(stmt.span, "")
                        });
                    }
                    self.symbol_table.up_scope(stmt.span);
                    self.build_symbols(block_stmts);
                    self.symbol_table.down_scope();
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn check_types(&mut self, stmts: &Vec<Stmt>) -> Result<(), OSLCompilerError> {
        for stmt in stmts {
            match &stmt.statement {
                Stmt_::ExpressionStatement(expr) => { ;
                    get_expr_type(expr, &self.symbol_table)?;
                },

                Stmt_::VariableDeclaration {name, value, ..} => {

                    match value.node {
                        Expr_::EmptyExpression => {},

                        _ => {
                            let expr = Expr {
                                span: Span{lo: name.span.lo, hi: value.span.hi, line: name.span.line},
                                node: Expr_::Assignment(
                                    Box::new(name.clone()),
                                    Box::new(value.clone())),
                            };

                            get_expr_type(&expr,&self.symbol_table)?;
                        }
                    }
                }

                Stmt_::ShaderDeclaration {body, ..} => {
                    match &body.statement {
                        Stmt_::BlockStatement(stmts) => {
                            self.check_types(&stmts.clone())?;
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        Ok(())
    }

}