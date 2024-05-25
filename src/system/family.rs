use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug};
use std::sync::{Arc, OnceLock, RwLock};
use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::Result;
use crate::tokens::TokenKind;

mod abop;
pub use abop::abop_family;

pub struct Builder {
    terminals: Vec<TokenDescription>,
    productions: Vec<TokenDescription>,
    interpretation: Option<Box<dyn Interpretation>>
}

impl Builder {
    /// Register a terminal, with an optional description of what that terminal represents.
    ///
    /// This does *not* create terminals (see, for instance, [`System`]),
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
    /// This does *not* create terminals (see, for instance, [`System`]),
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

    pub fn with_interpretation(mut self, interpretation:  Box<dyn Interpretation>) -> Self {
        self.interpretation = Some(interpretation);
        self
    }

    /// Registers the new [`SystemFamily`] and returns a pointer to it.
    ///
    /// Fails if the [`SystemFamily`] is malformed, or if the chosen name
    /// has already been registered ([`ErrorKind::Definitions`]), or
    /// if any locks have been poisoned ([`ErrorKind::Locking`]).
    pub fn register<S: AsRef<str>>(self, name: S) -> Result<Arc<SystemFamily>> {
        let name = name.as_ref();

        let mut map = reference().write()?;
        if map.contains_key(name) {
            return Err(Error::new(ErrorKind::Duplicate,
                                  format!("family name [{}] is already taken", name)));
        }

        let family = self.build(name)?;
        map.insert(name.to_string(), Arc::new(family));
        Ok(map.get(name).unwrap().clone())
    }

    /// Create a [`SystemFamily`] without registering it.
    ///
    /// See [`Builder::register`].
    pub fn build<S: AsRef<str>>(self, name: S) -> Result<SystemFamily> {
        let name = name.as_ref();
        if name.trim().is_empty() {
            return Err(Error::new(ErrorKind::Definitions, "family name cannot be empty"));
        }

        if self.interpretation.is_none() {
            return Err(Error::definition("family should have an interpretation"))
        }

        Ok(SystemFamily {
            name: name.to_string(),
            terminals: self.terminals.into_iter().collect(),
            productions: self.productions.into_iter().collect(),
            interpretation: Arc::from(self.interpretation.unwrap())
        })
    }

}

pub trait Interpretation: Debug + Sync + Send {
    fn interpret(&self, string: &ProductionString) -> Box<dyn Any>;
}

pub trait AsProduced<T> {
    fn result(&self) -> T;
}

/// An interpretation that does nothing except produce
#[derive(Debug, Clone)]
pub struct NullInterpretation {
}

impl NullInterpretation {
    pub fn new() -> Self {
        NullInterpretation { }
    }

    pub fn boxed() -> Box<Self> {
        Box::new(NullInterpretation::new())
    }
}

impl Default for NullInterpretation {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpretation for NullInterpretation {
    fn interpret(&self, _: &ProductionString) -> Box<dyn Any> {
        Box::new(())
    }
}

impl AsProduced<f32> for NullInterpretation {
    fn result(&self) -> f32 {
        0.0
    }
}

#[derive(Debug, Clone)]
pub struct SystemFamily {
    name: String,
    terminals: HashMap<String, TokenDescription>,
    productions: HashMap<String, TokenDescription>,
    interpretation: Arc<dyn Interpretation>
}

impl SystemFamily {
    /// Define a family of [`System`] instances.
    pub fn define() -> Builder {
        Builder { terminals: Vec::new(), productions: Vec::new(), interpretation: None }
    }

    /// Returns the name of the [`SystemFamily`]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns an iterator over all of the terminals registered for this family.
    pub fn terminals(&self) -> impl Iterator<Item=&TokenDescription> {
        self.terminals.values()
    }

    /// Returns an iterator over all of the Productions registered for this family.
    pub fn productions(&self) -> impl Iterator<Item=&TokenDescription> {
        self.productions.values()
    }

    /// Returns an iterator over all terminals and productions of this family.
    pub fn tokens(&self) -> impl Iterator<Item=&TokenDescription> {
        self.terminals().chain(self.productions())
    }

    pub fn interpretation(&self) -> Arc<dyn Interpretation> {
        self.interpretation.clone()
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

/// Examples
///
/// ```
/// use rusty_systems::system::family;
/// let abop = family::get_or_init_family("ABOP", family::abop_family);
/// ```
pub fn get_or_init_family<S, F>(name: S, default: F) -> Arc<SystemFamily>
    where S: AsRef<str>,
          F: FnOnce() -> SystemFamily
{
    // See if we have a value using just a read lock.
    if let Some(value) = get_family(name.as_ref()) {
        return value;
    }

    let mut map = reference().write().unwrap();
    // Now we have a write lock. Need to double-check that the family
    // hasn't been registered
    if let Some(value) = map.get(name.as_ref()) {
        return value.clone();
    }

    // Build, but now synchronised via the above write guard.
    let family = default();

    let family = Arc::new(family);
    map.insert(name.as_ref().to_string(), family.clone());

    family
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

pub fn register(family: SystemFamily) -> Result<Arc<SystemFamily>> {
    let name = family.name().clone();
    let mut map = reference().write()?;

    if map.get(&name).is_some() {
        return Err(Error::new(ErrorKind::Duplicate, format!("family {name} has already been registered")));
    }

    let family = Arc::new(family);
    map.insert(name.to_string(), family.clone());

    Ok(family)
}

pub trait TryIntoFamily {
    fn into_family(self) -> Result<Arc<SystemFamily>>;
}

impl TryIntoFamily for Arc<SystemFamily> {
    fn into_family(self) -> Result<Arc<SystemFamily>> {
        Ok(self.clone())
    }
}

impl TryIntoFamily for SystemFamily {
    fn into_family(self) -> Result<Arc<SystemFamily>> {
        Ok(Arc::new(self))
    }
}

impl TryIntoFamily for &str {
    fn into_family(self) -> Result<Arc<SystemFamily>> {
        get_family(self).ok_or_else(||
            Error::definition(format!("family {self} has not been registered")))
    }
}

impl TryIntoFamily for String {
    fn into_family(self) -> Result<Arc<SystemFamily>> {
        self.as_str().into_family()
    }
}

#[derive(Debug, Clone)]
pub struct TokenDescription {
    /// Indicates the kind of token this [`TokenDescription`] represents
    pub kind: TokenKind,
    /// The token's name
    pub name: String,
    /// What this token represents.
    pub description: Option<String>
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
        let result = SystemFamily::define()
            .with_interpretation(Box::<NullInterpretation>::default())
            .register("");
        assert!(result.is_err());

        let result = SystemFamily::define()
            .with_interpretation(Box::<NullInterpretation>::default())
            .register("bob");
        assert!(result.is_ok());
    }

    #[test]
    fn can_register() {
        let result = SystemFamily::define()
            .with_terminal("surname", Some("It's a surname"))
            .with_production("Company", None)
            .with_interpretation(Box::<NullInterpretation>::default())
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

    #[test]
    fn abop_available() {
        let abop = get_or_init_family("ABOP", abop_family);
        assert_eq!(abop.name(), "ABOP");
    }
}