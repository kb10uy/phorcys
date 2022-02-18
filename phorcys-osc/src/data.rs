//! Data types.

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

    /// String `s`.
    String(String),

    /// Alternative form of string `S`.
    Alternative(String),

    /// Blob `b`.
    Blob(Vec<u8>),

    /// Array with `[ ~ ]`.
    Array(Vec<Value>),
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
