use serde::{Deserialize, Serialize};

/// Represents the whole configuration of an avatar.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Configuration {
    pub id: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
}

/// Represents a parameter definition in configuration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub input: Option<ParameterAddress>,
    pub output: ParameterAddress,
}

/// Represents a pair of parameter address and data type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ParameterAddress {
    /// Parameter address.
    /// It should starts with '/'.
    pub address: String,

    /// Parameter data type. Originally named `type`.
    #[serde(rename = "type")]
    pub parameter_type: ParameterDataType,
}

/// Describes the data type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ParameterDataType {
    /// Boolean parameter.
    Bool,

    /// Integer parameter (unsigned 8bit).
    Int,

    /// Float-point number parameter.
    Float,
}
