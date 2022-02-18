//! Validation and parse of OSC Address.

/// Address manipulation.
pub enum Address {}

impl Address {
    /// Validate
    pub fn is_valid(path: &str) -> bool {
        path.starts_with('/') && path.is_ascii()
    }
}
