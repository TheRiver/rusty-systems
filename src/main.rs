use rusty_grammar::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut system = System::new();

    let production = system.parse_production("Company -> surname surname").unwrap();
    println!("The final production: {production:?}");

    let surname = system.get_token("surname").expect("surname token is not present");
    let output = system.derive_once(ProductionString::from(surname));

    println!("output {output:?}");

    let string = system.create_string("bob Company snot trot").expect("Unable to parse");
    println!("string+0: {string:?}");
    let string = system.derive_once(string).expect("No result");
    println!("string+1: {string:?}");
}
