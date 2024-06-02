//! Parsing grammars from *the Algorithmic Beauty of Plants* 
//! 
//! # The format
//! 
//! ```plant
//! # Comments start with a hash. 
//! # This describes the plant 5 in fig 1.24 of ABOP (pg 25)
//! n = 6           # Number of derivation iterations. This is one more iteration than in ABOP
//! delta = 22.5    # The angle that the + and - tokens turn the "turtle" by.
//! 
//! initial: X      # The starting string
//! 
//! # And now the productions
//! Forward -> Forward Forward 
//! X -> Forward + [ [ X ] - X ] - Forward [ - Forward X ] + X
//! ```
//! 
//! # Parsing
//! 
//! If we have a string in the format given above, you can parse it like so:
//! 
//! ```
//! use rusty_systems::system::family::abop::parser::parse;
//! # let plant_string = "initial: X\nX -> F F";
//!
//! let (interpretation, system, initial_string) = parse(plant_string).unwrap();
//!
//! ```
//! 
//! The [`parse`] function returns the following:
//! 
//! * `interpretation`, being a [`AbopTurtleInterpretation`]. If you want to output SVG from
//!    this, see [`SvgPathInterpretation`].
//! * `system`, being a [`System`] ready to run. 
//! * `initial_string`, being the [`ProductionString`] the file specifies as the initial string.

use crate::error::ErrorKind;
use crate::system::interpretation::abop::*;

type ParsedAbop = (AbopTurtleInterpretation, System, ProductionString);

/// Parses a string in a bespoke "plant" format. See the [namespace](crate::system::interpretation::abop::parser)
/// namespace documentation for more information. 
///
/// * `interpretation`, being a [`AbopTurtleInterpretation`]. If you want to output SVG from
///    this, see [`SvgPathInterpretation`].
/// * `system`, being a [`System`] ready to run. 
/// * `initial_string`, being the [`ProductionString`] the file specifies as the initial string.
/// 
/// See [`parse_file`] to parse a file containing a string in this format.
pub fn parse(string: &str) -> crate::Result<ParsedAbop> {
    let string = string.trim();
    if string.is_empty() {
        return Err(Error::new(ErrorKind::Parse, "String should not be empty"));
    }

    let mut lines = string.lines().peekable();

    let mut n = 2_usize;
    let mut delta = 5.0_f32;

    let system = AbopTurtleInterpretation::system()?;
    let mut prod_count = 0_usize;
    let mut initial : Option<&str> = None;

    #[allow(clippy::while_let_on_iterator)]
    while let Some(line) = lines.next() {
        let line = remove_comment(line);
        if line.is_empty() {
            continue;
        }

        if is_equality_line(line) {
            let equality = parse_equality(line)?;
            match equality.name {
                "n" | "N" => {
                    n = equality.value.parse()?;
                }
                "d" | "D" | "delta" | "∂" => {
                    delta = equality.value.parse()?;
                }
                _ => return Err(Error::new(ErrorKind::Parse, format!("Unrecognised line {}", line)))
            }

            continue;
        }

        if is_initial(line) {
            initial = Some(parse_initial(line));
            continue;
        }

        prod_count += 1;
        system.parse_production(line)?;
    }

    if prod_count == 0 {
        return Err(Error::new(ErrorKind::Parse, "No productions have been supplied"));
    }

    if initial.is_none() {
        return Err(Error::new(ErrorKind::Parse, "No initial axiom has been supplied"));
    }

    let initial = system.parse_prod_string(initial.unwrap())?;

    let interpretation = AbopTurtleInterpretation::new(n, delta);
    Ok((interpretation, system, initial))
}

// todo document
pub fn parse_file<P: AsRef<std::path::Path>>(name: P) -> crate::Result<ParsedAbop> {
    let contents = std::fs::read_to_string(name)?;
    parse(&contents)
}


struct EqualityLine<'a> {
    pub name: &'a str,
    pub value: &'a str
}

fn is_equality_line(line: &str) -> bool {
    line.contains('=')
}

fn is_initial(line: &str) -> bool {
    line.trim().starts_with("initial:")
}

fn parse_initial(line: &str) -> &str {
    let parts: Vec<_> = line.splitn(2, ':').collect();
    parts[1].trim()
}

fn parse_equality(line: &str) -> crate::Result<EqualityLine> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(Error::general("Invalid equality line"));
    }
    let name = parts[0].trim();
    let value = parts[1].trim();
    Ok(EqualityLine { name, value })
}

fn remove_comment(line: &str) -> &str {
    line.split('#').next().unwrap().trim()
}



#[cfg(test)]
mod tests {
    use super::*;
    
    static GENERAL : &str = "# Totally for testing purposes
n = 6
delta = 22.5

initial: X # Here we go
# Start on a line

Forward -> Forward Forward 
X -> Forward + [ [ X ] - X ] - Forward [ - Forward X ] + X

# ENDED";
    

    #[test]
    fn test_parsing() {
        
        let result = parse(GENERAL);
        assert!(result.is_ok());
        
        let (_, system, ..) = result.unwrap();
        assert_eq!(system.production_len(), 2);

        // The test data does not add any more tokens to the system than the family does.
        assert_eq!(system.token_len(), AbopTurtleInterpretation::system().unwrap().token_len());
        
    }
}