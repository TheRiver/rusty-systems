use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use crate::error::{Error, ErrorKind};
use crate::Result;


pub struct Builder {
}

impl Builder {
    /// Registers the new [`SystemFamily`] and returns a pointer to it.
    ///
    /// Fails if the [`SystemFamily`] is malformed, or if the chosen name
    /// has already been registered ([`ErrorKind::Definitions`]), or
    /// if any locks have been poisoned ([`ErrorKind::Locking`]).
    pub fn register<S: AsRef<str>>(&self, name: S) -> Result<Arc<SystemFamily>> {
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
            name: name.to_string()
        };
        
        map.insert(name.to_string(), Arc::new(family));
        Ok(map.get(name).unwrap().clone())
    }
}


#[derive(Debug, Clone)]
pub struct SystemFamily {
    name: String
}

impl SystemFamily {
    pub fn define() -> Builder {
        Builder { }
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name() {
        let result = SystemFamily::define().register("");
        assert!(result.is_err())
    }
}