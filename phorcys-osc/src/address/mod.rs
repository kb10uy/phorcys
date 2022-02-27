//! Contains manipulations of address and address patterns.

pub mod address;

#[cfg(feature = "address-pattern")]
pub mod pattern;

pub use crate::address::address::Address;

#[cfg(feature = "address-pattern")]
pub use crate::address::pattern::{AddressPattern, Expression};
