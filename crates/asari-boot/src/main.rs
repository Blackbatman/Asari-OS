use asari_platform::{Platform, PlatformId, PlatformCapabilities, PlatformError, FramebufferInfo};

struct QemuPlatform;

impl Platform for QemuPlatform {
    fn id(&self) -> PlatformId {
        PlatformId::Qemu
    }

    fn name(&self) -> &'static str {
        "QEMU (primary development target)"
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            can_kexec: false, // not yet implemented
            has_framebuffer: true,
            has_network: true,
            has_storage: true,
            biotic_ready: true,
        }
    }

    fn contract_version(&self) -> u32 {
        1
    }

    fn prepare_linux_kexec(&self) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented)
    }

    fn execute_kexec(&self) -> Result<(), PlatformError> {
        Err(PlatformError::NotImplemented)
    }

    fn framebuffer_info(&self) -> Option<FramebufferInfo> {
        Some(FramebufferInfo {
            width: 1920,
            height: 1080,
            stride: 1920 * 4,
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let platform = QemuPlatform;

    println!("Asari-OS boot stub");
    println!("Platform: {}", platform.name());
    println!("Contract version: {}", platform.contract_version());

    let caps = platform.capabilities();
    println!("Capabilities:");
    println!("  framebuffer: {}", caps.has_framebuffer);
    println!("  network:     {}", caps.has_network);
    println!("  storage:     {}", caps.has_storage);
    println!("  biotic_ready:{}", caps.biotic_ready);

    if args.iter().any(|a| a == "--smoke") {
        println!("Smoke test: OK");
    }

    println!("Ready for neural pathways to synchronize.");
}
