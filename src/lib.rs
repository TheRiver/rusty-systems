use productions::{Production, ProductionBuilder};

use crate::error::{Error, ErrorKind};

pub mod error;
pub mod productions;

pub mod prelude {
    pub use super::error::Error;
    pub use super::System;
}


/// A result type for functions that can return errors.
pub type Result<T> = std::result::Result<T, error::Error>;


#[derive(Debug, Clone)]
pub enum Token {
    Terminal(String),
    Production(String)
}

#[derive(Debug, Clone)]
pub struct System {
    terminals: Vec<Token>,
    productions: Vec<Production>,
    axiom: Option<Vec<Token>>
}

#[derive(Debug, Clone)]
pub struct Builder {
    terminals: Vec<String>
}


impl Builder {
    pub fn terminal<T: ToString>(mut self, name: T) -> Self {
        self.terminals.push(name.to_string());
        self
    }
    
    pub fn build(self) -> System {
        System::from(self)
    }
}



impl System {
    pub fn new() -> Self {
        System {
            terminals: Vec::new(),
            productions: Vec::new(),
            axiom: None
        }
    }

    pub fn define() -> Builder {
        Builder { terminals: Vec::new() }
    }

    pub fn add_terminal<T: ToTerminal>(&mut self, terminal: T) -> &Self {
        self.terminals.push(terminal.to_terminal());
        self
    }

    /// Start defining a new production to add to the system.
    ///
    /// See [`Production`]
    pub fn production(&mut self) -> ProductionBuilder {
        ProductionBuilder::new(self)
    }

    /// Create an output string
    ///
    /// The simplest way of running (will panic because the system has not been set up).
    ///
    /// ```
    /// use rusty_grammar::{RunSettings, System, Token, ToTerminal};
    /// let system = System::default();
    ///
    /// system.run(RunSettings::with(vec!["END".to_terminal()], 10)).unwrap(); 
    /// ```
    /// 
    /// If you have already set an axiom in the system, and just want to control
    /// the number of iterations, one might do this (note that this example fails
    /// since no starting axiom has been set):
    ///
    /// ```should_panic
    /// use rusty_grammar::{RunSettings, System, Token, ToTerminal};
    /// let system = System::default();
    /// 
    /// // Set the axiom here. 
    ///
    /// system.run(10).unwrap();                        // Run for a maximum of 10 iterations.
    /// system.run(RunSettings::default()).unwrap();    // Use default run sets.
    /// ```
    /// 
    /// // This example fails since no starting axiom has been set. 
    pub fn run<T>(&self, options: T) -> Result<()>
        where T : Into<RunSettings>
    {
        let options = options.into();
        println!("It is running for {}", options.max_iterations);
        
        let axiom = self.axiom.as_ref().or(options.axiom.as_ref());
        
        if axiom.is_none() {
            return Err(Error::new(ErrorKind::Execution, "a starting axiom should be supplied"))
        }
        
        if self.productions.is_empty() {
            return Ok(())
        }
        
        Ok(())
    }
}

impl Default for System {
    fn default() -> Self {
        System::new()
    }
}


impl From<Builder> for System {
    fn from(value: Builder) -> Self {
        System {
            terminals: value.terminals.into_iter().map(Token::Terminal).collect(),
            productions: Vec::new(),
            axiom: None
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::Terminal(value)
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Token::Terminal(value.to_string())
    }
}

pub trait ToTerminal {
    fn to_terminal(self) -> Token;
}

impl ToTerminal for String {
    fn to_terminal(self) -> Token {
        Token::from(self)
    }
}

impl ToTerminal for &str {
    fn to_terminal(self) -> Token {
        Token::from(self.to_string())
    }
}

impl ToTerminal for Token {
    fn to_terminal(self) -> Token {
        self
    }
}


const DEFAULT_ITERATIONS: usize = 10;

#[derive(Debug, Clone)]
pub struct RunSettings {
    max_iterations: usize,
    axiom: Option<Vec<Token>>
}

impl RunSettings {
    pub fn new(max_iterations: usize) -> Self {
        RunSettings {
            max_iterations,
            axiom: None
        }
    }
    
    pub fn with(axiom: Vec<Token>, max_iterations: usize) -> RunSettings {
        RunSettings {
            max_iterations,
            axiom: Some(axiom)
        }
    }
}

impl Default for RunSettings {
    fn default() -> Self {
        RunSettings::new(DEFAULT_ITERATIONS)
    }
}


impl From<usize> for RunSettings {
    fn from(value: usize) -> Self {
        RunSettings::new(value)
    }
}