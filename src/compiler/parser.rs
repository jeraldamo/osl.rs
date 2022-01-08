use crate::compiler::*;
use crate::compiler::Token::*;
use crate::compiler::ast::*;
use crate::compiler::lexer::*;
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

    ExpressionList: Vec<Expr> {
        Expression[x] => vec![x],
        ExpressionList[mut a] Comma Expression[x] => {
            a.push(x);
            a
        }
    }

    OptExpressionList: Vec<Expr> {
        ExpressionList[x] => x,
        => vec![],
    }

    Expression: Expr {
        VariableAssignment[x] => x,
        LogicalOrExpression[x] => x,
        FunctionCall[x] => x,
        // PointConstructorExpression[x] => x,
    }

    OptExpression: Expr {
        Expression[x] => x,
        => Expr {
            span: Span{hi: 0, lo: 0, line: 0},
            node: Expr_::EmptyExpression,
        }
    }

    FunctionCall: Expr {
        Identifier[name] LeftParen ExpressionList[arguments] RightParen => Expr {
            span: span!(),
            node: Expr_::FunctionCallExpression {
                name: Box::new(name),
                arguments: Box::new(arguments),
            }
        },
        VariableType[name] LeftParen ExpressionList[arguments] RightParen => Expr {
            span: span!(),
            node: Expr_::FunctionCallExpression {
                name: Box::new(name),
                arguments: Box::new(arguments),
            }
        }
    }

    // PointConstructorExpression: Expr {
    //     VariableType[point_type] LeftParen Expression[x] Comma Expression[y] Comma Expression[z] RightParen => Expr {
    //         span: span!(),
    //         node: Expr_::PointConstructor {
    //             point_type: Box::new(point_type),
    //             x: Box::new(x),
    //             y: Box::new(y),
    //             z: Box::new(z),
    //             space: None,
    //         }
    //     },
    //     VariableType[point_type] LeftParen Expression[space] Comma Expression[x] Comma Expression[y] Comma Expression[z] RightParen => Expr {
    //         span: span!(),
    //         node: Expr_::PointConstructor {
    //             point_type: Box::new(point_type),
    //             x: Box::new(x),
    //             y: Box::new(y),
    //             z: Box::new(z),
    //             space: Some(Box::new(space)),
    //         }
    //     }
    // }

    LogicalOrExpression: Expr {
        LogicalOrExpression[lhs] OPLogicalOr LogicalAndExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::LogicalOr, Box::new(lhs), Box::new(rhs)),
        },
        LogicalAndExpression[x] => x,
    }

    LogicalAndExpression: Expr {
        LogicalAndExpression[lhs] OPLogicalAnd BitwiseOrExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::LogicalAnd, Box::new(lhs), Box::new(rhs)),
        },
        BitwiseOrExpression[x] => x,
    }

    BitwiseOrExpression: Expr {
        BitwiseOrExpression[lhs] OPBitwiseOr BitwiseXorExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::BitwiseOr, Box::new(lhs), Box::new(rhs)),
        },
        BitwiseXorExpression[x] => x,
    }

    BitwiseXorExpression: Expr {
        BitwiseXorExpression[lhs] OPBitwiseXor BitwiseAndExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::BitwiseXor, Box::new(lhs), Box::new(rhs)),
        },
        BitwiseAndExpression[x] => x,
    }

    BitwiseAndExpression: Expr {
        BitwiseAndExpression[lhs] OPBitwiseAnd EqualityExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::BitwiseAnd, Box::new(lhs), Box::new(rhs)),
        },
        EqualityExpression[x] => x,
    }

    EqualityExpression: Expr {
        EqualityExpression[lhs] OPEquals ComparisonExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Equals, Box::new(lhs), Box::new(rhs)),
        },
        EqualityExpression[lhs] OPNotEqual ComparisonExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::NotEqual, Box::new(lhs), Box::new(rhs)),
        },
        ComparisonExpression[x] => x,
    }

    ComparisonExpression: Expr {
        ComparisonExpression[lhs] OPLessThan ShiftExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::LessThan, Box::new(lhs), Box::new(rhs)),
        },
        ComparisonExpression[lhs] OPLessThanEqual ShiftExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::LessThanEqual, Box::new(lhs), Box::new(rhs)),
        },
        ComparisonExpression[lhs] OPGreaterThan ShiftExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::GreaterThan, Box::new(lhs), Box::new(rhs)),
        },
        ComparisonExpression[lhs] OPGreaterThanEqual ShiftExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::GreaterThanEqual, Box::new(lhs), Box::new(rhs)),
        },
        ShiftExpression[x] => x,
    }

    ShiftExpression: Expr {
        ShiftExpression[lhs] OPShiftLeft AdditiveExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::ShiftLeft, Box::new(lhs), Box::new(rhs)),
        },
        ShiftExpression[lhs] OPShiftRight AdditiveExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::ShiftRight, Box::new(lhs), Box::new(rhs)),
        },
        AdditiveExpression[x] => x,
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
        MultiplicativeExpression[lhs] OPMultiply PreUnaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Multiply, Box::new(lhs), Box::new(rhs)),
        },
        MultiplicativeExpression[lhs] OPDivide PreUnaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Divide, Box::new(lhs), Box::new(rhs)),
        },
        MultiplicativeExpression[lhs] OPMod PreUnaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::BinaryExpression(Operators::Mod, Box::new(lhs), Box::new(rhs)),
        },
        PreUnaryExpression[x] => x,
    }

    PreUnaryExpression: Expr {
        OPIncrement PostUnaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::PreUnaryExpression(Operators::Increment, Box::new(rhs)),
        },
        OPDecrement PostUnaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::PreUnaryExpression(Operators::Decrement, Box::new(rhs)),
        },
        OPMinus PostUnaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::PreUnaryExpression(Operators::Minus, Box::new(rhs)),
        },
        OPBitwiseCompliment PostUnaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::PreUnaryExpression(Operators::BitwiseCompliment, Box::new(rhs)),
        },
        OPNot PostUnaryExpression[rhs] => Expr {
            span: span!(),
            node: Expr_::PreUnaryExpression(Operators::Not, Box::new(rhs)),
        },
        PostUnaryExpression[x] => x,
    }

    PostUnaryExpression: Expr {
        AccessExpression[lhs] OPIncrement => Expr {
            span: span!(),
            node: Expr_::PostUnaryExpression(Operators::Increment, Box::new(lhs)),
        },
        AccessExpression[lhs] OPDecrement => Expr {
            span: span!(),
            node: Expr_::PostUnaryExpression(Operators::Decrement, Box::new(lhs)),
        },
        AccessExpression[x] => x,
    }

    AccessExpression: Expr {
        PrimaryExpression[lhs] LeftSquare IntLiteral[val] RightSquare => Expr {
            span: span!(),
            node: Expr_::AccessExpression {
                lhs: Box::new(lhs),
                value: Box::new(val),
                dot: false,
            }
        },
        PrimaryExpression[lhs] Period Identifier[val] => Expr {
            span: span!(),
            node: Expr_::AccessExpression {
                lhs: Box::new(lhs),
                value: Box::new(val),
                dot: true,
            }
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
        IntLiteral[x] => x,
        FloatLiteral[x] => x,
        StringLiteral[x] => x,
    }

    IntLiteral: Expr {
        Integer(i) => Expr {
            span: span!(),
            node: Expr_::IntLiteral(i),
        }
    }

    FloatLiteral: Expr {
        Float(i) => Expr {
            span: span!(),
            node: Expr_::FloatLiteral(i),
        }
    }

    StringLiteral: Expr {
        Str(i) => Expr {
            span: span!(),
            node: Expr_::StringLiteral(i),
        }
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