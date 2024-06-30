use super::*;
use std::iter::FromIterator;
use crate::prelude::ProductionString;
use crate::productions::{Production, ProductionBody, ProductionHead};

/// Provides iteration tools for working with collections of [`Symbol`] objects.
pub trait SymbolIterable {
    /// Iterate over all symbols contained in the object.
    fn all_symbols_iter(&self) -> impl Iterator<Item=Symbol>;

    /// Creates a collection of all the symbols that
    /// [`self.all_symbols_iter`] iterates over.
    /// 
    /// If you use a [`HashSet`] as the return type, then you will have 
    /// a set of unique [`Symbol`] instances.
    fn all_symbols<C: FromIterator<Symbol>>(&self) -> C {
        self.all_symbols_iter().collect()
    }
}


impl SymbolIterable for ProductionString {
    fn all_symbols_iter(&self) -> impl Iterator<Item=Symbol> {
        self.iter().cloned()
    }
}

impl SymbolIterable for ProductionHead {
    fn all_symbols_iter(&self) -> impl Iterator<Item=Symbol> {
        let pre : Box<dyn Iterator<Item=Symbol>> = match self.pre_context() {
            None => Box::new(std::iter::empty()),
            Some(val) => Box::new(val.all_symbols_iter())
        };

        let target = std::iter::once(self.target()).cloned();

        let post : Box<dyn Iterator<Item=Symbol>> = match self.post_context() {
            None => Box::new(std::iter::empty()),
            Some(val) => Box::new(val.all_symbols_iter())
        };

        pre.chain(target).chain(post)
    }
}

impl SymbolIterable for ProductionBody {
    #[inline]
    fn all_symbols_iter(&self) -> impl Iterator<Item=Symbol> {
        self.string().all_symbols_iter()
    }
}

impl SymbolIterable for Production {
    fn all_symbols_iter(&self) -> impl Iterator<Item=Symbol> {
        let mut result : Box<dyn Iterator<Item=Symbol>> = Box::new(self.head().all_symbols_iter());

        for i in self.all_bodies() {
            result = Box::new(result.chain(i.all_symbols_iter()));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{parse_prod_string, parse_production};

    #[test]
    fn can_iterate_over_string() {
        let string = parse_prod_string("A B B C D").unwrap();

        let mut iter = string.all_symbols_iter();

        assert_eq!(iter.next().unwrap().code, get_code("A").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("B").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("B").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("C").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("D").unwrap());

        let b : Vec<_> = string.all_symbols();
        assert_eq!(b.len(), 5);
    }


    #[test]
    fn can_iterate_over_production() {
        let production = parse_production("Pre < Target > Post -> A B C C").unwrap();

        let mut iter = production.all_symbols_iter();

        assert_eq!(iter.next().unwrap().code, get_code("Pre").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("Target").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("Post").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("A").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("B").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("C").unwrap());
        assert_eq!(iter.next().unwrap().code, get_code("C").unwrap());

        let b : Vec<_> = production.all_symbols();
        assert_eq!(b.len(), 7);

        let b : HashSet<_> = production.all_symbols();
        assert_eq!(b.len(), 6);
    }

}