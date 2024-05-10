#[derive(Debug)]
pub enum Token {
    Terminal(String),
    Production
}

#[derive(Debug)]
pub struct System {
    terminals: Vec<Token>
}

#[derive(Debug)]
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
}


impl From<Builder> for System {
    fn from(value: Builder) -> Self {
        System { terminals: value.terminals.into_iter().map(Token::Terminal).collect() }
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



pub struct ProductionBuilder {
    
}

pub struct Production {
    name: String
}

pub struct ProductionBody {
    tokens: Vec<Token>
}