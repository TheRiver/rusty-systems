use rand::Rng;
use crate::{System, Token};
use crate::error::{Error, ErrorKind};
use crate::prelude::*;

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




#[derive(Debug)]
pub struct ProductionBuilder<'a> {
    head: Option<ProductionHead>,
    system: &'a mut System,
    bodies: Vec<ProductionBody>
}


impl<'a> ProductionBuilder<'a> {
    pub fn new(system: &'a mut System) -> ProductionBuilder<'a> {
        ProductionBuilder {
            head: None,
            system,
            bodies: Vec::new()
        }
    }

    pub fn head(&self) -> &Option<ProductionHead> {
        &self.head
    }
}



#[derive(Debug, Clone)]
pub struct Production {
    head: ProductionHead,
    bodies: Vec<ProductionBody>
}

#[derive(Debug, Clone)]
pub struct ProductionHead {
    name: String
}

#[derive(Debug, Clone)]
pub struct ProductionBody {
    chance: Chance,
    tokens: Vec<Token>
}

impl<'a> ProductionBuilder<'a> {
    /// Set the name of the production.
    pub fn named<T: ToString>(mut self, name: T) -> Self {
        self.head = Some(ProductionHead::from(name.to_string()));
        self
    }

    pub fn to_chance(mut self, chance: f32, body: &[Token]) -> Self {
        self.bodies.push(
            ProductionBody {
                chance: Chance::new(chance),
                tokens: body.to_vec(),
            }
        );
        self
    }

    pub fn to(mut self, body: &[Token]) -> Self {
        self.bodies.push(
            ProductionBody {
                chance: Chance::empty(),
                tokens: body.to_vec(),
            }
        );
        self
    }

    pub fn build(mut self) -> crate::Result<&'a Production> {
        if self.head.is_none() {
            return Err(Error::definition("Production has no head"));
        }
        if self.bodies.is_empty() {
            return Err(Error::definition("Production has no bodies"));
        }

        let chance_total : f32 = self.bodies.iter()
            .filter(|b| b.chance.is_user_set())
            .map(|b| b.chance.unwrap_or(0.0))
            .sum();

        if chance_total > 1.0 {
            return Err(Error::definition("total chance should not sum to more than 1.0"));
        }

        let remaining_rules : usize = self.bodies.iter()
            .filter(|b| b.chance.is_derived())
            .count();

        // For the rules that have not been set with
        // chance values, we want to ensure that the remaining
        // chance is distributed amongst them.
        if remaining_rules > 0 {
            let remaining_chance = 1.0 - chance_total;
            let per_rule_chance = remaining_chance / (remaining_rules as f32);

            for body in self.bodies.iter_mut() {
                if !body.chance.is_derived() { continue }
                body.chance.update(per_rule_chance)?
            }
        }

        self.system.productions.push(Production {
            head: self.head.unwrap(),
            bodies: self.bodies
        });
        return Ok(self.system.productions.last().unwrap());
    }
}

impl From<String> for ProductionHead {
    fn from(value: String) -> Self {
        ProductionHead { name: value }
    }
}

impl ProductionHead {
    /// Returns what the production is called.
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn matches(&self, token: &Token) -> bool {
        if let Token::Production(name) = token {
            return name == self.name();
        }

        false
    }
}

impl Production {
    #[inline]
    pub fn matches(&self, token: &Token) -> bool {
        self.head.matches(token)
    }

    pub fn run(&self) -> crate::Result<Axiom> {
        if self.bodies.is_empty() {
            return Err(Error::new(ErrorKind::Execution, format!("production [{}] has no bodies", self.head.name)))
        }
        
        let random : f32 = rand::thread_rng().gen_range(0.0..=1.0);
        let mut pos = 0.0_f32;
        
        for body in &self.bodies {
            pos += body.chance.chance
                .ok_or_else(|| 
                    Error::new(ErrorKind::Execution, 
                               format!("production [{}] has no chance value", self.head.name)))?;
            
            if pos >= random {
                return Ok(Axiom::from(body.tokens.clone()))
            }
        }
        
        // We like have a rounding problem. Because of how we've set up our
        // chances, the selected production body will have been the last one.
        Ok(Axiom::from(self.bodies.last().unwrap().tokens.clone()))
    }
}



#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::Token;

    #[test]
    fn no_chance_specified() {
        let mut system = System::define().build();
        system.production()
            .named("test")
            .to(&[Token::Terminal("bob".to_string())])
            .to(&[Token::Terminal("bobette".to_string())])
            .build().unwrap();

        let production = &system.productions[0];
        assert_eq!(production.bodies[0].chance.unwrap(), 0.5);
        assert_eq!(production.bodies[1].chance.unwrap(), 0.5);
    }

    #[test]
    fn fills_missing_chance() {
        let mut system = System::define().build();
        system.production()
            .named("test")
            .to_chance(0.75, &[Token::Terminal("bob".to_string())])
            .to(&[Token::Terminal("bobette".to_string())])
            .build().unwrap();

        let production = &system.productions[0];
        assert_eq!(production.bodies[0].chance.unwrap(), 0.75);
        assert_eq!(production.bodies[1].chance.unwrap(), 0.25);
    }

}


pub trait ToProduction {
    fn to_production(&self) -> Token;
}

impl ToProduction for &str {
    fn to_production(&self) -> Token {
        Token::Production(self.to_string())
    }
}