#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uart_16550::SerialPort;

/// Global serial port for early kernel output (COM1)
static mut SERIAL: Option<SerialPort> = None;

/// Initialize the serial port (COM1 at 0x3F8)
fn init_serial() {
    unsafe {
        let mut serial = SerialPort::new(0x3F8);
        serial.init();
        SERIAL = Some(serial);
    }
}

/// Print a string to the serial port
fn serial_print(msg: &str) {
    unsafe {
        if let Some(ref mut serial) = SERIAL {
            for byte in msg.bytes() {
                serial.send(byte);
            }
        }
    }
}

/// Kernel entry point called by the bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_serial();

    serial_print("\n");
    serial_print("========================================\n");
    serial_print("  Nova OS Kernel - Phase 1 Booted!\n");
    serial_print("  Emotional Swarm-Based Operating System\n");
    serial_print("  Esslinger & Co. | v10.0 Aligned\n");
    serial_print("========================================\n\n");
    serial_print("Serial output initialized successfully.\n");
    serial_print("Next: Memory management + Self-improving scheduler\n\n");

    // Kernel main loop (will be replaced by scheduler later)
    loop {}
}

/// Panic handler for no_std environment
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_print("\n!!! KERNEL PANIC !!!\n");
    loop {}
}
