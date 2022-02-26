//! Helpers for resolving VRChat's config file.

use directories::BaseDirs;

pub struct VrcDirs(BaseDirs);

impl VrcDirs {
    /// Creates `VrchatDirs`.
    /// Returns `None` if failed to fetching base directories.
    pub fn new() -> Option<VrcDirs> {
        BaseDirs::new().map(VrcDirs)
    }

    /// Returns OSC configuration directory of VRChat.
    /// e.g. `%PROFILE%\AppData\LocalLow\VRChat\VRChat\OSC`
    pub fn osc_dir(&self) -> String {
        self.0
            .data_local_dir()
            .join("../LocalLow/VRChat/VRChat/OSC")
            .to_string_lossy()
            .to_string()
    }
}
