//! Secrets (API keys) in the OS keyring, never in the config file or SQLite: Windows Credential
//! Manager through `keyring-core` and its Windows store. Elsewhere (Linux CI, tests) there is no
//! OS keyring, so [`OsKeyring`] reports an error and tests use [`MemorySecrets`].

use std::collections::HashMap;
use std::sync::Mutex;

/// The keyring service every astromech secret is filed under.
pub const SERVICE: &str = "astromech";

pub trait Secrets: Send + Sync {
    /// The secret called `name`, if one is saved.
    fn get(&self, name: &str) -> Result<Option<String>, String>;
    /// Save `value` as `name`, or forget it (`None`).
    fn set(&self, name: &str, value: Option<&str>) -> Result<(), String>;
}

/// The OS keyring (Windows Credential Manager, as the generic credential `<name>.astromech`).
pub struct OsKeyring;

impl OsKeyring {
    pub fn new() -> Self {
        #[cfg(windows)]
        {
            static INIT: std::sync::Once = std::sync::Once::new();
            INIT.call_once(|| match windows_native_keyring_store::Store::new() {
                Ok(s) => keyring_core::set_default_store(s),
                Err(e) => eprintln!("keyring: {e}"),
            });
        }
        Self
    }

    fn entry(name: &str) -> Result<keyring_core::Entry, String> {
        keyring_core::Entry::new(SERVICE, name).map_err(|e| match e {
            keyring_core::Error::NoDefaultStore => "no OS keyring on this system".to_owned(),
            e => format!("keyring: {e}"),
        })
    }
}

impl Default for OsKeyring {
    fn default() -> Self {
        Self::new()
    }
}

impl Secrets for OsKeyring {
    fn get(&self, name: &str) -> Result<Option<String>, String> {
        match Self::entry(name)?.get_password() {
            Ok(v) => Ok(Some(v)),
            Err(keyring_core::Error::NoEntry) => Ok(None),
            Err(e) => Err(format!("keyring: {e}")),
        }
    }

    fn set(&self, name: &str, value: Option<&str>) -> Result<(), String> {
        let e = Self::entry(name)?;
        match value {
            Some(v) => e.set_password(v).map_err(|e| format!("keyring: {e}")),
            None => match e.delete_credential() {
                Ok(()) | Err(keyring_core::Error::NoEntry) => Ok(()),
                Err(e) => Err(format!("keyring: {e}")),
            },
        }
    }
}

/// Secrets kept in memory only (tests).
#[derive(Default)]
pub struct MemorySecrets(Mutex<HashMap<String, String>>);

impl Secrets for MemorySecrets {
    fn get(&self, name: &str) -> Result<Option<String>, String> {
        Ok(self.0.lock().unwrap().get(name).cloned())
    }

    fn set(&self, name: &str, value: Option<&str>) -> Result<(), String> {
        let mut m = self.0.lock().unwrap();
        match value {
            Some(v) => m.insert(name.to_owned(), v.to_owned()),
            None => m.remove(name),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_secrets_save_and_forget() {
        let s = MemorySecrets::default();
        assert_eq!(s.get("k"), Ok(None));
        s.set("k", Some("v")).unwrap();
        assert_eq!(s.get("k"), Ok(Some("v".into())));
        s.set("k", None).unwrap();
        assert_eq!(s.get("k"), Ok(None));
    }

    #[cfg(not(windows))]
    #[test]
    fn without_an_os_keyring_reads_and_writes_fail_plainly() {
        let k = OsKeyring::new();
        assert_eq!(k.get("edsm-api-key"), Err("no OS keyring on this system".into()));
        assert!(k.set("edsm-api-key", Some("x")).is_err());
    }
}
