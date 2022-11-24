use plex::lexer;

use crate::compiler::{Token, Globals, Span, Types, ShaderTypes};
// use crate::errors::{OSLCompilerError, Item};


lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \t\r]+"# => Token::Whitespace,
    r#"\n"# => Token::Newline,
    // "C-style" comments (/* .. */) - can't contain "*/"
    r#"/[*](~(.*[*]/.*))[*]/"# => Token::Comment,
    // "C++-style" comments (// ...)
    r#"//[^\n]*"# => Token::Comment,
    // Meta tags - can't contain "]]"
    r#"\[\[(~(.*\]\].*))\]\]"# => Token::Meta(text.to_owned()),
    // Grab string literals
    r#"\"(\\.|[^\\"\n])*\""# => Token::Str(text.to_owned()),
    // Int literals
    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Token::Integer(i)
        } else {
            panic!("integer {} is out of range", text)
        }
    }
    r#"0[xX][0-9a-fA-F]+"# => {
        if let Ok(i) = text.parse() {
            Token::HexInteger(i)
        } else {
            panic!("integer {} is out of range", text)
        }
    }
    // Match floats, including scientific notation
    // r#"([0-9]+[eE][-+]?[0-9]+|\
    // [0-9]+\.[0-9]*([eE][-+]?[0-9]+)?|\
    // [0-9]*\.[0-9]+([eE][-+]?[0-9]+)?|\
    // [0-9]*\.[0-9]+)"# => {
    r#"[0-9]*\.[0-9]+"# => {
        if let Ok(f) = text.parse() {
            Token::Float(f)
        } else {
            panic!("float {} is out of range", text)
        }
    }

    r#"else if"# => Token::ElseIf,

    // Keywords
    r#"and"# => Token::KWAnd,
    r#"break"# => Token::KWBreak,
    r#"closure"# => Token::KWClosure,
    r#"color"# => Token::Type(Types::Color),
    r#"continue"# => Token::KWContinue,
    r#"displacement"# => Token::Shader(ShaderTypes::Displacement),
    r#"do"# => Token::KWDo,
    r#"else"# => Token::KWElse,
    r#"emit"# => Token::KWEmit,
    r#"float"# => Token::Type(Types::Float),
    r#"for"# => Token::KWFor,
    r#"if"# => Token::KWIf,
    r#"illuminance"# => Token::KWIlluminance,
    r#"illuminate"# => Token::KWIlluminate,
    r#"int"# => Token::Type(Types::Int),
    r#"light"# => Token::Shader(ShaderTypes::Light),
    r#"matrix"# => Token::Type(Types::Matrix),
    r#"normal"# => Token::Type(Types::Normal),
    r#"not"# => Token::KWNot,
    r#"or"# => Token::KWOr,
    r#"output"# => Token::KWOutput,
    r#"point"# => Token::Type(Types::Point),
    r#"public"# => Token::KWPublic,
    r#"return"# => Token::KWReturn,
    r#"shader"# => Token::Shader(ShaderTypes::Shader),
    r#"string"# => Token::Type(Types::String),
    r#"struct"# => Token::KWStruct,
    r#"surface"# => Token::Shader(ShaderTypes::Surface),
    r#"vector"# => Token::Type(Types::Vector),
    r#"void"# => Token::Type(Types::Void),
    r#"volume"# => Token::Shader(ShaderTypes::Volume),
    r#"while"# => Token::KWWhile,


    // Reserved keywords
    // Currently we just panic, will add error handling...
    r#"(bool|case|catch|char|class|const|delete|default|double|enum|\
    extern|false|friend|goto|inline|long|new|operator|private|protected|\
    short|signed|sizeof|static|switch|template|this|throw|true|try|typedefl|\
    uniform|union|unsigned|varying|virtual|volatile)"# => Token::Error{
        message: String::from("Use of reserved keyword"),
        content: String::new(),
    },

    // Punctuation
    r#"\("# => Token::LeftParen,
    r#"\)"# => Token::RightParen,
    r#"\{"# => Token::LeftCurly,
    r#"\}"# => Token::RightCurly,
    r#"\["# => Token::LeftSquare,
    r#"\]"# => Token::RightSquare,
    r#";"# => Token::Semicolon,
    r#":"# => Token::Colon,
    r#"\."# => Token::Period,
    r#","# => Token::Comma,

    // Global variables
    r#"P"# => Token::Global(Globals::P),
    r#"I"# => Token::Global(Globals::I),
    r#"N"# => Token::Global(Globals::N),
    r#"Ng"# => Token::Global(Globals::Ng),
    r#"dPdU"# => Token::Global(Globals::Dpdu),
    r#"dPdV"# => Token::Global(Globals::Dpdv),
    r#"Ps"# => Token::Global(Globals::Ps),
    r#"I"# => Token::Global(Globals::I),
    r#"u[ \t\r]*,[ \t\r]*v"# => Token::Global(Globals::UV),
    r#"time"# => Token::Global(Globals::Time),
    r#"dtime"# => Token::Global(Globals::Dtime),
    r#"dPdtime"# => Token::Global(Globals::Dpdtime),
    r#"Ci"# => Token::Global(Globals::Ci),

    // Identifiers
    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => Token::Ident(text.to_owned()),

    // Operators
    r#"="# => Token::OPAssign,
    r#"=="# => Token::OPEquals,
    r#"\+"# => Token::OPPlus,
    r#"-"# => Token::OPMinus,
    r#"\*"# => Token::OPMultiply,
    r#"/"# => Token::OPDivide,
    r#"%"# => Token::OPMod,
    r#"\&"# => Token::OPBitwiseAnd,
    r#"\|"# => Token::OPBitwiseOr,
    r#"\^"# => Token::OPBitwiseXor,
    r#"\~"# => Token::OPBitwiseCompliment,
    r#"<<"# => Token::OPShiftLeft,
    r#">>"# => Token::OPShiftRight,
    r#"\+="# => Token::OPAddAssign,
    r#"-="# => Token::OPSubtractAssign,
    r#"\*="# => Token::OPMultiplyAssign,
    r#"/="# => Token::OPDivideAssign,
    r#"\&="# => Token::OPBitwiseAndAssign,
    r#"\|="# => Token::OPBitwiseOrAssign,
    r#"\^="# => Token::OPBitwiseXorAssign,
    r#"<<="# => Token::OPShiftLeftAssign,
    r#">>="# => Token::OPShiftRightAssign,
    r#"\&\&"# => Token::OPLogicalAnd,
    r#"\|\|"# => Token::OPLogicalOr,
    r#"!"# => Token::OPNot,
    r#"<"# => Token::OPLessThan,
    r#">"# => Token::OPGreaterThan,
    r#"<="# => Token::OPLessThanEqual,
    r#">="# => Token::OPGreaterThanEqual,
    r#"!="# => Token::OPNotEqual,
    r#"\+\+"# => Token::OPIncrement,
    r#"--"# => Token::OPDecrement,

    r#"."# => Token::Error{
        message: String::from("Unexpected character"),
        content: String::new(),
    },
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
    cur_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
            cur_line: 1,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi, line: self.cur_line })
            } else {
                self.cur_line = 0;
                return None;
            };
            match tok {
                Token::Whitespace => {
                    continue;
                }
                Token::Comment => {
                    self.cur_line += self.original[span.lo..span.hi].matches('\n').count();
                    continue;
                }
                Token::Newline => {
                    self.cur_line += 1;
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
