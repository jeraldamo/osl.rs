use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
   pub input_file: String,

   /// Number of times to greet
   #[clap(short, long, value_parser, default_value_t = 1)]
   count: u8,
}
