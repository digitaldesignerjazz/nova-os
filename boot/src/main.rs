#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use nova_os;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // This is where Nova OS would be initialized with real boot information
    // For now we just loop

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
