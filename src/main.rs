use rusty_grammar::{System, Token};

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
