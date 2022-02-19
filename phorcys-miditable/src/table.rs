use serde::{Deserialize, Serialize};
use toml::Value as TomlValue;

/// Specifies the structure of parameters table file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersTable {
    /// VRChat avatar ID.
    /// Used for parameter validity check.
    pub avatar_id: String,

    /// Entries.
    pub entries: Vec<ParametersTableEntry>,
}

/// Represents an entry for parameters table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersTableEntry {
    /// Entry name.
    pub name: String,

    /// Note number by which this entry is triggered.
    pub midi_note: u8,

    /// Channel number by which this entry is triggered.
    /// If not given, all channels are accepted.
    pub midi_channel: Option<u8>,

    /// Parameters to change.
    pub parameters: Vec<(String, TomlValue)>,
}
