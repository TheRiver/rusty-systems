use crate::Error;
use crate::parser::Token;
use crate::prelude::ProductionString;
use crate::symbols::Symbol;

#[derive(Debug)]
pub struct Statement<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> From<Vec<Token<'a>>> for Statement<'a> {
    fn from(tokens: Vec<Token<'a>>) -> Self {
        Statement {
            tokens
        }
    }
}


impl<'a> Statement<'a> {
    pub fn compile<T>(self) -> Result<T, Error>
    where T: TryFrom<Statement<'a>, Error=Error>
    {
        T::try_from(self)
    }

    #[inline]
    pub fn tokens(&self) -> &Vec<Token<'a>> {
        &self.tokens
    }

    #[inline]
    pub fn tokens_iter(&self) -> impl Iterator<Item=Token<'_>> {
        self.tokens.iter().copied()
    }
}


impl<'a> TryFrom<Statement<'a>> for ProductionString {
    type Error = Error;

    fn try_from(statement: Statement<'a>) -> Result<Self, Self::Error> {
        let symbols: Result<Vec<Symbol>, _> = statement.tokens_iter()
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
        let statement = Statement::from(vec![
            Token::new("a", 0, 1),
            Token::new("b", 1, 2),
            Token::new("c", 2, 3),
        ]);

        let result : Result<ProductionString> = statement.compile();
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, parse_prod_string("a b c").unwrap());
    }
}