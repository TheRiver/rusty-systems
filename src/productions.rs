//! Utilities for creating rewrite rules for a [`System`].

use std::cell::RefCell;
use std::hash::{Hash, Hasher};

use rand::{Rng, thread_rng};

use crate::{Result};
use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::Symbol;

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
    #[inline]
    pub fn is_derived(&self) -> bool {
        matches!(self.kind, ChanceKind::Derived)
    }

    /// Returns true iff this is of kind [`ChanceKind::Set`]
    #[inline]
    pub fn is_user_set(&self) -> bool {
        matches!(self.kind, ChanceKind::Set)
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
    pre: Option<ProductionString>,
    target: Symbol,
    post: Option<ProductionString>
}

impl ProductionHead {
    /// Create a new production head.
    pub fn build(pre: Option<ProductionString>, target: Symbol, post: Option<ProductionString>) -> Result<Self> {
        Ok(ProductionHead {
            pre,
            target,
            post
        })
    }

    /// Returns the symbol that this production is a target of.
    #[inline]
    pub fn target(&self) -> &Symbol {
        &self.target
    }

    #[inline]
    pub fn pre_context(&self) -> Option<&ProductionString> {
        self.pre.as_ref()
    }

    #[inline]
    pub fn post_context(&self) -> Option<&ProductionString> {
        self.post.as_ref()
    }

    /// Returns true iff this matches the given
    /// string's index position of the string.
    pub fn matches(&self, string: &ProductionString, index: usize) -> bool {
        self.pre_matches(string, index) &&
        self.post_matches(string, index) &&
            string.symbols()
                .get(index)
                .map(|symbol| self.target == *symbol)
                .unwrap_or(false)
    }

    pub fn pre_matches(&self, string: &ProductionString, index: usize) -> bool {
        if self.pre.is_none() {
            return true;
        }

        let left = self.pre.as_ref().unwrap();

        if index == 0 {
            return left.is_empty();
        }

        let symbols: Vec<_> = string.symbols()[0..index].iter().rev().collect();
        if symbols.len() < left.len() {
            return false;
        }

        return left.iter().rev().enumerate().all(|(i, t)| t == symbols[i]);
    }

    pub fn post_matches(&self, string: &ProductionString, index: usize) -> bool {
        if self.post.is_none() {
            return true;
        }

        let right = self.post.as_ref().unwrap();

        if index == string.len() - 1 {
            return right.is_empty();
        }

        let symbols = string.symbols()[index + 1 ..].to_vec();
        if symbols.len() < right.len() {
            return false;
        }

        return right.iter().enumerate().all(|(i, t)| *t == symbols[i]);
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

    /// Creates a new production body from the given
    /// [`ProductionString`] that can occur with the given chance.
    pub fn try_with_chance(chance: f32, string: ProductionString) -> Result<Self> {
        if !(0.0..=1.0).contains(&chance) {
            return Err(Error::new(ErrorKind::Parse, "chance should be between 0.0 and 1.0 inclusive"));
        }

        Ok(ProductionBody {
            string,
            chance: Chance::new(chance),
        })
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

    #[inline]
    pub fn chance(&self) -> &Chance {
        &self.chance
    }
}


/// Represents production rules in an L-System.
///
/// These are rules
/// that may be represented in the form `A -> B`, where
/// A (called here the [`ProductionHead`]) is the symbol
/// that will be matched again, and the symbols after
/// the arrow (in this case the `B`, called here the [`ProductionBody`] is what
/// the symbols matching the head in the input string / axiom will be replaced with.
///
/// See:
/// * [`Production::head`]
/// * [`Production::body`]
/// * [`System::parse_production`]
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

    pub fn body(&self) -> Result<&ProductionBody> {
        if self.body.is_empty() {
            return Err(Error::execution("Production has no bodies set"))
        }

        // Return the only instance. Chance does not matter here.
        if self.body.len() == 1 {
            return Ok(self.body.last().unwrap());
        }

        let total_chance : f32 = self.body.iter()
            .map(|b| b.chance.unwrap_or(0.0))
            .sum();

        if total_chance < 0.0 {
            return Err(Error::execution("chance should never be negative"));
        }

        if total_chance > 1.0 {
            return Err(Error::execution("total chance of production bodies should not be greater than 1.0"));
        }

        let remaining = self.body.iter().filter(|b| b.chance.is_derived()).count();
        let default_chance = if remaining == 0 {
            0_f32
        } else {
            (1.0_f32 - total_chance) / (remaining as f32)
        };

        let mut current = 0_f32;
        let random : f32 = thread_rng().gen_range(0.0..=1.0);

        for body in &self.body {
            current += body.chance.unwrap_or(default_chance);
            if random < current {
                return Ok(body);
            }
        }

        // All remaining chance given to last body.
        return Ok(self.body.last().unwrap());
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
    
    /// Returns a reference to all of the bodies that this production contains
    pub fn all_bodies(&self) -> &Vec<ProductionBody> {
        &self.body
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


/// A trait for collections that accept and store productions.
///
/// Design note: this does not use `&mut self` to allow greater
/// flexibility with sharing [`System`] and other implementers of this
/// trait across threads.
pub trait ProductionStore {
    fn add_production(&self, production: Production) -> Result<Production>;
}

impl ProductionStore for RefCell<Vec<Production>> {
    fn add_production(&self, production: Production) -> Result<Production> {
        let mut vec = self.borrow_mut();
        vec.push(production);
        vec.last().cloned().ok_or_else(|| Error::general("Unable to add production"))
    }
}



#[cfg(test)]
mod tests {
    use crate::parser::parse_prod_string;
    use super::*;

    #[test]
    fn production_matches() {
        let system = System::default();
        let production = system.parse_production("X -> F F").unwrap();

        let string = parse_prod_string("X").unwrap();
        assert!(production.matches(&string, 0));

        let production = system.parse_production("X < X -> F F").unwrap();
        assert!(!production.matches(&string, 0));

        let string = parse_prod_string("X X").unwrap();
        assert!(!production.matches(&string, 0));
        assert!( production.matches(&string, 1));


        let production = system.parse_production("a b < X -> F F").unwrap();
        let string = parse_prod_string("a b X").unwrap();
        assert!(!production.matches(&string, 0));
        assert!(!production.matches(&string, 1));
        assert!( production.matches(&string, 2));



        let production = system.parse_production("X > X -> F F").unwrap();
        assert!(!production.matches(&string, 0));

        let string = parse_prod_string("X X").unwrap();
        assert!( production.matches(&string, 0));
        assert!(!production.matches(&string, 1));


        let production = system.parse_production("X > a b -> F F").unwrap();
        let string = parse_prod_string("a X a b").unwrap();
        assert!(!production.matches(&string, 0));
        assert!( production.matches(&string, 1));
        assert!(!production.matches(&string, 2));
        assert!(!production.matches(&string, 3));

        let system = System::default();
        let string = parse_prod_string("G S S S X").unwrap();
        // system.parse_production("G > S -> ").unwrap();
        let production = system.parse_production("G < S -> S G").unwrap();

        assert!(!production.matches(&string, 0));
        assert!( production.matches(&string, 1));
        assert!(!production.matches(&string, 2));
    }
}