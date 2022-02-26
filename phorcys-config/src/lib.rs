pub mod vrchat;
pub mod xsoverlay;

/// Prelude module, prefixed with `Vrc` to avoid identifier conflict.
pub mod prelude {
    pub use crate::vrchat::{
        config::{
            Configuration as VrcConfiguration, Parameter as VrcParameter,
            ParameterAddress as VrcParameterAddress, ParameterDataType as VrcParameterDataType,
        },
        dir::VrcDirs,
    };
}
