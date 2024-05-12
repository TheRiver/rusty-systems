use rusty_grammar::prelude::*;

fn main() {
    println!("Hello, world!");
    
    let mut system = System::new();
    
    system.add_production("Company -> Surname Surname").unwrap();

    // let mut system = System::define()
    //     .terminal("surname")
    //     .build();
    // 
    // system.add_terminal("bobb");
    // system.add_terminal(Token::Terminal("fner".to_string()));
    // 
    // 
    // system.add_production()
    //     .named("Company")
    //     .to(&[Token::from("surname"), Token::from("surname")])
    //     .to(&[Token::from("surname"), Token::from("and"), Token::from("surname")])
    //     .build()
    //     .expect("Unable to build production");
    // 
    // println!("{:?}", &system);
    // 
    // let result = system.run(RunSettings::with(
    //     vec!["EXAMPLE".to_terminal(), "Company".to_production()],
    //     10));
    // 
    // match result {
    //     Ok(value) => println!("The axiom is {:?}", value),
    //     Err(err) => eprintln!("There was a problem {err}")
    // };
    // 
    // let mut system = System::default();
    // let surname = system.terminal("surname").expect("Unable to define surname");
    // 
    // println!("The value is [{surname}]");
    // system.add_production()
    //     .named("Company")
    //     .bob(vec![surname, surname]);

    // system.production()
    //     .named("Company")
    //     .to(vec![surname, surname]);
}
