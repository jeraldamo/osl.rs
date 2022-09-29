use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::{self, ColorArg};
use std::ops::Range;

use crate::compiler::Span;

#[derive(Debug, Clone)]
pub enum OSLCompilerError {

    MismatchedTypesAssignment {lhs: Item, rhs: Item},

    MismatchedTypesBinary {lhs: Item, rhs: Item},

    MismatchedTypesUnary {rhs: Item},

    MismatchedTypesArgument {expected: Item, received: Item},

    InvalidCondition {expr: Item},

    NonExistentIdent {ident: Item},

    OutOfScopeIdent {origin: Item, options: Vec<Item>},

    ExistingVariable {existing: Item, new: Item},

    LexerError {message: String, error: Item},

    GlobalScopeVariable {var: Item},

    GlobalScopeBlock {block: Item},

    ParserError {error: Item},

    MissingShader,

    MultipleShaders,

    GenericError (Item),
}

impl OSLCompilerError {
    pub fn report(&self) -> Diagnostic<()> {
        match self {
            OSLCompilerError::MismatchedTypesAssignment {lhs, rhs} => Diagnostic::error()
                .with_message(format!("The type {} cannot be implicitly cast to type {}.", rhs.content.clone(), lhs.content.clone()))
                .with_labels(vec![
                    Label::secondary((), lhs.range.clone())
                        .with_message(format!("Type {}", lhs.content.clone())),
                    Label::primary((), rhs.range.clone())
                        .with_message(format!("Type {}", rhs.content.clone())),
                ]),

            OSLCompilerError::MismatchedTypesBinary {lhs, rhs} => Diagnostic::error()
                .with_message("This operation is invalid due to mismatched types.")
                .with_labels(vec![
                   Label::secondary((), lhs.range.clone())
                       .with_message(format!("Type {}", lhs.content.clone())),
                   Label::secondary((), rhs.range.clone())
                       .with_message(format!("Type {}", rhs.content.clone())),
                ]),

            OSLCompilerError::MismatchedTypesUnary {rhs} => Diagnostic::error()
                .with_message("This operation is invalid due to an unsupported type.")
                .with_labels(vec![
                    Label::secondary((), rhs.range.clone())
                        .with_message(format!("Type {}", rhs.content.clone())),
                ]),

            OSLCompilerError::MismatchedTypesArgument {expected, received} => Diagnostic::error()
                .with_message("A function argument did not have the correct type.")
                .with_labels(vec![
                    Label::primary((), received.range.clone())
                        .with_message(format!("Expected type {}, received type {}",
                            expected.content.clone(),
                            received.content.clone())),
                ]),

            OSLCompilerError::InvalidCondition {expr} => Diagnostic::error()
                .with_message("Conditional expressions must evaluate to type Int.")
                .with_labels(vec![
                    Label::primary((), expr.range.clone())
                        .with_message(format!("Expression of type {}", expr.content)),
                ]),

            OSLCompilerError::NonExistentIdent {ident} => Diagnostic::error()
                .with_message("Reference to non-existent symbol")
                .with_labels(vec![
                    Label::primary((), ident.range.clone())
                        .with_message(format!("Symbol {} does not exist", ident.content)),
                ]),

            OSLCompilerError::OutOfScopeIdent{origin, options} => {
                let mut labels: Vec<Label<_>> = vec![Label::primary((), origin.range.clone())
                                                         .with_message("Referenced here"),];
                for option in options {
                    labels.push(Label::secondary((), option.range.clone())
                        .with_message("Declared here"));
                }

                Diagnostic::error()
                    //.with_code("E0308")
                    .with_message("Reference to an out of scope symbol")
                    .with_labels(labels)
            },

            OSLCompilerError::GlobalScopeVariable{var} => Diagnostic::error()
                //.with_code("E0308")
                .with_message("Variable cannot be created in the global scope")
                .with_labels(vec![
                    Label::primary((), var.range.clone()).with_message(format!(
                        "Variable {} is in the global scope", var.content)),
                ]),

            OSLCompilerError::GlobalScopeBlock{block} => Diagnostic::error()
                //.with_code("E0308")
                .with_message("Block cannot be created in the global scope")
                .with_labels(vec![
                    Label::primary((), block.range.clone()).with_message("Block is in the global scope"),
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

            OSLCompilerError::ParserError {error } => Diagnostic::error()
                .with_message("Error parsing OSL file")
                .with_labels(vec![
                    Label::primary((), error.range.clone())
                        .with_message(error.content.clone())
                ]),

            OSLCompilerError::MissingShader => Diagnostic::error()
                .with_message("Missing shader function")
                .with_notes(vec![String::from("At least one shader function is required per OSL file.")]),

            OSLCompilerError::MultipleShaders => Diagnostic::error()
                .with_message("Multiple shader functions")
                .with_notes(vec![String::from("At most one shader function is allowed per OSL file.")]),

            OSLCompilerError::GenericError(error) => Diagnostic::error()
                .with_message("This is a temporary generic error...")
                .with_labels(vec![
                    Label::primary((), error.range.clone())
                        .with_message(error.content.clone())
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
