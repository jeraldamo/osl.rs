mod lexer;
mod ast;
mod parser;
pub mod symtab;
mod spirv;


use lexer::Lexer;
use parser::parse;
use symtab::SymbolTable;
use ast::Stmt;
use super::errors::*;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
    pub line: usize,
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Keywords {
    And,
    Break,
    Closure,
    Color,
    Continue,
    Displacement,
    Do,
    Else,
    Emit,
    Float,
    For,
    If,
    Illuminance,
    Illuminate,
    Int,
    Light,
    Matrix,
    Normal,
    Not,
    Or,
    Output,
    Point,
    Public,
    Return,
    Shader,
    String,
    Struct,
    Surface,
    Vector,
    Void,
    Volume,
    While,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ReservedKeywords {
    Bool,
    Case,
    Catch,
    Char,
    Class,
    Const,
    Delete,
    Default,
    Double,
    Enum,
    Extern,
    False,
    Friend,
    Goto,
    Inline,
    Long,
    New,
    Operator,
    Private,
    Protected,
    Short,
    Signed,
    Sizeof,
    Static,
    Switch,
    Template,
    This,
    Throw,
    True,
    Try,
    Typedef,
    Uniform,
    Union,
    Unsigned,
    Varying,
    Virtual,
    Volatile,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Operators {
    Assign,           // =
    Equals,           // ==
    Plus,             // +
    Minus,            // -
    Multiply,         // *
    Divide,           // /
    Mod,              // %
    BitwiseAnd,       // &
    BitwiseOr,        // |
    BitwiseXor,       // ^
    BitwiseCompliment,// ~
    ShiftLeft,        // <<
    ShiftRight,       // >>
    AddAssign,        // +=
    SubtractAssign,   // -=
    MultiplyAssign,   // *=
    DivideAssign,     // /=
    BitwiseAndAssign, // &=
    BitwiseOrAssign,  // |=
    BitwiseXorAssign, // ^=
    ShiftLeftAssign,  // <<=
    ShiftRightAssign, // >>=
    LogicalAnd,       // &&
    LogicalOr,        // ||
    Not,              // !
    LessThan,         // <
    GreaterThan,      // >
    LessThanEqual,    // <=
    GreaterThanEqual, // >=
    NotEqual,         // !=
    Increment,        // ++
    Decrement,        // --
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ShaderTypes {
    Surface,
    Displacement,
    Volume,
    Light,
    Shader,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ColorSpaces {
    RGB,
    HSV,
    HSL,
    YIQ,
    XYZ,
    XYY,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GeometricSpaces {
    Common,
    Object,
    Shader,
    World,
    Camera,
    Screen,
    Raster,
    NDC,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Types {
    Int,
    Float,
    String,
    Color,
    Point,
    Vector,
    Normal,
    Matrix,
    Void,
    Closure(Box<Types>),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Globals {
    P,
    I,
    N,
    Ng,
    Dpdu,
    Dpdv,
    Ps,
    UV,
    Time,
    Dtime,
    Dpdtime,
    Ci,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Token {
    KWAnd,
    KWBreak,
    KWClosure,
    KWColor,
    KWContinue,
    KWDisplacement,
    KWDo,
    KWElse,
    KWEmit,
    KWFloat,
    KWFor,
    KWIf,
    KWIlluminance,
    KWIlluminate,
    KWInt,
    KWLight,
    KWMatrix,
    KWNormal,
    KWNot,
    KWOr,
    KWOutput,
    KWPoint,
    KWPublic,
    KWReturn,
    KWShader,
    KWString,
    KWStruct,
    KWSurface,
    KWVector,
    KWVoid,
    KWVolume,
    KWWhile,

    ElseIf,

    ReservedKeyword(ReservedKeywords),
    Error{
        message: String,
        content: String,
    },

    Ident(String),
    Global(Globals),
    Type(Types),
    Shader(ShaderTypes),

    Integer(i64),
    HexInteger(i64),
    Float(f64),
    Str(String),

    OPAssign,           // =
    OPEquals,           // ==
    OPPlus,             // +
    OPMinus,            // -
    OPMultiply,         // *
    OPDivide,           // /
    OPMod,              // %
    OPBitwiseAnd,       // &
    OPBitwiseOr,        // |
    OPBitwiseXor,       // ^
    OPBitwiseCompliment,// ~
    OPShiftLeft,        // <<
    OPShiftRight,       // >>
    OPAddAssign,        // +=
    OPSubtractAssign,   // -=
    OPMultiplyAssign,   // *=
    OPDivideAssign,     // /=
    OPBitwiseAndAssign, // &=
    OPBitwiseOrAssign,  // |=
    OPBitwiseXorAssign, // ^=
    OPShiftLeftAssign,  // <<=
    OPShiftRightAssign, // >>=
    OPLogicalAnd,       // &&
    OPLogicalOr,        // ||
    OPNot,              // !
    OPLessThan,         // <
    OPGreaterThan,      // >
    OPLessThanEqual,    // <=
    OPGreaterThanEqual, // >=
    OPNotEqual,         // !=
    OPIncrement,        // ++
    OPDecrement,        // --

    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftSquare,
    RightSquare,
    Semicolon,
    Colon,
    Period,
    Comma,

    Whitespace,
    Newline,
    Comment,
    Meta(String),
}

#[derive(Debug)]
pub enum Backend {
    LLVM,
    SPIRV,
    OSO,
}


pub fn compile(contents: String, backend: Backend) -> Result<Vec<u8>, OSLCompilerError> {


    println!("Lexing tokens...");
    let tokens = Lexer::new(contents.as_str());

    for tok in tokens.clone() {
        match tok.0 {
            Token::Error{message, content} => {
                return Err(OSLCompilerError::LexerError {
                    message,
                    error: Item::new(tok.1, content),
                });
            },
            _ => {},
        }
    }

    for tok in tokens.clone() {
        println!{"{:?}", tok};
    }

    println!("Parsing AST...");
    let statements = match parser::parse(tokens.clone()) {
        Err(error) => {
            let (_token, span) = error.0.unwrap();
            return Err(OSLCompilerError::ParserError {
                error: Item::new(span, error.1)
            });
        }
        Ok(stmts) => stmts
    };

    println!("{:#?}", statements);


    println!("Building symbol table...");
    let mut symbol_table = SymbolTable::new(contents.len())?;
    symbol_table.build_symbols(&statements)?;

    println!("Checking symantics...");
    check_semantics(&symbol_table, &tokens, &statements)?;

    // let mut comp = compiler::Compiler::new(tokens.clone().collect(), &statements, contents.len())?;
    // comp.compile()?;

    Ok(vec![])
}

fn check_semantics(symbol_table: &SymbolTable, tokens: &Lexer, program: &Vec<Stmt>) -> Result<(), OSLCompilerError> {
    // Make sure that the program has one and only one shader function
    if symbol_table.n_shaders == 0 {
        return Err(OSLCompilerError::MissingShader);
    }
    else if symbol_table.n_shaders > 1 {
        return Err(OSLCompilerError::MultipleShaders);
    }

    // Cycle through tokens and make sure that any ident token references a symbol in the
    // symbol table and has access to it.
    for (token, span) in tokens.clone() {
        match token {
            Token::Ident(s) => {
                symbol_table.check_access(span, s.clone())?;
            },
            _ => {}
        }
    }

    symbol_table.check_types(program)?;

    Ok(())
}
