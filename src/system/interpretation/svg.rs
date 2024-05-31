//!
//! See https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths

use crate::geometry::Path;
use crate::prelude::{Interpretation, ProductionString, System};
use crate::tokens::TokenStore;


#[derive(Debug, Clone, Default)]
pub struct SvgPathInterpretation<T> 
    where T: Interpretation<Item=Vec<Path>>
{
    initial: T
}

impl<T> Interpretation for SvgPathInterpretation<T> 
    where T: Interpretation<Item=Vec<Path>> 
{
    type Item = ();

    fn system() -> crate::Result<System> {
        todo!()
    }

    fn interpret<S: TokenStore>(&self, tokens: &S, string: &ProductionString) -> crate::Result<Self::Item> {
        todo!()
    }
}
    