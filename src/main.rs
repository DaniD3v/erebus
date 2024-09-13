use std::{fs, path::Path};

use ariadne::{sources, Color, Label, Report, ReportKind};
use clap::Parser as ClapParser;

use args::{Args, Emit};
use erebus_parser::{Ast, Parsable};

mod args;

fn failed_compiling(input_file: &Path, error_amount: usize) -> ! {
    assert!(error_amount > 0);

    panic!(
        "failed compiling \"{}\" due to {}.",
        input_file.display(),
        match error_amount {
            1 => "1 error".to_owned(),
            x => format!("{x} errors."),
        },
    )
}

fn main() {
    let args = Args::parse();
    let input_content = fs::read_to_string(&args.input_file)
        .unwrap_or_else(|_| panic!("failed to read {:#?}", &args.input_file));

    let ast = match Ast::parse(&input_content).into_result() {
        Ok(ast) => ast,
        Err(errors) => {
            let filename = args.input_file.display().to_string();

            for err in &errors {
                Report::build(ReportKind::Error, filename.clone(), err.span().start)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new((filename.clone(), err.span().into_range()))
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print(sources([(filename.clone(), input_content.clone())]))
                    .unwrap()
            }

            failed_compiling(&args.input_file, errors.len())
        }
    };

    if matches!(args.emit, Emit::Ast) {
        println!("Ast: {:#?}", ast)
    }
}
