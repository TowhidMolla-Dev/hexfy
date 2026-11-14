/// An OS Hex runs on.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Os {
    Macos,
    Windows,
    Linux,
}

impl Os {
    /// The OS this build runs on.
    pub const CURRENT: Self = if cfg!(target_os = "macos") {
        Self::Macos
    } else if cfg!(target_os = "windows") {
        Self::Windows
    } else {
        Self::Linux
    };
}
