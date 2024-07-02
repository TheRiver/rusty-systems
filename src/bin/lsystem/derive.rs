use std::process::ExitCode;

use rusty_systems::prelude::*;
use rusty_systems::interpretation::abop::parser;
use rusty_systems::interpretation::svg::SvgPathInterpretation;

use crate::{error_style, green};
use crate::cli::{Cli, InterpretationArgs};

pub fn handle_derive(args: &Cli, derive: &InterpretationArgs) -> ExitCode {
    if args.verbose {
        print!("Reading {} ", derive.file.to_str().unwrap());
    }

    let (interpretation, system, axiom) = {
        let result = parser::parse_file(derive.file.as_ref());
        if let Err(e) = result {
            if args.verbose { println!("❌") }
            eprint!("\n{}: ", error_style().paint("Error"));
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }

        result.unwrap()
    };

    if args.verbose {
        println!("{}", green().paint("\t✔"));
    }

    let interpretation = SvgPathInterpretation::new_with(derive.width, derive.height, interpretation);

    let result = system.derive(axiom, interpretation.run_settings());
    if let Err(e) = result {
        eprintln!("\n{}", error_style().paint("Error"));
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }
    let result = interpretation.interpret(&system, &result.unwrap());

    if let Err(e) = result {
        eprintln!("\n{}", error_style().paint("Error"));
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }
    let result = result.unwrap();

    let mut output = derive.output.to_path_buf();
    let extension = output.extension().unwrap_or_default().to_ascii_lowercase();
    if extension != "svg" {
        output = output.with_extension("svg");
    }

    if let Some(e) = result.save_file(output.as_path()).err() {
        eprintln!("\n{}", error_style().paint("Error"));
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    if args.verbose {
        println!("Saved SVG file to {} {}", output.to_string_lossy(), green().paint("\t✔"));
    }

    ExitCode::SUCCESS
}
