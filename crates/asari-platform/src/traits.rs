//! Asari-OS Platform Contract
//!
//! Every supported platform must implement this trait honestly.
//! Capabilities that are not yet proven must report false / None.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformId {
    Qemu,
    LinuxNative,
    Aarch64,
    Ps5Experimental,
}

#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    pub can_kexec: bool,
    pub has_framebuffer: bool,
    pub has_network: bool,
    pub has_storage: bool,
    pub biotic_ready: bool, // playful internal flag for "elegant composition" readiness
}

pub trait Platform {
    fn id(&self) -> PlatformId;
    fn name(&self) -> &'static str;
    fn capabilities(&self) -> PlatformCapabilities;
    fn contract_version(&self) -> u32;

    // Boot / execution
    fn prepare_linux_kexec(&self) -> Result<(), PlatformError>;
    fn execute_kexec(&self) -> Result<(), PlatformError>;

    // Display
    fn framebuffer_info(&self) -> Option<FramebufferInfo>;
}

#[derive(Debug)]
pub struct FramebufferInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

#[derive(Debug)]
pub enum PlatformError {
    NotSupported,
    NotImplemented,
    HardwareFailure(String),
}
