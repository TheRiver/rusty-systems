use std::any::Any;

use crate::prelude::{ProductionString, SystemFamily};
use crate::system::family::Interpretation;

pub fn abop_family() -> SystemFamily {
    SystemFamily::define()
        .with_terminal("[", Some("Start a branch"))
        .with_terminal("]", Some("Finish a branch"))
        .with_terminal("+", Some("Turn turtle right"))
        .with_terminal("-", Some("Turn turtle left"))
        .with_production("Forward", Some("Move the turtle forward"))
        .with_production("X", Some("A growth point for the plant / branch"))
        .with_interpretation(AbopInterpretation::boxed())
        .build("ABOP")
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct AbopInterpretation {

}

impl AbopInterpretation {
    pub fn boxed() -> Box<Self> {
        Box::new(AbopInterpretation { })
    }
}


impl Interpretation for AbopInterpretation {
    #[cfg(feature = "skia")]
    fn interpret(&self, _: &ProductionString) -> Box<dyn Any> {
        Box::new(0.0_f32)
    }
    
    #[cfg(not(feature = "skia"))]
    fn interpret(&self, _: &ProductionString) -> Box<dyn Any> {
        Box::new(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "skia")]
    fn skia_interpretation() {
        use crate::prelude::System;
        use crate::system::family::abop_family;

        let family = abop_family();
        let system = System::of_family(family.clone()).unwrap();
        system.parse_production("Forward -> Forward Forward").unwrap();

        let string = system.parse_prod_string("Forward").unwrap();
        let string = system.derive_once(string).unwrap().unwrap();

        let result = family.interpretation.interpret(&string);
        let result = result.downcast_ref::<f32>().copied().unwrap();

        assert_eq!(result, 0.0);
    }

}