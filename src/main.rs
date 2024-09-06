use std::fs;

use clap::Parser as ClapParser;

use args::{Args, Emit};
use chumsky::Parser;
use parser::parser;

mod args;
mod parser;

fn main() {
    let args = Args::parse();
    let input_content = fs::read_to_string(&args.input_file)
        .unwrap_or_else(|_| panic!("failed to read {:#?}", &args.input_file));

    let ast = parser().parse(input_content).unwrap();

    if matches!(args.emit, Emit::Ast) {
        println!("Ast: {:#?}", ast)
    }
}
