use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Token {
    Terminal(String),
    Production
}

#[derive(Debug, Clone)]
pub struct System {
    terminals: Vec<Token>,
    productions: Vec<Production>
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
        ProductionBuilder { head: None, system: self }
    }
}


impl From<Builder> for System {
    fn from(value: Builder) -> Self {
        System { 
            terminals: value.terminals.into_iter().map(Token::Terminal).collect(),
            productions: Vec::new()
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::Terminal(value)
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


#[derive(Debug)]
pub struct ProductionBuilder<'a> {
    head: Option<ProductionHead>,
    system: &'a mut System
}

#[derive(Debug, Clone)]
pub struct Production {
    head: ProductionHead
}

#[derive(Debug, Clone)]
pub struct ProductionHead {
    name: String
}

#[derive(Debug, Clone)]
pub struct ProductionBody {
    tokens: Vec<Token>
}

impl<'a> ProductionBuilder<'a> {
    /// Set the name of the production.
    pub fn named<T: ToString>(mut self, name: T) -> Self {
        self.head = Some(ProductionHead::from(name.to_string()));
        self
    }
    
    pub fn build(mut self) -> Result<&'a Production> {
        if self.head.is_none() {
            return Err(GrammarError::message("Production has no head"));
        }
        self.system.productions.push(Production { head: self.head.expect("No head") });
        
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

}

#[derive(Debug, Clone)]
pub enum GrammarError {
    General(String)
}

impl Display for GrammarError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { GrammarError::General(message) => write!(f, "{}", message) }
    }
}

impl std::error::Error for GrammarError { }


pub type Result<T> = std::result::Result<T, GrammarError>;

impl GrammarError { 
    fn message<T : ToString>(message: T) -> Self {
        GrammarError::General(message.to_string())
    }
}