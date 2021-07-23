pub mod lexer;
pub mod ast;
pub mod parser;
pub mod compiler;
pub mod errors;
pub mod symtab;


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
#[derive(Debug, Clone)]
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
}
