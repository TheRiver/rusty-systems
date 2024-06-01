use crate::geometry::{Path, Point, Vector};
use crate::prelude::*;
use crate::system::family::get_or_init_family;
use crate::system::interpretation::Interpretation;
use crate::system::interpretation::svg::SvgPathInterpretation;
use crate::tokens::TokenStore;

pub mod parser;

pub fn abop_family() -> SystemFamily {
    SystemFamily::define()
        .with_terminal("[", Some("Start a branch"))
        .with_terminal("]", Some("Finish a branch"))
        .with_terminal("+", Some("Turn turtle right"))
        .with_terminal("-", Some("Turn turtle left"))
        .with_production("Forward", Some("Move the turtle forward, drawing a line"))
        .with_production("Move", Some("Move the turtle forward WITHOUT drawing"))
        .with_production("X", Some("A growth point for the plant / branch"))
        .build("ABOP")
        .unwrap()
}

/// This implements a [turtle graphics][turtle] approach
/// to the interpretation of strings, as described in the [Algorithmic Beauty of Plants][abop].
/// 
/// See:
/// * [Logo][logo]
/// * [`AbopSvgInterpretation`]
/// 
/// [turtle]: https://en.wikipedia.org/wiki/Turtle_graphics
/// [logo]: https://en.wikipedia.org/wiki/Logo_(programming_language)
/// [abop]: http://algorithmicbotany.org/papers/#abop
#[derive(Debug, Clone)]
pub struct AbopTurtleInterpretation {
    /// the number of iterations
    n: usize,
    /// in degrees
    delta: f32
}

impl Default for AbopTurtleInterpretation {
    fn default() -> Self {
        AbopTurtleInterpretation::new(5, 22.5)
    }
}

impl AbopTurtleInterpretation {
    pub fn new(n: usize, delta: f32) -> Self {
        Self {
            n,
            delta
        }
    }

    /// The number of rewite iterations that a system should perform
    ///
    /// See [`RunSettings::for_max_iterations`]
    pub fn n(&self) -> usize {
        self.n
    }

    /// The angle that the turtle will move.
    pub fn delta(&self) -> f32 {
        self.delta
    }
}

pub type AbopSvgInterpretation = SvgPathInterpretation<AbopTurtleInterpretation>;

impl Interpretation for AbopTurtleInterpretation {
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
        let space = tokens.get_token("Move").unwrap();
        let right = tokens.get_token("+").unwrap();
        let left = tokens.get_token("-").unwrap();
        let push = tokens.get_token("[").unwrap();
        let pop = tokens.get_token("]").unwrap();

        // We will interpret the tokens as instructions to a LOGO turtle. The following
        // variables keep track of the position that we're at and the direction we're facing.
        // the stack is for the push / pop tokens.
        let mut pos_stack: Vec<(Point, Vector)> = Vec::new();
        let mut pos = Point::zero();
        let mut dir = Vector::new(0.0, 5.0);
        let angle: f64 = self.delta() as f64; // degrees

        // Every time we "branch" (using push and pop), we start a new path.
        let mut paths: Vec<Path> = Vec::new();

        let mut path = Path::new();
        path.push(pos);

        // todo indicate what productions / tokens haven't been used
        for token in string {
            if token == forward {                   // interpret forward tokens.
                pos = pos + dir;
                path.push(pos);
            } else if token == space {
                pos = pos + dir;
                if path.len() > 1 {
                    paths.push(path)
                }
                path = Path::new();
                path.push(pos);
            } else if token == push {               // interpret push tokens. This starts "a branch" of the plant.
                pos_stack.push((pos, dir));
            } else if token == pop {                // interpret pop tokens. This ends "a branch", returning to where the branch started.
                (pos, dir) = pos_stack.pop().expect("Nothing to pop");
                if path.len() > 1 {
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

    fn run_settings(&self) -> RunSettings {
        #[allow(clippy::needless_update)]
        RunSettings {
            max_iterations: self.n,
            ..RunSettings::default()
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::interpretation::Interpretation;

    #[test]
    fn geometry_interpretation() {
        let system = AbopTurtleInterpretation::system().unwrap();
        system.parse_production("Forward -> Forward Forward").unwrap();

        let string = system.parse_prod_string("Forward").unwrap();
        let string = system.derive_once(string).unwrap();
        assert_eq!(string.len(), 2);

        let result = AbopTurtleInterpretation::default_interpret(&system, &string).unwrap();
        assert_eq!(result.len(), 1);

        let result = result[0].clone();
        assert_eq!(result.len(), 3)
    }

}