use std::process::ExitCode;
use ansi_term::Color;
use clap::{Args, Parser, Subcommand};
use rusty_systems::prelude::Interpretation;
use rusty_systems::system::interpretation::abop::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command
}

#[derive(Debug, Subcommand)]
enum Command {
    Derive {
        file: Box<std::path::Path>, // todo Figure out how to document
        #[arg(short, long)]
        output: Box<std::path::Path>
    }
}

// todo Need to set up features to include libraries only for this.
fn main() -> ExitCode {
    let args = Cli::parse();

    let head_style = Color::White.bold().underline();
    let green = Color::Green;
    let error_style = Color::Red.underline();

    match args.command {
        Command::Derive { file, output } => {
            if args.verbose {
                print!("Reading {} ", file.to_str().unwrap());
            }

            let (interpretation, system) = {
                let result = parser::parse_file(file.as_ref());
                if let Err(e) = result {
                    println!("❌");
                    eprint!("\n{}: ", error_style.paint("Error"));
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }

                result.unwrap()
            };

            if args.verbose {
                println!("{}", green.paint("✔"));
            }

            let axiom = system.parse_prod_string("Forward").unwrap();
            // todo should this be option<result> or result<option>
            let result = system.derive(axiom, interpretation.run_settings()).unwrap().unwrap();
            let result = interpretation.interpret(&system, &result);

            if let Err(e) = result {
                eprintln!("\n{}", error_style.paint("Error"));
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            }
            let result = result.unwrap();

            println!("Result is: {:?}", result);
        }


    }




    ExitCode::SUCCESS
}