use std::fs;

use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::{StandardStream, ColorChoice};
use codespan_reporting::term;

use clap::Parser;

use osl::compiler::{compile, Backend};
use osl::cli::*;

fn main() -> Result<(), String> {

    let args = CliArgs::parse();

    let contents = fs::read_to_string(args.input_file).expect("Invalid file");
    println!("{}", contents.len());

    let file = SimpleFile::new("test.osl", contents.clone());

    match compile(contents.clone(), Backend::LLVM) {
        Err(e) => {
            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config{
                start_context_lines: 3,
                end_context_lines: 3,
                ..Default::default()
            };
            term::emit(&mut writer.lock(), &config, &file, &e.report())
                .map_err(|e| e.to_string())?;
        }
        _ => {}
    }

    Ok(())
}

