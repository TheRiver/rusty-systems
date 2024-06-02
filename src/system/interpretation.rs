use std::fmt::Debug;
use crate::prelude::{ProductionString, RunSettings, System};
use crate::tokens::TokenStore;

pub mod abop;
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

    /// Returns default run settings for this interpretation.
    ///
    /// This defines how a system should be derived.
    fn run_settings(&self) -> RunSettings {
        RunSettings::default()
    }

}

/// An interpretation that does nothing except
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
