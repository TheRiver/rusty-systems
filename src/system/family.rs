use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use crate::error::{Error, ErrorKind};
use crate::Result;
use crate::tokens::TokenKind;


pub struct Builder {
    terminals: Vec<TokenDescription>,
    productions: Vec<TokenDescription>,
}

impl Builder {
    /// Register a terminal, with an optional description of what that terminal represents.
    ///
    /// This does *not* create terminals (see, for instance, [`crate::prelude::System::add_token`]),
    /// it just defines what tokens are allowed.
    ///
    /// For example:
    /// ```
    /// use rusty_systems::prelude::SystemFamily;
    /// SystemFamily::define()
    ///     .with_terminal("surname", Some("Represents a randomly chosen surname"))
    ///     .with_terminal("hyphen", None);
    /// ```
    pub fn with_terminal<S: AsRef<str>>(mut self, name: S, description: Option<S>) -> Self {
        let token = TokenDescription {
            kind: TokenKind::Terminal,
            name: name.as_ref().to_string(),
            description: description.map(|s| s.as_ref().to_string())
        };
        self.terminals.push(token);
        self
    }

    /// Register a production, with an optional description of the kind of action that production represents.
    ///
    /// This does *not* create terminals (see, for instance, [`crate::prelude::System::add_token`]),
    /// it just defines what tokens are allowed.
    ///
    /// For example:
    /// ```
    /// use rusty_systems::prelude::SystemFamily;
    /// SystemFamily::define()
    ///     .with_production("f", Some("Draw a line 'forward'"))
    ///     .with_production("+", None);
    /// ```
    pub fn with_production<S: AsRef<str>>(mut self, name: S, description: Option<S>) -> Self {
        let token = TokenDescription {
            kind: TokenKind::Production,
            name: name.as_ref().to_string(),
            description: description.map(|s| s.as_ref().to_string())
        };
        self.productions.push(token);
        self
    }


    /// Registers the new [`SystemFamily`] and returns a pointer to it.
    ///
    /// Fails if the [`SystemFamily`] is malformed, or if the chosen name
    /// has already been registered ([`ErrorKind::Definitions`]), or
    /// if any locks have been poisoned ([`ErrorKind::Locking`]).
    pub fn register<S: AsRef<str>>(self, name: S) -> Result<Arc<SystemFamily>> {
        let name = name.as_ref();
        if name.trim().is_empty() {
            return Err(Error::new(ErrorKind::Definitions, "family name cannot be empty"));
        }

        let mut map = reference().write()?;
        if map.contains_key(name) {
            return Err(Error::new(ErrorKind::Definitions,
                                  format!("family name [{}] is already taken", name)));
        }

        let family = SystemFamily {
            name: name.to_string(),
            terminals: self.terminals.into_iter().collect(),
            productions: self.productions.into_iter().collect()
        };

        map.insert(name.to_string(), Arc::new(family));
        Ok(map.get(name).unwrap().clone())
    }
}


#[derive(Debug, Clone)]
pub struct SystemFamily {
    name: String,
    terminals: HashMap<String, TokenDescription>,
    productions: HashMap<String, TokenDescription>
}

impl SystemFamily {
    /// Define a family of [`crate::prelude::System`] instances.
    pub fn define() -> Builder {
        Builder { terminals: Vec::new(), productions: Vec::new() }
    }

    /// Returns the name of the [`SystemFamily`]
    pub fn name(&self) -> &String {
        &self.name
    }
}



/// Returns a pointer to the given [`SystemFamily`], if it has been registered.
///
/// Note: if synchronisation locks are poisoned this will return [`None`],
/// as though there was no family registered.
pub fn get_family<S: AsRef<str>>(name: S) -> Option<Arc<SystemFamily>> {
    let map = reference().read().ok()?;
    map.get(name.as_ref()).cloned()
}

/// Returns true if and only if a family with the given name has been registered.
///
/// Note: if synchronisation locks are poisoned this will return [`None`],
/// as though there was no family registered.
pub fn family_exists<S: AsRef<str>>(name: S) -> bool {
    get_family(name).is_some()
}

/// Private. This returns a pointer to the family registry.
fn reference() -> &'static RwLock<HashMap<String, Arc<SystemFamily>>> {
    static REGISTRY : OnceLock<RwLock<HashMap<String, Arc<SystemFamily>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}




#[derive(Debug, Clone)]
struct TokenDescription {
    /// Indicates the kind of token this [`TokenDescription`] represents
    kind: TokenKind,
    /// The token's name
    name: String,
    /// What this token represents.
    description: Option<String>
}

impl FromIterator<TokenDescription> for HashMap<String, TokenDescription> {
    fn from_iter<T: IntoIterator<Item=TokenDescription>>(iter: T) -> Self {
        let mut result = HashMap::new();

        for i in iter {
            result.insert(i.name.clone(), i);
        }

        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name() {
        let result = SystemFamily::define().register("");
        assert!(result.is_err())
    }

    #[test]
    fn can_register() {
        let result = SystemFamily::define()
            .with_terminal("surname", Some("It's a surname"))
            .with_production("Company", None)
            .register("NameSystems")
            .unwrap();

        assert_eq!(result.terminals.len(), 1);
        assert_eq!(result.productions.len(), 1);
        assert_eq!(result.name(), "NameSystems");

        let surname = result.terminals.get("surname").unwrap();
        assert_eq!(surname.name, "surname");
        assert!(surname.description.is_some());
        assert_eq!(surname.description.as_ref().unwrap(), "It's a surname");
    }
}