//! todo describe this

use crate::error::ErrorKind;
use crate::system::interpretation::abop::*;

type ParsedAbop = (AbopTurtleInterpretation, System, ProductionString);

// todo document parse function
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