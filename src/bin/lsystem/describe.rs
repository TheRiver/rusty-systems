use std::process::ExitCode;
use rusty_systems::system::family;
use rusty_systems::system::family::TokenDescription;
use crate::heading_style;


pub fn describe() -> ExitCode {
    let family = family::abop_family();

    let print= |t : &TokenDescription| {
        print!("{: <10}", t.name);
        t.description.iter().for_each(|d| print!("{}", d));
        println!();
    };

    println!("{}", heading_style().paint("Terminals:"));
    family.terminals().for_each(&print);

    println!("\n{}", heading_style().paint("Productions:"));
    family.productions().for_each(&print);

    ExitCode::SUCCESS
}
