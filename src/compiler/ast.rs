use crate::compiler::{Span, Types, Operators, ShaderTypes, Globals};
use crate::compiler::symtab::*;
use crate::errors::*;


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
    PreUnaryExpression(Operators, Box<Expr>),
    PostUnaryExpression(Operators, Box<Expr>),
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
    },
    PointConstructor {
        point_type: Box<Expr>,
        x: Box<Expr>,
        y: Box<Expr>,
        z: Box<Expr>,
        space: Option<Box<Expr>>,
    },
    AccessExpression {
        lhs: Box<Expr>,
        value: Box<Expr>,
        dot: bool,
    },
    FunctionCallExpression {
        name: Box<Expr>,
        arguments: Box<Vec<Expr>>,
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

pub fn get_shader_type_value(expr: &Expr) -> Option<ShaderTypes> {
    match &expr.node {
        Expr_::ShaderType(t) => Some(t.clone()),
        _ => None,
    }
}

pub fn get_expr_type(expr: &Expr, symbols: &SymbolTable) -> Result<Types, OSLCompilerError> {
    match &expr.node {
        Expr_::AccessExpression {lhs, value, dot} => {
            let lhs_type = get_expr_type(lhs, symbols)?;

            return if *dot {
                let access_value = match value.node.clone() {
                    Expr_::Ident(s) => s,
                    _ => String::new(),
                };

                match lhs_type {
                    Types::Color => {
                        match access_value.as_str() {
                            "r" | "b" | "g" => Ok(Types::Float),
                            _ => Err(OSLCompilerError::GenericError(Item::new(expr.span, "Bad value"))),
                        }
                    },

                    Types::Point |
                    Types::Vector |
                    Types::Normal => {
                        match access_value.as_str() {
                            "x" | "y" | "z" => Ok(Types::Float),
                            _ => Err(OSLCompilerError::GenericError(Item::new(expr.span, "Bad value"))),
                        }
                    },

                    _ => Err(OSLCompilerError::GenericError(Item::new(expr.span, "Bad value"))),
                }
            } else {
                let access_value = match value.node.clone() {
                    Expr_::IntLiteral(i) => i,
                    _ => -1,
                };

                match lhs_type {
                    Types::Color |
                    Types::Point |
                    Types::Vector |
                    Types::Normal => {
                        match access_value {
                            0..3 => Ok(Types::Float),
                            _ => Err(OSLCompilerError::GenericError(Item::new(expr.span, "Bad value"))),
                        }
                    },

                    Types::Matrix => {
                        match access_value {
                            0..16 => Ok(Types::Float),
                            _ => Err(OSLCompilerError::GenericError(Item::new(expr.span, "Bad value"))),
                        }
                    },

                    _ => Err(OSLCompilerError::GenericError(Item::new(expr.span, "Bad value"))),
                }
            };
        },

        Expr_::FunctionCallExpression {name, arguments} => {

            // let arg_types: Vec<Types> = arguments.iter()
            //     .map(|expr| {get_expr_type(expr, symbols)?})
            //     .collect();
            //
            // let symbol = symbols.get_reference(expr.span, s.clone());
            // return match symbol {
            //     Symbols::Variable{var_type, ..} => Ok(var_type),
            //     _ => Ok(Types::Void),
            // }

            //let symbol = symbols.get_reference(name.span, name.n)

            return match &name.node {
                Expr_::VariableType(t) => Ok(*t),
                _ => Ok(Types::Void),
            }
        }

        Expr_::PointConstructor {point_type, x, y, z, space} => {
            let x_type = get_expr_type(x, symbols)?;
            match x_type {
                Types::Float | Types::Int => {},
                _ => return Err(OSLCompilerError::MismatchedTypesArgument {
                    expected: Item::new(x.span, "Float"),
                    received: Item::new(x.span, format!("{:?}", x_type)),
                })
            }

            let y_type = get_expr_type(y, symbols)?;
            match y_type {
                Types::Float | Types::Int => {},
                _ => return Err(OSLCompilerError::MismatchedTypesArgument {
                    expected: Item::new(y.span, "Float"),
                    received: Item::new(y.span, format!("{:?}", y_type)),
                })
            }

            let z_type = get_expr_type(z, symbols)?;
            match z_type {
                Types::Float | Types::Int => {},
                _ => return Err(OSLCompilerError::MismatchedTypesArgument {
                    expected: Item::new(z.span, "Float"),
                    received: Item::new(z.span, format!("{:?}", z_type)),
                })
            }

            match space {
                None => {},
                Some(s) => {
                    let space_type = get_expr_type(s, symbols)?;
                    match space_type {
                        Types::String => {},
                        _ => return Err(OSLCompilerError::MismatchedTypesArgument {
                            expected: Item::new(s.span, "String"),
                            received: Item::new(s.span, format!("{:?}", space_type)),
                        })
                    }
                }
            }

            match point_type.node.clone() {
                Expr_::VariableType(t) => {
                    match t {
                        Types::Point |
                        Types::Color |
                        Types::Vector |
                        Types::Normal => Ok(t),

                        _ => return Ok(Types::Void),
                    }
                }
                _ => return Ok(Types::Void),
            }
        },

        Expr_::Assignment(lhs, rhs) => {
            let lhs_type = get_expr_type(lhs, symbols)?;
            let rhs_type = get_expr_type(rhs, symbols)?;

            // Create error
            let error = OSLCompilerError::MismatchedTypesAssignment {
                lhs: Item::new(lhs.span, format!("{:?}", lhs_type)),
                rhs: Item::new(rhs.span, format!("{:?}", rhs_type))
            };

            return match (lhs_type, rhs_type) {
                (Types::Int, Types::Int) => Ok(Types::Int),

                (Types::Float, Types::Float) => Ok(Types::Float),
                (Types::Float, Types::Int) => Ok(Types::Float),

                (Types::Color, Types::Color) => Ok(Types::Color),
                (Types::Color, Types::Float) => Ok(Types::Color),
                (Types::Color, Types::Int) => Ok(Types::Color),

                (Types::Point, Types::Point) => Ok(Types::Point),
                (Types::Point, Types::Float) => Ok(Types::Point),
                (Types::Point, Types::Int) => Ok(Types::Point),

                (Types::Vector, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Types::Float) => Ok(Types::Vector),
                (Types::Vector, Types::Int) => Ok(Types::Vector),

                (Types::Normal, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Types::Float) => Ok(Types::Normal),
                (Types::Normal, Types::Int) => Ok(Types::Normal),

                (Types::Matrix, Types::Matrix) => Ok(Types::Matrix),
                (Types::Matrix, Types::Float) => Ok(Types::Matrix),
                (Types::Matrix, Types::Int) => Ok(Types::Matrix),

                _ => Err(error),
            };
        },

        Expr_::PostUnaryExpression(op, lhs) => {
            let lhs_type = get_expr_type(lhs, symbols)?;

            // Create error
            let error = OSLCompilerError::MismatchedTypesUnary {
                rhs: Item::new(lhs.span, format!("{:?}", lhs_type))
            };

            return match (lhs_type, op) {
                (Types::Int, _) => Ok(Types::Int),

                (Types::Float, _) => Ok(Types::Float),

                _ => Err(error)
            };
        },

        Expr_::PreUnaryExpression(op, rhs) => {
            let rhs_type = get_expr_type(rhs, symbols)?;

            // Create error
            let error = OSLCompilerError::MismatchedTypesUnary {
                rhs: Item::new(rhs.span, format!("{:?}", rhs_type))
            };

            return match (op, rhs_type) {
                (_, Types::Int) => Ok(Types::Int),

                (Operators::Increment, Types::Float) => Ok(Types::Float),
                (Operators::Decrement, Types::Float) => Ok(Types::Float),
                (Operators::Minus, Types::Float) => Ok(Types::Float),
                (_, Types::Float) => Err(error),

                (Operators::Minus, Types::Point) => Ok(Types::Point),
                (Operators::Minus, Types::Color) => Ok(Types::Color),
                (Operators::Minus, Types::Vector) => Ok(Types::Vector),
                (Operators::Minus, Types::Normal) => Ok(Types::Normal),
                (Operators::Minus, Types::Matrix) => Ok(Types::Matrix),

                _ => Err(error)
            };
        },

        Expr_::BinaryExpression(op, lhs, rhs) => {
            let lhs_type = get_expr_type(lhs, symbols)?;
            let rhs_type = get_expr_type(rhs, symbols)?;

            // Create error
            let error = OSLCompilerError::MismatchedTypesBinary {
                lhs: Item::new(lhs.span, format!("{:?}", lhs_type)),
                rhs: Item::new(rhs.span, format!("{:?}", rhs_type))
            };

            return match (lhs_type, op, rhs_type) {
                // Ints can use any operator
                (Types::Int, _, Types::Int) => Ok(Types::Int),

                // Bitwise operators only apply to ints
                (_, Operators::BitwiseAnd, _) => Err(error),
                (_, Operators::BitwiseAndAssign, _) => Err(error),
                (_, Operators::BitwiseOr, _) => Err(error),
                (_, Operators::BitwiseOrAssign, _) => Err(error),
                (_, Operators::BitwiseXor, _) => Err(error),
                (_, Operators::BitwiseXorAssign, _) => Err(error),
                (_, Operators::ShiftLeft, _) => Err(error),
                (_, Operators::ShiftLeftAssign, _) => Err(error),
                (_, Operators::ShiftRight, _) => Err(error),
                (_, Operators::ShiftRightAssign, _) => Err(error),

                // Logical operators only apply to ints
                (_, Operators::LogicalAnd, _) => Err(error),
                (_, Operators::LogicalOr, _) => Err(error),

                // Mod operator only applies ot ints
                (_, Operators::Mod, _) => Err(error),

                // Ints and floats can do math together (int will always be cast to float)
                (Types::Float, Operators::Plus, Types::Int) => Ok(Types::Float),
                (Types::Int, Operators::Plus, Types::Float) => Ok(Types::Float),
                (Types::Float, Operators::AddAssign, Types::Int) => Ok(Types::Float),
                (Types::Float, Operators::Minus, Types::Int) => Ok(Types::Float),
                (Types::Int, Operators::Minus, Types::Float) => Ok(Types::Float),
                (Types::Float, Operators::SubtractAssign, Types::Int) => Ok(Types::Float),
                (Types::Float, Operators::Multiply, Types::Int) => Ok(Types::Float),
                (Types::Int, Operators::Multiply, Types::Float) => Ok(Types::Float),
                (Types::Float, Operators::MultiplyAssign, Types::Int) => Ok(Types::Float),
                (Types::Float, Operators::Divide, Types::Int) => Ok(Types::Float),
                (Types::Int, Operators::Divide, Types::Float) => Ok(Types::Float),
                (Types::Float, Operators::DivideAssign, Types::Int) => Ok(Types::Float),

                // Ints and floats can be less than / greater than each other and return int
                (Types::Float, Operators::LessThan, Types::Int) => Ok(Types::Int),
                (Types::Int, Operators::LessThan, Types::Float) => Ok(Types::Int),
                (Types::Float, Operators::LessThan, Types::Float) => Ok(Types::Int),
                (Types::Float, Operators::LessThanEqual, Types::Int) => Ok(Types::Int),
                (Types::Int, Operators::LessThanEqual, Types::Float) => Ok(Types::Int),
                (Types::Float, Operators::LessThanEqual, Types::Float) => Ok(Types::Int),
                (Types::Float, Operators::GreaterThan, Types::Int) => Ok(Types::Int),
                (Types::Int, Operators::GreaterThan, Types::Float) => Ok(Types::Int),
                (Types::Float, Operators::GreaterThan, Types::Float) => Ok(Types::Int),
                (Types::Float, Operators::GreaterThanEqual, Types::Int) => Ok(Types::Int),
                (Types::Int, Operators::GreaterThanEqual, Types::Float) => Ok(Types::Int),
                (Types::Float, Operators::GreaterThanEqual, Types::Float) => Ok(Types::Int),

                // No others can
                (_, Operators::LessThan, _) => Err(error),
                (_, Operators::LessThanEqual, _) => Err(error),
                (_, Operators::GreaterThan, _) => Err(error),
                (_, Operators::GreaterThanEqual, _) => Err(error),

                // All types can be equal/not equal with its own type and returns int
                (Types::Float, Operators::Equals, Types::Float) => Ok(Types::Int),
                (Types::Float, Operators::NotEqual, Types::Float) => Ok(Types::Int),
                (Types::Color, Operators::Equals, Types::Color) => Ok(Types::Int),
                (Types::Color, Operators::NotEqual, Types::Color) => Ok(Types::Int),
                (Types::Point, Operators::Equals, Types::Point) => Ok(Types::Int),
                (Types::Point, Operators::NotEqual, Types::Point) => Ok(Types::Int),
                (Types::Vector, Operators::Equals, Types::Vector) => Ok(Types::Int),
                (Types::Vector, Operators::NotEqual, Types::Vector) => Ok(Types::Int),
                (Types::Normal, Operators::Equals, Types::Normal) => Ok(Types::Int),
                (Types::Normal, Operators::NotEqual, Types::Normal) => Ok(Types::Int),
                (Types::Matrix, Operators::Equals, Types::Matrix) => Ok(Types::Int),
                (Types::Matrix, Operators::NotEqual, Types::Matrix) => Ok(Types::Int),
                (Types::String, Operators::Equals, Types::String) => Ok(Types::Int),
                (Types::String, Operators::NotEqual, Types::String) => Ok(Types::Int),

                // Ints and floats can be equal/not equal to each other
                (Types::Float, Operators::Equals, Types::Int) => Ok(Types::Int),
                (Types::Float, Operators::NotEqual, Types::Int) => Ok(Types::Int),
                (Types::Int, Operators::Equals, Types::Float) => Ok(Types::Int),
                (Types::Int, Operators::NotEqual, Types::Float) => Ok(Types::Int),

                // Other types cannot be equal/not equal with different types
                (_, Operators::Equals, _) => Err(error),
                (_, Operators::NotEqual, _) => Err(error),

                // Strings may not use any other operator
                (Types::String, _, _) => Err(error),
                (_, _, Types::String) => Err(error),


                // Floats can use all remaining operators and returns float
                (Types::Float, _, Types::Float) => Ok(Types::Float),

                // Color arithmetic operations
                (Types::Color, Operators::Multiply, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::Multiply, Types::Float) => Ok(Types::Color),
                (Types::Float, Operators::Multiply, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::MultiplyAssign, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::MultiplyAssign, Types::Float) => Ok(Types::Color),
                (Types::Color, Operators::Divide, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::Divide, Types::Float) => Ok(Types::Color),
                (Types::Float, Operators::Divide, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::DivideAssign, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::DivideAssign, Types::Float) => Ok(Types::Color),
                (Types::Color, Operators::Plus, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::AddAssign, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::Minus, Types::Color) => Ok(Types::Color),
                (Types::Color, Operators::SubtractAssign, Types::Color) => Ok(Types::Color),

                // Points arithmetic operations
                (Types::Point, Operators::Multiply, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::Multiply, Types::Float) => Ok(Types::Point),
                (Types::Float, Operators::Multiply, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::MultiplyAssign, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::MultiplyAssign, Types::Float) => Ok(Types::Point),
                (Types::Point, Operators::Divide, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::Divide, Types::Float) => Ok(Types::Point),
                (Types::Float, Operators::Divide, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::DivideAssign, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::DivideAssign, Types::Float) => Ok(Types::Point),
                (Types::Point, Operators::Plus, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::AddAssign, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::Minus, Types::Point) => Ok(Types::Point),
                (Types::Point, Operators::SubtractAssign, Types::Point) => Ok(Types::Point),

                // Vectors arithmetic operations
                (Types::Vector, Operators::Multiply, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::Multiply, Types::Float) => Ok(Types::Vector),
                (Types::Float, Operators::Multiply, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::MultiplyAssign, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::MultiplyAssign, Types::Float) => Ok(Types::Vector),
                (Types::Vector, Operators::Divide, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::Divide, Types::Float) => Ok(Types::Vector),
                (Types::Float, Operators::Divide, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::DivideAssign, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::DivideAssign, Types::Float) => Ok(Types::Vector),
                (Types::Vector, Operators::Plus, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::AddAssign, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::Minus, Types::Vector) => Ok(Types::Vector),
                (Types::Vector, Operators::SubtractAssign, Types::Vector) => Ok(Types::Vector),

                // Normals arithmetic operations
                (Types::Normal, Operators::Multiply, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::Multiply, Types::Float) => Ok(Types::Normal),
                (Types::Float, Operators::Multiply, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::MultiplyAssign, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::MultiplyAssign, Types::Float) => Ok(Types::Normal),
                (Types::Normal, Operators::Divide, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::Divide, Types::Float) => Ok(Types::Normal),
                (Types::Float, Operators::Divide, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::DivideAssign, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::DivideAssign, Types::Float) => Ok(Types::Normal),
                (Types::Normal, Operators::Plus, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::AddAssign, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::Minus, Types::Normal) => Ok(Types::Normal),
                (Types::Normal, Operators::SubtractAssign, Types::Normal) => Ok(Types::Normal),

                // Matrices arithmetic operations
                (Types::Matrix, Operators::Multiply, Types::Matrix) => Ok(Types::Matrix),
                (Types::Matrix, Operators::Multiply, Types::Float) => Ok(Types::Matrix),
                (Types::Float, Operators::Multiply, Types::Matrix) => Ok(Types::Matrix),
                (Types::Matrix, Operators::MultiplyAssign, Types::Matrix) => Ok(Types::Matrix),
                (Types::Matrix, Operators::MultiplyAssign, Types::Float) => Ok(Types::Matrix),
                (Types::Matrix, Operators::Divide, Types::Matrix) => Ok(Types::Matrix),
                (Types::Matrix, Operators::Divide, Types::Float) => Ok(Types::Matrix),
                (Types::Float, Operators::Divide, Types::Matrix) => Ok(Types::Matrix),
                (Types::Matrix, Operators::DivideAssign, Types::Matrix) => Ok(Types::Matrix),
                (Types::Matrix, Operators::DivideAssign, Types::Float) => Ok(Types::Matrix),

                // Everything else
                _ => Err(error)
            };

        },

        Expr_::GlobalVariable(g) => {
            return match g {
                Globals::P => Ok(Types::Point),
                Globals::I => Ok(Types::Vector),
                Globals::N => Ok(Types::Normal),
                Globals::Ng => Ok(Types::Normal),
                Globals::UV => Ok(Types::Float),
                Globals::Dpdu => Ok(Types::Vector),
                Globals::Dpdv => Ok(Types::Vector),
                Globals::Ps => Ok(Types::Point),
                Globals::Time => Ok(Types::Float),
                Globals::Dtime => Ok(Types::Float),
                Globals::Dpdtime => Ok(Types::Vector),

                _ => Ok(Types::Void)
            }
        },

        Expr_::Ident(s) => {
            let symbol = symbols.get_reference(expr.span, s.clone());
            return match symbol {
                Symbols::Variable{var_type, ..} => Ok(var_type),
                _ => Ok(Types::Void),
            }
        },

        Expr_::VariableType(t) => return Ok(t.clone()),
        Expr_::IntLiteral(..) => return Ok(Types::Int),
        Expr_::FloatLiteral(..) => return Ok(Types::Float),
        Expr_::StringLiteral(..) => return Ok(Types::String),

        Expr_::EmptyExpression => return Ok(Types::Void),

        _ => return Ok(Types::Void),
    }
}
