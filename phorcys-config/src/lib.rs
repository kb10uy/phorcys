pub mod config;

/// Prelude module, prefixed with `Vrc` to avoid identifier conflict.
pub mod prelude {
    pub use crate::config::{
        Configuration as VrcConfiguration, Parameter as VrcParameter,
        ParameterAddress as VrcParameterAddress, ParameterDataType as VrcParameterDataType,
    };
}
