use rusty_grammar::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut system = System::new();

    let production = system.parse_production("Company -> Surname Surname").unwrap();

    println!("The final production: {production:?}");
}
