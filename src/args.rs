use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// TODO programming language
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Input file
    #[arg(short, long)]
    pub input_file: PathBuf,

    /// Type of output to emit
    #[arg(short, long)]
    #[arg(value_enum, default_value_t=Emit::default())]
    pub emit: Emit,
}

#[derive(Default, ValueEnum, Clone, Debug)]
pub enum Emit {
    #[default]
    Ast,
}
