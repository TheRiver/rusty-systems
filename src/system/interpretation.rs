use std::fmt::Debug;
use crate::prelude::{ProductionString, System};
use crate::tokens::TokenStore;

pub mod abop;
#[cfg(feature = "skia")]
pub mod skia;
pub mod svg;

pub trait Interpretation: Debug + Sync + Send + Default {
    type Item;

    /// Returns a default system that can handle tokens that this Interpretation
    /// understands.
    ///
    /// Note that an interpretation can [`Interpretation::interpret`] other
    /// systems not produced by this function. THIS FUNCTION IS ONLY A CONVENIENCE
    /// FUNCTION.
    fn system() -> crate::Result<System>;

    fn interpret<S: TokenStore>(&self,
                                tokens: &S,
                                string: &ProductionString) -> crate::Result<Self::Item>;


    fn default_interpret<S: TokenStore>(tokens: &S,
                                        string: &ProductionString) -> crate::Result<Self::Item> {
        let instance = Self::default();
        instance.interpret(tokens, string)
    }

}

/// An interpretation that does nothing except produce
#[derive(Debug, Clone, Default)]
pub struct NullInterpretation {
}

impl NullInterpretation {
}

impl Interpretation for NullInterpretation {
    type Item = ();

    #[inline]
    fn system() -> crate::Result<System> {
        Ok(System::default())
    }

    #[inline]
    fn interpret<S: TokenStore>(&self, _: &S, _: &ProductionString) -> crate::Result<Self::Item> {
        Ok(())
    }
}
