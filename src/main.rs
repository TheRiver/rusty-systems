use rusty_grammar::{RunSettings, System, Token, ToTerminal};
use rusty_grammar::productions::ToProduction;

fn main() {
    println!("Hello, world!");

    let mut system = System::define()
        .terminal("surname")
        .build();
    
    system.add_terminal("bobb");
    system.add_terminal(rusty_grammar::Token::Terminal("fner".to_string()));
    
    
    system.production()
        .named("Company")
        .to(&[Token::from("surname"), Token::from("surname")])
        .to(&[Token::from("surname"), Token::from("and"), Token::from("surname")])
        .build()
        .expect("Unable to build production");

    println!("{:?}", &system);

    let result = system.run(RunSettings::with(
        vec!["EXAMPLE".to_terminal(), "Company".to_production()], 
        10));

    match result {
        Ok(value) => println!("The axiom is {:?}", value),
        Err(err) => eprintln!("There was a problem {err}")
    };


    // system.production("Name")
    //     .chance(0.3).to(vec![ terminal("surname")])
    //     .chance(0.2).to(vec![terminal("surname"), "-", terminal("surname")])
    //     .build();
    //
    // system.run(Production("company"))



    // Name ->  0.3 surname
    //      -> 0.2 surname '-' surname
    //      -> surname 'and' surname

    // company -> 0.3 Name
    //         -> Name 'and sons'
    //         -> Name 'and co'
    //
}
