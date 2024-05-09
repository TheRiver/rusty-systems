use std::cell::RefCell;
use std::collections::HashSet;

pub enum Token {
    Terminal,
    Production
}

#[derive(Debug)]
pub struct System {
    terminals: Vec<String>
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

    pub fn terminal<T: ToString>(&mut self, name: T) -> &Self {
        self.terminals.push(name.to_string());
        self
    }
}


impl From<Builder> for System {
    fn from(value: Builder) -> Self {
        System { terminals: value.terminals }
    }
    
     
}