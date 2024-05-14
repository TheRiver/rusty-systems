use rusty_grammar::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut system = System::new();

    let production = system.parse_production("Company -> surname surname").unwrap();
    println!("The final production: {production:?}");

    let surname = system.get_token("surname").expect("surname token is not present");
    let output = system.derive_once(ProductionString::from(surname));

    println!("output {output:?}");

    let string = system.to_production_string("bob Company snot trot").expect("Unable to parse");
    println!("string+0: {string:?}");
    let string = system.derive_once(string).expect("No result");
    println!("string+1: {string:?}");


    let mut system = System::new();
    system.parse_production("Company -> surname Company").expect("Unable to add production");
    let string = system.to_production_string("Company").expect("Unable to create string");
    let result = system.derive(string, Default::default()).expect("Umable to derive");
    
    println!("\nAfter derivation: \n\t[{}]", system.to_string(&result).unwrap());
    
}
