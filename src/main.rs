#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Entry point for the Nova OS kernel.
/// This is called by the bootloader.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // TODO (Phase 1):
    // - Initialize serial output or VGA
    // - Set up memory management
    // - Initialize interrupt handlers
    // - Start self-improving scheduler
    // - Launch emotional swarm runtime

    // For now: simple infinite loop (kernel is alive)
    loop {
        // In real implementation we would output "Nova OS kernel booted!"
        // via serial or framebuffer here.
    }
}

/// Panic handler required for no_std environment
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
