//! Contains manipulation and types about OSC Packet.

use crate::{
    address::Address,
    data::Value,
    error::{Error, Result},
};

/// Represents an immutable OSC packet.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Packet {
    address: Box<str>,
    arguments: Box<[Value]>,
}

impl Packet {
    /// Returns method path.
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Returns method arguments.
    pub fn arguments(&self) -> &[Value] {
        &self.arguments
    }

    /// Consumes itself and splits into owned path and arguments.
    pub fn split_into(self) -> (String, Vec<Value>) {
        (self.address.into(), self.arguments.into())
    }

    /// Serialize this packet.
    /// Returned bytes **does not** contain the whole size.
    pub fn serialize(self) -> Box<[u8]> {
        let mut serialized_bytes = Vec::with_capacity(32);

        // Address
        let mut terminated_address: Vec<u8> = self.address.into_boxed_bytes().into();
        terminated_address.push(0);
        Value::align_bytes(&mut terminated_address);
        serialized_bytes.append(&mut terminated_address);

        // Tags
        let mut type_tags: String = ",".into();
        for arg in &self.arguments[..] {
            arg.push_type_tag_to(&mut type_tags);
        }
        let mut type_tags: Vec<u8> = type_tags.into_bytes().into();
        type_tags.push(0);
        Value::align_bytes(&mut type_tags);
        serialized_bytes.append(&mut type_tags);

        // Data
        for arg in Vec::from(self.arguments) {
            arg.write_aligned_into(&mut serialized_bytes);
        }

        serialized_bytes.into_boxed_slice()
    }
}

impl Packet {
    /// Deserializes bytes into packet.
    pub fn deserialize(bytes: Vec<u8>) -> Result<Packet> {
        let (address, tag, argument_bytes) = Packet::split_bytes(bytes)?;

        let mut arguments = vec![];
        let mut rest_tag = &tag.as_bytes()[1..];
        let mut rest_argument = &argument_bytes[..];
        while !rest_tag.is_empty() {
            let (arg, next_tag, next_argument) = Packet::parse_argument(rest_tag, rest_argument)?;
            rest_tag = next_tag;
            rest_argument = next_argument;
            arguments.push(arg);
        }

        Ok(Packet {
            address: address.into_boxed_str(),
            arguments: arguments.into_boxed_slice(),
        })
    }

    /// Splits raw bytes array into address, types tag, and argument data.
    /// Returned address and tag are guaranteed that they have correct leaders and consist of only ASCII-bytes.
    fn split_bytes(mut bytes: Vec<u8>) -> Result<(String, String, Vec<u8>)> {
        // Check alignment
        if bytes.len() % 4 != 0 {
            return Err(Error::UnalignedData);
        }

        // Slice address
        let rest_bytes = &bytes[..];
        let address_first_nul = match rest_bytes.iter().position(|&b| b == 0x00) {
            None => return Err(Error::NotTerminated),
            Some(0) => return Err(Error::InvalidAddress),
            Some(i) => i,
        };
        let address: Vec<u8> = (&bytes[..address_first_nul]).into();
        let address = String::from_utf8(address).map_err(|_| Error::InvalidAddress)?;
        let address_aligned = Value::aligned_length(address_first_nul + 1);
        if !Address::is_valid(&address) {
            return Err(Error::InvalidAddress);
        }

        // Slice types tag
        let rest_bytes = &rest_bytes[address_aligned..];
        let tag_first_nul = match rest_bytes.iter().position(|&b| b == 0x00) {
            None => return Err(Error::NotTerminated),
            Some(0) => return Err(Error::InvalidTag),
            Some(i) => i,
        };
        let tag: Vec<u8> = (&rest_bytes[..tag_first_nul]).into();
        let tag = String::from_utf8(tag).map_err(|_| Error::InvalidTag)?;
        let tag_aligned = Value::aligned_length(tag_first_nul + 1);
        if !tag.starts_with(',') || !tag.is_ascii() {
            return Err(Error::InvalidTag);
        }

        // Cut out arguments
        let arguments_left = bytes.split_off(address_aligned + tag_aligned);

        Ok((address, tag, arguments_left))
    }

    /// Parses an argument.
    fn parse_argument<'t, 'a>(
        rest_tag: &'t [u8],
        rest_argument: &'a [u8],
    ) -> Result<(Value, &'t [u8], &'a [u8])> {
        if rest_tag.is_empty() {
            return Err(Error::IllegalStructure);
        }

        match rest_tag[0] {
            b'N' => Ok((Value::Nil, &rest_tag[1..], rest_argument)),
            b'I' => Ok((Value::Infinitum, &rest_tag[1..], rest_argument)),
            b'T' => Ok((Value::Boolean(true), &rest_tag[1..], rest_argument)),
            b'F' => Ok((Value::Boolean(false), &rest_tag[1..], rest_argument)),
            b'c' => {
                if rest_argument.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                Ok((
                    Value::Character(u32::from_be_bytes([
                        rest_argument[0],
                        rest_argument[1],
                        rest_argument[2],
                        rest_argument[3],
                    ]) as u8 as char),
                    &rest_tag[1..],
                    &rest_argument[4..],
                ))
            }
            b'i' => {
                if rest_argument.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                Ok((
                    Value::Int32(i32::from_be_bytes([
                        rest_argument[0],
                        rest_argument[1],
                        rest_argument[2],
                        rest_argument[3],
                    ])),
                    &rest_tag[1..],
                    &rest_argument[4..],
                ))
            }
            b'h' => {
                if rest_argument.len() < 8 {
                    return Err(Error::NotEnoughData);
                }

                Ok((
                    Value::Int64(i64::from_be_bytes([
                        rest_argument[0],
                        rest_argument[1],
                        rest_argument[2],
                        rest_argument[3],
                        rest_argument[4],
                        rest_argument[5],
                        rest_argument[6],
                        rest_argument[7],
                    ])),
                    &rest_tag[1..],
                    &rest_argument[8..],
                ))
            }
            b'f' => {
                if rest_argument.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                Ok((
                    Value::Float32(f32::from_be_bytes([
                        rest_argument[0],
                        rest_argument[1],
                        rest_argument[2],
                        rest_argument[3],
                    ])),
                    &rest_tag[1..],
                    &rest_argument[4..],
                ))
            }
            b'd' => {
                if rest_argument.len() < 8 {
                    return Err(Error::NotEnoughData);
                }

                Ok((
                    Value::Float64(f64::from_be_bytes([
                        rest_argument[0],
                        rest_argument[1],
                        rest_argument[2],
                        rest_argument[3],
                        rest_argument[4],
                        rest_argument[5],
                        rest_argument[6],
                        rest_argument[7],
                    ])),
                    &rest_tag[1..],
                    &rest_argument[8..],
                ))
            }
            b'r' => {
                if rest_argument.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                Ok((
                    Value::Color([
                        rest_argument[0],
                        rest_argument[1],
                        rest_argument[2],
                        rest_argument[3],
                    ]),
                    &rest_tag[1..],
                    &rest_argument[4..],
                ))
            }
            b'm' => {
                if rest_argument.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                Ok((
                    Value::MidiMessage([
                        rest_argument[0],
                        rest_argument[1],
                        rest_argument[2],
                        rest_argument[3],
                    ]),
                    &rest_tag[1..],
                    &rest_argument[4..],
                ))
            }
            b't' => {
                if rest_argument.len() < 8 {
                    return Err(Error::NotEnoughData);
                }

                Ok((
                    Value::TimeTag(u64::from_be_bytes([
                        rest_argument[0],
                        rest_argument[1],
                        rest_argument[2],
                        rest_argument[3],
                        rest_argument[4],
                        rest_argument[5],
                        rest_argument[6],
                        rest_argument[7],
                    ])),
                    &rest_tag[1..],
                    &rest_argument[8..],
                ))
            }
            b's' => {
                if rest_argument.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                let first_nul = match rest_argument.iter().position(|&b| b == 0x00) {
                    None => return Err(Error::NotTerminated),
                    Some(i) => i,
                };
                let string: Vec<u8> = (&rest_argument[..first_nul]).into();
                let string = String::from_utf8(string).map_err(|_| Error::InvalidAddress)?;
                let string_aligned = Value::aligned_length(first_nul + 1);

                Ok((
                    Value::String(string),
                    &rest_tag[1..],
                    &rest_argument[string_aligned..],
                ))
            }
            b'S' => {
                if rest_argument.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                let first_nul = match rest_argument.iter().position(|&b| b == 0x00) {
                    None => return Err(Error::NotTerminated),
                    Some(i) => i,
                };
                let string: Vec<u8> = (&rest_argument[..first_nul]).into();
                let string = String::from_utf8(string).map_err(|_| Error::InvalidAddress)?;
                let string_aligned = Value::aligned_length(first_nul + 1);

                Ok((
                    Value::Alternative(string),
                    &rest_tag[1..],
                    &rest_argument[string_aligned..],
                ))
            }
            b'b' => {
                if rest_argument.len() < 4 {
                    return Err(Error::NotEnoughData);
                }

                let length = i32::from_be_bytes([
                    rest_argument[0],
                    rest_argument[1],
                    rest_argument[2],
                    rest_argument[3],
                ]) as usize;
                let aligned_length = Value::aligned_length(length);

                if rest_argument.len() < aligned_length + 4 {
                    return Err(Error::NotEnoughData);
                }
                let blob = (&rest_argument[4..(length + 4)]).into();

                Ok((
                    Value::Blob(blob),
                    &rest_tag[1..],
                    &rest_argument[aligned_length..],
                ))
            }
            b'[' => {
                let mut values = vec![];
                let mut inner_rest_tag = &rest_tag[1..];
                let mut inner_rest_argument = rest_argument;

                loop {
                    if rest_argument.is_empty() {
                        break Err(Error::IllegalStructure);
                    }
                    if rest_argument[0] == b']' {
                        break Ok((Value::Array(values), &rest_tag[1..], rest_argument));
                    }

                    let (arg, next_tag, next_argument) =
                        Packet::parse_argument(inner_rest_tag, inner_rest_argument)?;
                    inner_rest_tag = next_tag;
                    inner_rest_argument = next_argument;
                    values.push(arg);
                }
            }
            b']' => Err(Error::IllegalStructure),
            otherwise => Err(Error::UnknownType(otherwise)),
        }
    }
}

/// Builder object for `Packet`.
#[derive(Debug, Clone)]
pub struct PacketBuilder {
    address: String,
    arguments: Vec<Value>,
}

impl PacketBuilder {
    /// Creates new builder.
    pub fn new(address: &str) -> Result<PacketBuilder> {
        if !Address::is_valid(&address) {
            return Err(Error::InvalidAddress);
        }

        Ok(PacketBuilder {
            address: address.into(),
            arguments: vec![],
        })
    }

    /// Builds immutable `Packet`.
    pub fn build(self) -> Packet {
        Packet {
            address: self.address.into_boxed_str(),
            arguments: self.arguments.into_boxed_slice(),
        }
    }

    /// Pushes an argument for packet.
    pub fn push_argument(mut self, argument: Value) -> Self {
        self.arguments.push(argument);
        self
    }

    /// Replaces completely arguments with given those.
    pub fn set_arguments(mut self, arguments: impl Into<Vec<Value>>) -> Self {
        self.arguments = arguments.into();
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{
        error::Error,
        packet::{Packet, PacketBuilder},
    };

    /// Ensures that `Packet::deserialize()` processes valid bytes.
    /// The packet has only address.
    #[test]
    fn test_deserializer_simple() {
        let bytes = vec![
            b'/', b'p', b'a', b't', // Address
            b'h', 0x00, 0x00, 0x00, // Address
            b',', 0x00, 0x00, 0x00, // Tag
        ];

        let packet = Packet::deserialize(bytes).expect("Deserialize failed");
        let (address, arguments) = packet.split_into();
        assert_eq!(address, "/path");
        assert_eq!(arguments, &[]);
    }

    /// Ensures that `Packet::deserialize()` rejects invalid address.
    /// The packet has only address.
    #[test]
    fn test_deserializer_error_address() {
        let bytes = vec![
            b'X', b'p', b'a', b't', // Address
            b'h', 0x00, 0x00, 0x00, // Address
            b',', 0x00, 0x00, 0x00, // Tag
        ];

        let packet = Packet::deserialize(bytes);
        assert_eq!(packet, Err(Error::InvalidAddress));
    }

    /// Ensures that `Packet::deserialize()` rejects invalid address.
    /// The packet has only address.
    #[test]
    fn test_deserializer_error_tag() {
        let bytes = vec![
            b'/', b'p', b'a', b't', // Address
            b'h', 0x00, 0x00, 0x00, // Address
            b'/', 0x00, 0x00, 0x00, // Tag
        ];

        let packet = Packet::deserialize(bytes);
        assert_eq!(packet, Err(Error::InvalidTag));
    }

    /// Ensures that `Packet::serialize()` processes valid packet.
    /// The packet has only address.
    #[test]
    fn test_serializer_simple() {
        let packet = PacketBuilder::new("/path/to")
            .expect("Should valid")
            .build();
        let packet_bytes = packet.serialize();

        assert_eq!(
            &packet_bytes[..],
            &[
                b'/', b'p', b'a', b't', // Address
                b'h', b'/', b't', b'o', // Address
                0x00, 0x00, 0x00, 0x00, // Address
                b',', 0x00, 0x00, 0x00, // Tag
            ]
        );
    }
}
