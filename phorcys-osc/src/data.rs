//! Data types.

/// Represents a time tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TimeTag(pub u64);

/// Represents single data in OSC packet.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    /// Nil `N`.
    Nil,

    /// Infinitum `I`.
    Infinitum,

    /// Boolean true or false `T / F`.
    Boolean(bool),

    /// ASCII character `c`.
    Character(char),

    /// 32bit integer `i`.
    Int32(i32),

    /// 64bit integer `i`.
    Int64(i64),

    /// 32bit float-point `d`.
    Float32(f32),

    /// 64bit float-point `d`.
    Float64(f64),

    /// Color `r`.
    Color([u8; 4]),

    /// MIDI message `m`.
    MidiMessage([u8; 4]),

    /// OSC time tag `t`.
    TimeTag(TimeTag),

    /// String `s`.
    String(String),

    /// Alternative form of string `S`.
    Alternative(String),

    /// Blob `b`.
    Blob(Vec<u8>),

    /// Array with `[ ~ ]`.
    Array(Vec<Value>),
}

impl Value {
    /// Pushes type tag of this value.
    pub fn push_type_tag_to(&self, tag_string: &mut String) {
        match self {
            Value::Nil => tag_string.push('N'),
            Value::Infinitum => tag_string.push('I'),
            Value::Boolean(true) => tag_string.push('T'),
            Value::Boolean(false) => tag_string.push('F'),
            Value::Character(_) => tag_string.push('c'),
            Value::Int32(_) => tag_string.push('i'),
            Value::Int64(_) => tag_string.push('h'),
            Value::Float32(_) => tag_string.push('f'),
            Value::Float64(_) => tag_string.push('d'),
            Value::Color(_) => tag_string.push('r'),
            Value::MidiMessage(_) => tag_string.push('m'),
            Value::TimeTag(_) => tag_string.push('t'),
            Value::String(_) => tag_string.push('s'),
            Value::Alternative(_) => tag_string.push('S'),
            Value::Blob(_) => tag_string.push('b'),
            Value::Array(values) => {
                tag_string.push('[');
                for v in values {
                    v.push_type_tag_to(tag_string);
                }
                tag_string.push(']');
            }
        }
    }

    /// Consumes itself and writes argument data into `buffer` with aligning.
    pub fn write_aligned_into(self, buffer: &mut Vec<u8>) {
        match self {
            Value::Nil => (),
            Value::Infinitum => (),
            Value::Boolean(_) => (),
            Value::Character(c) => buffer.extend_from_slice(&(c as u32).to_be_bytes()),
            Value::Int32(x) => buffer.extend_from_slice(&x.to_be_bytes()),
            Value::Int64(x) => buffer.extend_from_slice(&x.to_be_bytes()),
            Value::Float32(x) => buffer.extend_from_slice(&x.to_be_bytes()),
            Value::Float64(x) => buffer.extend_from_slice(&x.to_be_bytes()),
            Value::Color(c) => buffer.extend_from_slice(&c),
            Value::MidiMessage(m) => buffer.extend_from_slice(&m),
            Value::TimeTag(t) => buffer.extend_from_slice(&t.0.to_be_bytes()),
            Value::String(s) => {
                let mut aligned = s.into_bytes();
                aligned.push(0);
                Value::align_bytes(&mut aligned);
                buffer.extend_from_slice(&aligned);
            }
            Value::Alternative(s) => {
                let mut aligned = s.into_bytes();
                aligned.push(0);
                Value::align_bytes(&mut aligned);
                buffer.extend_from_slice(&aligned);
            }
            Value::Blob(mut b) if b.len() < i32::MAX as usize => {
                let length_bytes = (b.len() as i32).to_be_bytes();
                Value::align_bytes(&mut b);
                buffer.extend_from_slice(&length_bytes);
                buffer.extend_from_slice(&b);
            }
            Value::Array(values) => {
                for v in values {
                    v.write_aligned_into(buffer);
                }
            }
            _ => unreachable!("Invalid data"),
        }
    }
}

impl Value {
    /// Align the `data` length to multiple of 4.
    pub fn aligned_length(original_length: usize) -> usize {
        let mut words = original_length >> 2;
        if original_length % 4 != 0 {
            words += 1;
        }
        words << 2
    }

    /// Aligns the length `bytes`.
    /// This function will not append NUL terminator.
    pub fn align_bytes(bytes: &mut Vec<u8>) {
        let aligned_length = Value::aligned_length(bytes.len());
        for _ in 0..(aligned_length - bytes.len()) {
            bytes.push(0);
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        Value::Character(c)
    }
}

impl From<i32> for Value {
    fn from(x: i32) -> Self {
        Value::Int32(x)
    }
}

impl From<i64> for Value {
    fn from(x: i64) -> Self {
        Value::Int64(x)
    }
}

impl From<f32> for Value {
    fn from(x: f32) -> Self {
        Value::Float32(x)
    }
}

impl From<f64> for Value {
    fn from(x: f64) -> Self {
        Value::Float64(x)
    }
}

impl From<String> for Value {
    fn from(x: String) -> Self {
        Value::String(x)
    }
}

impl From<&str> for Value {
    fn from(x: &str) -> Self {
        Value::String(x.into())
    }
}

#[cfg(test)]
mod test {
    use super::Value;

    #[test]
    fn test_value_length_calculation() {
        assert_eq!(Value::aligned_length(1), 4);
        assert_eq!(Value::aligned_length(4), 4);
        assert_eq!(Value::aligned_length(101), 104);
    }

    #[test]
    fn test_aligned_write_null() {
        let mut buffer = vec![];
        Value::Nil.write_aligned_into(&mut buffer);
        Value::Infinitum.write_aligned_into(&mut buffer);
        Value::Boolean(false).write_aligned_into(&mut buffer);
        Value::Boolean(true).write_aligned_into(&mut buffer);
        assert_eq!(buffer, &[]);
    }

    #[test]
    fn test_aligned_write() {
        let mut buffer = vec![];
        Value::Int32(0x12345678).write_aligned_into(&mut buffer);
        Value::Int64(0x78ABCDEF01234567).write_aligned_into(&mut buffer);
        Value::Float32(1.0).write_aligned_into(&mut buffer);
        Value::Float64(1.0).write_aligned_into(&mut buffer);
        assert_eq!(
            buffer,
            &[
                0x12, 0x34, 0x56, 0x78, // Int32
                0x78, 0xAB, 0xCD, 0xEF, // Int64
                0x01, 0x23, 0x45, 0x67, // Int64
                0x3F, 0x80, 0x00, 0x00, // Float32
                0x3F, 0xF0, 0x00, 0x00, // Float64
                0x00, 0x00, 0x00, 0x00, // Float64
            ]
        );
    }
}
