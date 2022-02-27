//! OSC bundle manipulations.

use crate::{
    data::TimeTag,
    error::{Error, Result},
    packet::Packet,
};

/// OSC-string of OSC-bundle header.
pub const BUNDLE_HEADER: &[u8] = b"#bundle\0";

/// Represents an OSC-bundle data.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Bundle {
    time_tag: TimeTag,
    elements: Box<[Packet]>,
}

impl Bundle {
    /// Returns timetag.
    pub fn time_tag(&self) -> TimeTag {
        self.time_tag
    }

    /// Returns elements reference.
    pub fn elements(&self) -> &[Packet] {
        &self.elements
    }

    /// Consumes itself and splits into owned timetag and elements.
    pub fn split_into(self) -> (TimeTag, Vec<Packet>) {
        (self.time_tag, self.elements.into())
    }
}

impl Bundle {
    pub fn deserialize(bytes: Vec<u8>) -> Result<Bundle> {
        if bytes.len() < 16 || &bytes[..8] != BUNDLE_HEADER {
            return Err(Error::InvalidBundle);
        }
        let time_tag = u64::from_be_bytes(bytes[8..16].try_into().expect("Wrong length"));
        let mut elements = vec![];
    }
}
