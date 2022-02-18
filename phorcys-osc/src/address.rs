//! Validation and parse of OSC Address.

enum Path {}

impl Path {
    /// Validate
    pub fn is_valid(path: &str) -> bool {
        path.starts_with('/') && path.is_ascii()
    }
}
