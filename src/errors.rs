use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::{self, ColorArg};
use std::ops::Range;

use crate::Span;

#[derive(Debug, Clone)]
pub enum OSLCompilerError {

    OutOfScopeVariable {var: Item},

    ExistingVariable {existing: Item, new: Item},

    LexerError {message: String, error: Item},
}

impl OSLCompilerError {
    pub fn report(&self) -> Diagnostic<()> {
        match self {

            OSLCompilerError::OutOfScopeVariable{var} => Diagnostic::error()
                //.with_code("E0308")
                .with_message("Variable out of scope")
                .with_labels(vec![
                    Label::primary((), var.range.clone()).with_message(format!(
                        "Variable {} out of scope", var.content)),
                    ]),
            
            OSLCompilerError::ExistingVariable{existing, new} => Diagnostic::error()
                //.with_code("E0384")
                .with_message("Cannot declare variable twice in same scope")
                .with_labels(vec![
                    Label::secondary((), existing.range.clone()).with_message(
                        &format!(
                            "Original declaration for {}",
                            existing.content,
                        ),
                    ),
                    Label::primary((), new.range.clone())
                        .with_message(format!(
                            "New declaration for {}",
                            new.content,
                        )),
                ]),


            OSLCompilerError::LexerError{message, error} => Diagnostic::error()
                //.with_code("E0308")
                .with_message(message)
                .with_labels(vec![
                    Label::primary((), error.range.clone()),
                ]),
        }
    }
}

/// An item in the source code to be used in the `Error` enum.
/// In a more complex program it could also contain a `files::FileId` to handle errors that occur inside multiple files.
#[derive(Debug, Clone)]
pub struct Item {
    range: Range<usize>,
    content: String,
}

impl Item {
    pub fn new(span: Span, content: impl Into<String>) -> Item {
        let range = span.lo..span.hi;
        let content = content.into();
        Item { range, content }
    }
}