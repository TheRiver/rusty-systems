use std::process::ExitCode;
use std::sync::OnceLock;

use ansi_term::{Color, Style};
use clap::Parser;

use crate::derive::handle_derive;

mod derive;
mod describe;
mod cli;



// todo Need to set up features to include libraries only for this.
fn main() -> ExitCode {
    let args = cli::Cli::parse();


    match &args.command {
        cli::Command::Derive(derive) => {
            handle_derive(&args, derive)
        },
        cli::Command::Describe => {
            describe::describe()
        }
    }
}




fn green() -> &'static Style {
    static GREEN: OnceLock<Style> = OnceLock::new();
    GREEN.get_or_init(|| Style::new().fg(Color::Green))
}

fn error_style() -> &'static Style {
    static ERROR: OnceLock<Style> = OnceLock::new();
    ERROR.get_or_init(|| Color::Red.underline())
}

fn heading_style() -> &'static Style {
    static HEADING: OnceLock<Style> = OnceLock::new();
    HEADING.get_or_init(|| Color::White.underline().bold())
}

