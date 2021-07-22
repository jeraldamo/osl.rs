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

    let file = SimpleFile::new("test.osl", contents.clone());

    match compile(contents.clone()) {
        Err(e) => {
            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();
            term::emit(&mut writer.lock(), &config, &file, &e.report());
        }
        _ => {}
    }
}

fn compile(contents: String) -> Result<(), OSLCompilerError> {



    let tokens= lexer::Lexer::new(contents.as_str());

    let mut encountered_lexing_error = false;

    for tok in tokens.clone() {
        match tok.0 {
            Token::Error(msg) => {
                eprintln!("Syntax Error (line {}): {}", tok.1.line, msg);
                encountered_lexing_error = true;
            },
            _ => {},
        }
    }

    if !encountered_lexing_error {
        for tok in tokens.clone() {
            println!{"{:?}", tok};
        }

        let statements = parser::parse(tokens.clone()).expect("Error Parsing");

        println!("{:#?}", statements);

        let mut comp = compiler::Compiler::new(&statements);
        comp.compile()?;
    }

    Ok(())
}
