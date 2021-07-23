use crate::{Span, Types, Operators, ShaderTypes, Globals};

#[derive(Debug, Clone)]
pub struct Stmt {
    pub span: Span,
    pub statement: Stmt_,
}

#[derive(Debug, Clone)]
pub enum Stmt_ {
    ExpressionStatement(Expr),
    EmptyStatement,
    BlockStatement(Box<Vec<Stmt>>),
    FunctionDeclaration {
        name: Expr,
        ret_type: Expr,
        params: Vec<Expr>,
        body: Box<Stmt>,
        // public: bool,
    },
    VariableDeclaration {
        var_type: Expr,
        name: Expr,
        value: Expr,
    },
    ReturnStatement(Expr),
    StructDeclaration {
        name: Expr,
        body: Box<Stmt>,
    },
    ShaderDeclaration {
        name: Expr,
        shader_type: Expr,
        params: Vec<Expr>,
        body: Box<Stmt>,
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Span,
    pub node: Expr_,
}

#[derive(Debug, Clone)]
pub enum Expr_ {
    BinaryExpression(Operators, Box<Expr>, Box<Expr>),
    Assignment(Box<Expr>, Box<Expr>),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    GlobalVariable(Globals),
    Ident(String),
    EmptyExpression,
    VariableType(Types),
    ShaderType(ShaderTypes),
    Parameter {
        par_type: Box<Expr>,
        name: Box<Expr>,
        out: bool,
        value: Box<Expr>,
    }
}

pub fn get_ident_value(expr: &Expr) -> Option<String> {
    match &expr.node {
        Expr_::Ident(s) => Some(s.clone()),
        _ => None,
    }
}

pub fn get_var_type_value(expr: &Expr) -> Option<Types> {
    match &expr.node {
        Expr_::VariableType(t) => Some(t.clone()),
        _ => None,
    }
}