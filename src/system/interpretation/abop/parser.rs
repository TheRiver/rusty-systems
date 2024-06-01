//! todo describe this

use crate::error::ErrorKind;
use crate::system::interpretation::abop::*;

type ParsedAbop = (AbopTurtleInterpretation, System);

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

    #[allow(clippy::while_let_on_iterator)]
    while let Some(line) = lines.next() {
        let line = line.trim();
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
                _ => {
            } }

            continue;
        }

        system.parse_production(line)?;
    }

    let interpretation = AbopTurtleInterpretation::new(n, delta);
    Ok((interpretation, system))
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

fn parse_equality(line: &str) -> crate::Result<EqualityLine> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(Error::general("Invalid equality line"));
    }
    let name = parts[0].trim();
    let value = parts[1].trim();
    Ok(EqualityLine { name, value })
}