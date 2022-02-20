//! Validation and parse of OSC Address.

use crate::error::{Error, Result};

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Prohibited characters in OSC method part.
/// **Slash contained**.
pub const METHOD_PROHIBITED_CHAR: [char; 10] = [' ', '#', '*', ',', '/', '?', '[', ']', '{', '}'];

/// A valid OSC address, reference form.
/// It does not express an OSC address pattern.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(String);

impl Address {
    /// Checks validity and wraps into `Address`.
    pub fn new(address: &str) -> Result<Address> {
        if !address.starts_with('/') || !address.is_ascii() {
            return Err(Error::InvalidAddress);
        }
        if address[1..]
            .split('/')
            .any(|p| p.is_empty() || p.find(METHOD_PROHIBITED_CHAR).is_some())
        {
            return Err(Error::InvalidAddress);
        }

        Ok(Address(address.into()))
    }

    /// Pushes OSC address part.
    pub fn push_part(&mut self, part: &str) -> Result<()> {
        if part.is_empty() || !part.is_ascii() || part.find(METHOD_PROHIBITED_CHAR).is_some() {
            return Err(Error::InvalidAddress);
        }

        self.0.push('/');
        self.0.push_str(part);
        Ok(())
    }

    /// Consume itself and return inner `String`.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Address> for String {
    fn from(x: Address) -> Self {
        x.0
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::Address;

    /// Ensures valid OSC addresses are accepted.
    #[test]
    fn test_valid_address() {
        assert!(Address::new("/address").is_ok());
        assert!(Address::new("/address/to").is_ok());
        assert!(Address::new("/address/to/osc_method").is_ok());
        assert!(Address::new("/address/to/osc_method-2").is_ok());
    }

    /// Ensures invalid OSC addresses are rejected.
    #[test]
    fn test_invalid_address() {
        assert!(Address::new("").is_err());
        assert!(Address::new("X").is_err());
        assert!(Address::new(",").is_err());
        assert!(Address::new("/").is_err());
        assert!(Address::new("/address/").is_err());
        assert!(Address::new("/address/to*").is_err());
        assert!(Address::new("/address/to osc method").is_err());
    }
}
