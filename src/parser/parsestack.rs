use crate::prelude::*;
use crate::parser::iterator::TokenIterator;
use crate::parser::statement::*;
use crate::parser::statement::Match::Matches;

#[derive(Debug)]
pub struct ParseStack<'a> {
    stack: Vec<Statement<'a>>,
    iterator: TokenIterator<'a>
}

impl<'a> ParseStack<'a> {
    pub fn new(iterator: TokenIterator<'a>) -> Self {
        ParseStack {
            stack: Vec::new(),
            iterator
        }
    }


    pub fn parse(&mut self) -> Result<(), Error> {
        loop {
            if let Matches(n) = ProductionString::matches(self.iterator.clone()) {
                self.stack.push(Statement::new(StatementKind::ProductionString, self.iterator.clone().take(n).collect()));
                if n > 0 {
                    self.iterator.nth(n - 1);
                }

                continue;
            }

            break;
        }

        match self.iterator.clone().next() {
            None => Ok(()),
            Some(t) => Err(Error::parse_error(format!("Expected end of string, but found [{t}] instead"))) // todo Needs better error message.
        }
    }
}

impl From<&'static str> for ParseStack<'_> {
    fn from(string: &'static str) -> Self {
        ParseStack::new(TokenIterator::new(string))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    
    #[test]
    fn parse_stack() {
        let mut stack = ParseStack::from("A B C;");

        stack.parse().expect("Unable to parse");
        assert_eq!(stack.stack.len(), 1);

        let compiled: Result<ProductionString> = stack.stack[0].compile();
        assert!(compiled.is_ok());
        assert_eq!(compiled.unwrap(), "A B C".parse().unwrap());

        let mut stack = ParseStack::from("A B C; D E F;");

        stack.parse().expect("Unable to parse");
        assert_eq!(stack.stack.len(), 2);

        let compiled: Result<ProductionString> = stack.stack[0].compile();
        assert!(compiled.is_ok());
        assert_eq!(compiled.unwrap(), "A B C".parse().unwrap());

        let compiled: Result<ProductionString> = stack.stack[1].compile();
        assert!(compiled.is_ok());
        assert_eq!(compiled.unwrap(), "D E F".parse().unwrap());
    }
}