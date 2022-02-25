pub mod config;
pub mod path;

/// Prelude module, prefixed with `Vrc` to avoid identifier conflict.
pub mod prelude {
    pub use crate::config::{
        Configuration as VrcConfiguration, Parameter as VrcParameter,
        ParameterAddress as VrcParameterAddress, ParameterDataType as VrcParameterDataType,
    };
    pub use crate::path::VrcDirs;
}
