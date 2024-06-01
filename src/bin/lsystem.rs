use std::process::ExitCode;
use std::sync::OnceLock;

use ansi_term::{Color, Style};
use clap::{Args, Parser, Subcommand};

use crate::derive::handle_derive;

mod derive;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command
}

#[derive(Debug, Args)]
struct DeriveArgs {
    file: Box<std::path::Path>, // todo Figure out how to document
    #[arg(short, long)]
    output: Box<std::path::Path>,
    #[arg(long, default_value = "500")]
    width: usize,
    #[arg(long, default_value = "500")]
    height: usize
}

#[derive(Debug, Subcommand)]
enum Command {
    Derive(DeriveArgs)
}


fn green() -> &'static Style {
    static GREEN: OnceLock<Style> = OnceLock::new();
    GREEN.get_or_init(|| Style::new().fg(Color::Green))
}

fn error_style() -> &'static Style {
    static ERROR: OnceLock<Style> = OnceLock::new();
    ERROR.get_or_init(|| Color::Red.underline())
}






// todo Need to set up features to include libraries only for this.
fn main() -> ExitCode {
    let args = Cli::parse();


    match &args.command {
        Command::Derive(derive) => {
            handle_derive(&args, &derive)
        }
    }
}




