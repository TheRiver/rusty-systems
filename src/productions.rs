use std::hash::{Hash, Hasher};
use crate::error::Error;
use crate::prelude::*;
use crate::Token;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Copy, Clone)]
pub enum ChanceKind {
    /// This chance value was set by the user
    Set,
    /// This chance value was derived based on the value
    /// that was left over after considering all the [`ChanceKind::Set`] chance values.
    Derived
}

#[derive(Debug, Copy, Clone)]
pub struct Chance {
    kind: ChanceKind,
    chance: Option<f32>
}

impl Chance {
    /// Creates a new [`ChanceKind::Set`] chance value.
    pub fn new(chance: f32) -> Self {
        assert!(chance > 0_f32, "chance should be positive");
        assert!(chance <= 1.0_f32, "chance should be less than or equal to 1.0");

        Chance {
            kind: ChanceKind::Set,
            chance: Some(chance)
        }
    }

    /// Returns an unset chance object that is meant to be automatically
    /// determined by the system.
    #[inline]
    pub fn empty() -> Self {
        Chance {
            kind: ChanceKind::Derived,
            chance: None
        }
    }

    /// Returns true iff this is of kind [`ChanceKind::Derived`]
    pub fn is_derived(&self) -> bool {
        matches!(self.kind, ChanceKind::Derived)
    }

    /// Returns true iff this is of kind [`ChanceKind::Set`]
    #[inline]
    pub fn is_user_set(&self) -> bool {
        !self.is_derived()
    }

    /// Update the chance value that is stored here.
    ///
    /// Chance values of kind [`ChanceKind::Set`] cannot be updated.
    pub fn update(&mut self, chance: f32) -> crate::Result<()> {
        if self.is_user_set() {
            return Err(Error::definition("user set chance values should not be updated"));
        }

        if chance < 0.0 {
            return Err(Error::definition("chance should be positive"));
        }

        if chance > 1.0 {
            return Err(Error::definition("chance should be less than 1.0"));
        }

        self.chance = Some(chance);
        Ok(())
    }

    #[inline]
    pub fn expect(&self, message: &str) -> f32 {
        self.chance.expect(message)
    }

    #[inline]
    pub fn unwrap(&self) -> f32 {
        self.chance.unwrap()
    }

    #[inline]
    pub fn unwrap_or(&self, default: f32) -> f32 {
        self.chance.unwrap_or(default)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductionHead {
    target: Token
}

impl ProductionHead {
    /// Create a new production head.
    ///
    /// This will return [`Err`] if the given token is not a [`crate::tokens::TokenKind::Production`]
    pub fn build(target: Token) -> crate::Result<Self> {
        if !target.is_production() {
            return Err(Error::general("token should be a Production"));
        }

        Ok(ProductionHead {
            target
        })
    }

    /// Returns the token that this production is a target of.
    #[inline]
    pub fn target(&self) -> Token {
        self.target
    }

    /// Returns true iff this matches the given
    /// string's index position of the string.
    pub fn matches(&self, string: &ProductionString, index: usize) -> bool {
        string.tokens()
            .get(index)
            .map(|token| self.target == *token)
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone)]
pub struct ProductionBody {
    string: ProductionString,
    chance: Chance
}

impl ProductionBody {
    /// Creates a new production body from the given
    /// [`ProductionString`].
    pub fn new(string: ProductionString) -> Self {
        ProductionBody {
            string,
            chance: Chance::empty()
        }
    }
    
    /// Create a production body that is just the empty string
    pub fn empty() -> Self {
        ProductionBody {
            string: ProductionString::empty(),
            chance: Chance::empty()
        }
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.string.len()
    }
    
    #[inline]
    pub fn string(&self) -> &ProductionString {
        &self.string
    }
}

#[derive(Debug, Clone)]
pub struct Production {
    head: ProductionHead,
    body: Vec<ProductionBody>
}

impl Production {
    pub fn new(head: ProductionHead, body: ProductionBody) -> Self {
        Production {
            head,
            body: vec![body]
        }
    }

    #[inline]
    pub fn head(&self) -> &ProductionHead {
        &self.head
    }

    #[inline]
    pub fn body(&self) -> Option<&ProductionBody> {
        if self.body.is_empty() {
            return None
        }

        return self.body.choose(&mut thread_rng())
    }

    /// Returns true iff this production's [`Production::head`] matches the given
    /// string's index position of the string.
    #[inline]
    pub fn matches(&self, string: &ProductionString, index: usize) -> bool {
        self.head().matches(string, index)
    }
    
    pub fn add_body(&mut self, body: ProductionBody) {
        self.body.push(body);
    }
    
    /// Adds all of the body elements from `other` into `self`. 
    pub fn merge(&mut self, other: Self) {
        other.body.into_iter().for_each(|b| self.add_body(b));
    }
}

impl PartialEq for Production {
    fn eq(&self, other: &Self) -> bool {
        self.head().eq(other.head())
    }
}

impl Eq for Production { }

impl Hash for Production {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.head.hash(state);
    }
}

pub trait ProductionStore {
    fn add_production(&mut self, production: Production) -> crate::Result<&Production>;
}

