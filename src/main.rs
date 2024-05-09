use rusty_grammar::System;

fn main() {
    println!("Hello, world!");

    let system = System::define()
        .terminal("surname")
        .build();
    
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
