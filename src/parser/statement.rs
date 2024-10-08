use std::fmt::{Display, Formatter};
use crate::Error;
use crate::parser::iterator::TokenIterator;
use crate::parser::{Token, TokenKind};
use crate::prelude::ProductionString;
use crate::symbols::Symbol;
use self::Match::*;

#[derive(Debug, Default)]
pub struct Statement<'a> {
    kind: StatementKind,
    tokens: Vec<Token<'a>>,
    error: Option<Error>
}


impl From<Error> for Statement<'_> {
    fn from(error: Error) -> Self {
        Statement {
            error: Some(error),
            ..Statement::default()
        }
    }
}


impl<'a> Statement<'a> {
    #[inline]
    pub fn new(kind: StatementKind, tokens: Vec<Token<'a>>) -> Self {
        Statement {
            kind,
            tokens,
            error: None
        }
    }

    pub fn compile<T>(&self) -> Result<T, Error>
    where T: ParsableType
    {
        self.error.clone()
            .map_or_else(
                || T::compile(CheckedStatement::new(self)), 
                |e| Err(e)
            )
    }

    #[inline]
    pub fn tokens(&self) -> &Vec<Token<'a>> {
        &self.tokens
    }

    #[inline]
    pub fn tokens_iter(&self) -> impl Iterator<Item=Token<'_>> {
        self.tokens.iter().copied()
    }

    #[inline]
    pub fn error(&self) -> Option<&Error> {
        self.error.as_ref()
    }

    #[inline]
    pub fn kind(&self) -> StatementKind {
        self.kind
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum StatementKind {
    #[default]
    Error,
    ProductionString
}

impl Display for StatementKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementKind::Error => write!(f, "Error"),
            StatementKind::ProductionString => write!(f, "ProductionString")
        }
    }
}

/// A struct whose sole purpose is to ensure a type check against
/// [`StatementKind`] when accessing a [`Statement`] instance.
/// 
/// See [`CheckedStatement::expect`], which performs the check.
#[derive(Debug, Copy, Clone)]
pub struct CheckedStatement<'a, 'b> {
    statement: &'a Statement<'b>
}

impl<'a, 'b> CheckedStatement<'a, 'b> {
    #[inline]
    pub fn new(statement: &'a Statement<'b>) -> Self {
        CheckedStatement {
            statement
        }
    }
    
    pub fn expect(&self, kind: StatementKind) -> Result<&'a Statement<'b>, Error> {
        if kind != self.statement.kind() {
            return Err(Error::parse_error(format!("Statement is of type {} and not {}", self.statement.kind, kind)))
        }
        
        Ok(self.statement)
    }
}




/// Records how something we wanted to parse matches
/// against the input tokens ([`TokenIterator`]).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Match {
    /// This is a match, with how many tokens it matched.
    Matches(usize),
    /// This isn't a match, and returns the number of tokens it did match.
    No(usize)
}




pub trait ParsableType: Sized {
    fn kind() -> StatementKind;
    fn matches(iterator: TokenIterator) -> Match;
    
    fn compile_from<'a, I>(iterator: I) -> Result<Self, Error> 
        where I: Iterator<Item=Token<'a>>;

    fn compile(statement: CheckedStatement<'_, '_>) -> Result<Self, Error> {
        let statement = statement.expect(Self::kind())?;
        Self::compile_from(statement.tokens_iter())
    }
}

impl ParsableType for ProductionString {
    #[inline]
    fn kind() -> StatementKind {
        StatementKind::ProductionString
    }

    fn matches(iterator: TokenIterator) -> Match {
        for (i, token) in iterator.enumerate() {
            match token.kind {
                TokenKind::Symbol => continue,
                TokenKind::Terminator => return Matches(i + 1),
                _ => return No(i)
            }
        }

        No(0)
    }

    fn compile_from<'a, I>(iterator: I) -> Result<Self, Error>
    where
        I: Iterator<Item=Token<'a>>
    {
        let symbols: Result<Vec<Symbol>, _> = iterator
            .take_while(|t| !t.is_terminal())
            .map(Token::try_into)
            .collect();
        Ok(ProductionString::from(symbols?))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_prod_string;
    use super::*;
    use crate::Result;

    #[test]
    fn statement_to_production_string() {
        let statement = Statement::new(
            StatementKind::ProductionString,
            vec![
                Token::new("a", 0, 1),
                Token::new("b", 1, 2),
                Token::new("c", 2, 3),
            ]);
        
        let result : Result<ProductionString> = statement.compile();
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, parse_prod_string("a b c").unwrap());
    }


    #[test]
    fn matches_production_string() {
        assert_eq!(ProductionString::matches(TokenIterator::new("")), No(0));
        assert_eq!(ProductionString::matches(TokenIterator::new("->")), No(0));
        assert_eq!(ProductionString::matches(TokenIterator::new("A B C;")), Matches(4));
        assert_eq!(ProductionString::matches(TokenIterator::new("A B C; A B;")), Matches(4));
        assert_eq!(ProductionString::matches(TokenIterator::new("A;")), Matches(2));
        assert_eq!(ProductionString::matches(TokenIterator::new(";")), Matches(1));
        assert_eq!(ProductionString::matches(TokenIterator::new("A -> B")), No(1));
    }
}