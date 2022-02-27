//! OSC bundle manipulations.

use crate::{
    data::TimeTag,
    error::{Error, Result},
    packet::Packet,
};

/// OSC-string of OSC-bundle header.
pub const BUNDLE_HEADER: &[u8] = b"#bundle\0";

/// Represents an OSC Bundle data.
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

    /// Serializes this packet.
    /// Returned bytes **does not** contain the whole size.
    pub fn serialize(self) -> Box<[u8]> {
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(BUNDLE_HEADER);
        bytes.extend_from_slice(&self.time_tag.0.to_be_bytes());
        for element in self.elements.to_vec() {
            bytes.extend_from_slice(&element.serialize());
        }

        bytes.into_boxed_slice()
    }
}

impl Bundle {
    /// Deserializes an OSC Bundle.
    pub fn deserialize(bytes: &[u8]) -> Result<Bundle> {
        if bytes.len() < 16 || &bytes[..8] != BUNDLE_HEADER {
            return Err(Error::InvalidBundle);
        }
        let time_tag = TimeTag(u64::from_be_bytes(
            bytes[8..16].try_into().expect("Wrong length"),
        ));
        let mut elements = vec![];
        let mut rest_bytes = &bytes[16..];
        while rest_bytes.len() >= 4 {
            let length =
                i32::from_be_bytes(rest_bytes[..4].try_into().expect("Wrong length")) as usize;
            if rest_bytes[4..].len() < length {
                return Err(Error::NotEnoughData);
            }

            // TODO: parse rest_bytes
            let element_bytes = &rest_bytes[4..(length + 4)];
            let element = Packet::deserialize(element_bytes)?;
            elements.push(element);

            rest_bytes = &rest_bytes[(length + 4)..];
        }

        Ok(Bundle {
            time_tag,
            elements: elements.into_boxed_slice(),
        })
    }
}
