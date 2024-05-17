use rusty_grammar::system::System;

fn main() {
    println!("YAS");
    
    //     variables : X F
    //     constants : + − [ ]
    //     start  : X
    //     rules  : (X → F+[[X]-X]-F[-FX]+X), (F → FF)
    //     angle  : 25°
    
    let plant = System::default();
    
    let prod1 = plant.parse_production("Forward -> Forward Forward")
        .expect("Unable to parse production");
    let prod2 = plant.parse_production("X -> Forward + [ [ X ] - X ] - Forward [ - Forward + X")
        .expect("Unable to parse production");
    
    println!("prod1: {}", plant.format(&prod1).unwrap());
    println!("prod2: {}", plant.format(&prod2).unwrap());
    
    
    
    
    
}