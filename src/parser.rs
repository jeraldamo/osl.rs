use crate::*;
use crate::Token::*;
use crate::ast::*;
use crate::lexer::*;
use plex::parser;

parser! {
    fn parse_(Token, Span);

    // Combine two spans
    (a, b) {
        Span {
            lo: a.lo,
            hi: b.hi,
            line: a.line,
        }
    }

    // Entry NT
    Program: Vec<Stmt> {
        StatementList[stmts] => stmts,
    }

    // Recursive NT for a list of statements
    StatementList: Vec<Stmt> {
        Statement[s] => vec![s],
        StatementList[mut stmts] Statement[s] => {
            stmts.push(s);
            stmts
        }
    }

    OptStatementList: Vec<Stmt> {
        StatementList[stmts] => stmts,
        => vec![],
    }

    // Breaks into the different statement types
    Statement: Stmt {
        ExpressionStatement[s] => s,
        BlockStatement[s] => s,
        VariableDeclaration[s] => s,
        FunctionDeclaration[s] => s,
        ReturnStatement[s] => s,
        StructDeclaration[s] => s,
        ShaderDeclaration[s] => s,
    }

    // An expression ending in a semicolon
    ExpressionStatement: Stmt {
        OptExpression[x] Semicolon => Stmt {
            span: span!(),
            statement: Stmt_::ExpressionStatement(x),
        },
    }

    BlockStatement: Stmt {
        LeftCurly OptStatementList[stmts] RightCurly => Stmt {
            span: span!(),
            statement: Stmt_::BlockStatement(Box::new(stmts)),
        }
    }

    VariableDeclaration: Stmt {
        VariableType[var_type] Identifier[name] OptAssignment[value] Semicolon=> Stmt {
            span: span!(),
            statement: Stmt_::VariableDeclaration {
                var_type,
                name,
                value,
            }
        },
    }


    FunctionDeclaration: Stmt {
        VariableType[ret_type] Identifier[name] LeftParen OptFormalParameterList[params] RightParen BlockStatement[s] => Stmt {
            span: span!(),
            statement: Stmt_::FunctionDeclaration {
                name,
                ret_type,
                params,
                body: Box::new(s),
            }
        },
    }

    ShaderDeclaration: Stmt {
        ShaderType[shader_type] Identifier[name] LeftParen OptFormalParameterList[params] RightParen BlockStatement[s] => Stmt {
            span: span!(),
            statement: Stmt_::ShaderDeclaration {
                name,
                shader_type,
                params,
                body: Box::new(s),
            }
        }
    }

    ReturnStatement: Stmt {
        KWReturn OptExpression[x] Semicolon => Stmt {
            span: span!(),
            statement: Stmt_::ReturnStatement(x),
        }
    }

    StructDeclaration: Stmt {
        KWStruct Identifier[name] BlockStatement[block] => Stmt {
            span: span!(),
            statement: Stmt_::StructDeclaration {
                name,
                body: Box::new(block),
            }
        }
    }


    //===============
    // Expressions
    //===============

    VariableAssignment: Expr {
        Identifier[s] Assignment[x] => Expr {
            span: span!(),
            node: Expr_::Assignment(Box::new(s), Box::new(x)),
        }
    }

    Assignment: Expr {
        OPAssign Expression[x] => x,
    }

    OptAssignment: Expr {
        Assignment[x] => x,
        => Expr {
            span: Span{hi: 0, lo: 0, line: 0},
            node: Expr_::EmptyExpression,
        }
    }

    Identifier: Expr {
        Ident(s) => Expr {
            span: span!(),
            node: Expr_::Ident(s),
        },
    }

    VariableType: Expr {
        Type(s) => Expr {
            span: span!(),
            node: Expr_::VariableType(s),
        },
    }

    ShaderType: Expr {
        Shader(s) => Expr {
            span: span!(),
            node: Expr_::ShaderType(s),
        },
    }

    Parameter: Expr {
        VariableType[par_type] Identifier[name] OptAssignment[value] => Expr {
            span: span!(),
            node: Expr_::Parameter {
                par_type: Box::new(par_type),
                name: Box::new(name),
                out: false,
                value: Box::new(value),
            }
        },
        KWOutput VariableType[par_type] Identifier[name] OptAssignment[value] => Expr {
            span: span!(),
            node: Expr_::Parameter {
                par_type: Box::new(par_type),
                name: Box::new(name),
                out: true,
                value: Box::new(value),
            }
        }
    }

    FormalParameterList: Vec<Expr> {
        Parameter[x] => vec![x],
        FormalParameterList[mut p] Comma Parameter[x] => {
            p.push(x);
            p
        }
    }

    OptFormalParameterList: Vec<Expr> {
        FormalParameterList[x] => x,
        => vec![],
    }

    Expression: Expr {
        VariableAssignment[x] => x,
        AdditiveExpression[x] => x,
    }

    OptExpression: Expr {
        Expression[x] => x,
        => Expr {
            span: Span{hi: 0, lo: 0, line: 0},
            node: Expr_::EmptyExpression,
        }
    }

    AdditiveExpression: Expr {
        AdditiveExpression[lhs] OPPlus MultiplicativeExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Plus, Box::new(lhs), Box::new(rhs)),
        },
        AdditiveExpression[lhs] OPMinus MultiplicativeExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Minus, Box::new(lhs), Box::new(rhs)),
        },
        MultiplicativeExpression[x] => x,
    }

    MultiplicativeExpression: Expr {
        MultiplicativeExpression[lhs] OPMultiply PrimaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Multiply, Box::new(lhs), Box::new(rhs)),
        },
        MultiplicativeExpression[lhs] OPDivide PrimaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Divide, Box::new(lhs), Box::new(rhs)),
        },
        MultiplicativeExpression[lhs] OPMod PrimaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Mod, Box::new(lhs), Box::new(rhs)),
        },
        PrimaryExpression[x] => x,
    }

    PrimaryExpression: Expr {
        Literal[x] => x,
        Identifier[x] => x,
        GlobalVariable[x] => x,
        ParenthesizedExpression[x] => x,
    }

    GlobalVariable: Expr {
        Global(i) => Expr {
            span: span!(),
            node: Expr_::GlobalVariable(i),
        }
    }

    Literal: Expr {
        Integer(i) => Expr {
            span: span!(),
            node: Expr_::IntLiteral(i),
        },
        Float(i) => Expr {
            span: span!(),
            node: Expr_::FloatLiteral(i),
        },
        Str(i) => Expr {
            span: span!(),
            node: Expr_::StringLiteral(i),
        },
    }

    ParenthesizedExpression: Expr {
        LeftParen Expression[x] RightParen => x,
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Vec<Stmt>, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}