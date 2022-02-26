pub mod vrchat;
pub mod xsoverlay;

/// Prelude module, prefixed with `Vrc` and `Xso` to avoid identifier conflict.
pub mod prelude {
    pub use crate::vrchat::{
        config::{
            Configuration as VrcConfiguration, Parameter as VrcParameter,
            ParameterAddress as VrcParameterAddress, ParameterDataType as VrcParameterDataType,
        },
        dir::VrcDirs,
    };

    pub use crate::xsoverlay::{
        NotificationAudio as XsoNotificationAudio, NotificationBuilder as XsoNotificationBuilder,
        NotificationDescription as XsoNotificationDescription,
        NotificationIcon as XsoNotificationIcon, RawNotification as XsoRawNotification,
    };
}
