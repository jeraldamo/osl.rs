use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::{StandardStream, ColorChoice};
use codespan_reporting::term;

use osl::*;
use osl::lexer;
use osl::parser;
use osl::compiler;
use osl::errors::*;

fn main() {

    let shader_file = "test.osl";
    let file_path = current_dir()
        .unwrap()
        .join(PathBuf::from("shaders")
            .join(shader_file.clone())
        );
    let contents = fs::read_to_string(file_path).expect("Invalid file");
    println!("{}", contents.len());

    let file = SimpleFile::new("test.osl", contents.clone());

    match compile(contents.clone()) {
        Err(e) => {
            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config{
                start_context_lines: 3,
                end_context_lines: 3,
                ..Default::default()
            };
            term::emit(&mut writer.lock(), &config, &file, &e.report());
        }
        _ => {}
    }
}

fn compile(contents: String) -> Result<(), OSLCompilerError> {

    let tokens= lexer::Lexer::new(contents.as_str());

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

    let statements = parser::parse(tokens.clone()).expect("Error Parsing");

    println!("{:#?}", statements);

    let mut comp = compiler::Compiler::new(&statements, contents.len());
    comp.compile()?;

    Ok(())
}
