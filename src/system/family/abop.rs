use crate::geometry::{Path, Point, Vector};
use crate::prelude::*;
use crate::system::family::{get_or_init_family, Interpretation};
use crate::tokens::TokenStore;

pub fn abop_family() -> SystemFamily {
    SystemFamily::define()
        .with_terminal("[", Some("Start a branch"))
        .with_terminal("]", Some("Finish a branch"))
        .with_terminal("+", Some("Turn turtle right"))
        .with_terminal("-", Some("Turn turtle left"))
        .with_production("Forward", Some("Move the turtle forward"))
        .with_production("X", Some("A growth point for the plant / branch"))
        .build("ABOP")
        .unwrap()
}

#[derive(Debug)]
pub struct AbopInterpretation {
}

impl AbopInterpretation {
    pub fn new() -> Self {
        AbopInterpretation { }
    }
}

impl Default for AbopInterpretation {
    fn default() -> Self {
        AbopInterpretation::new()
    }
}


impl Interpretation for AbopInterpretation {
    type Item = Vec<Path>;

    fn system() -> crate::Result<System> {
        let family = get_or_init_family("ABOP", abop_family);
        System::of_family(family)
    }

    fn interpret<S: TokenStore>(&self,
                                tokens: &S,
                                string: &ProductionString) -> crate::Result<Self::Item> {
        // We need token values to interpret the strings.
        let forward = tokens.get_token("Forward").unwrap();
        let right = tokens.get_token("+").unwrap();
        let left = tokens.get_token("-").unwrap();
        let push = tokens.get_token("[").unwrap();
        let pop = tokens.get_token("]").unwrap();

        // We will interpret the tokens as instructions to a LOGO turtle. The following
        // variables keep track of the position that we're at and the direction we're facing.
        // the stack is for the push / pop tokens.
        let mut pos_stack: Vec<(Point, Vector)> = Vec::new();
        let mut pos = Point::zero();
        let mut dir = Vector::down();
        let angle: f64 = 22.5; // degrees

        // Every time we "branch" (using push and pop), we start a new path.
        let mut paths: Vec<Path> = Vec::new();

        let mut path = Path::new();
        path.push(pos);

        for token in string {
            if token == forward {                   // interpret forward tokens.
                pos = pos + dir;
                path.push(pos);
            } else if token == push {               // interpret push tokens. This starts "a branch" of the plant.
                pos_stack.push((pos, dir));
            } else if token == pop {                // interpret pop tokens. This ends "a branch", returning to where the branch started.
                (pos, dir) = pos_stack.pop().expect("Nothing to pop");
                if !path.is_empty() {
                    paths.push(path)
                }
                path = Path::new();
                path.push(pos);
            } else if token == left {               // Rotate a bit
                dir = dir.rotate(-angle);
            } else if token == right {              // Rotate in the other direction.
                dir = dir.rotate(angle);
            }
        }

        if !path.is_empty() {
            paths.push(path)
        }

        Ok(paths)
    }
}


#[cfg(test)]
mod tests {
    use crate::system::family::abop::AbopInterpretation;
    use crate::system::family::Interpretation;

    #[test]
    fn geometry_interpretation() {
        let system = AbopInterpretation::system().unwrap();
        system.parse_production("Forward -> Forward Forward").unwrap();

        let string = system.parse_prod_string("Forward").unwrap();
        let string = system.derive_once(string).unwrap().unwrap();
        assert_eq!(string.len(), 2);

        let interpretation = AbopInterpretation::default();

        let result = interpretation.interpret(&system, &string).unwrap();
        assert_eq!(result.len(), 1);

        let result = result[0].clone();
        assert_eq!(result.len(), 3)
    }

}