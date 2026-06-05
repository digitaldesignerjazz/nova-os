#![no_std]
#![no_main]

use core::fmt::{self, Write};
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

/// Write a string to serial
fn serial_print(args: fmt::Arguments) {
    unsafe {
        if let Some(ref mut serial) = SERIAL {
            let _ = serial.write_fmt(args);
        }
    }
}

/// println! macro for kernel
#[macro_export]
macro_rules! println {
    () => (serial_print(format_args!("\n")));
    ($($arg:tt)*) => (serial_print(format_args!("{}\n", format_args!($($arg)*))));
}

/// print! macro for kernel
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (serial_print(format_args!($($arg)*)));
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_serial();

    println!();
    println!("========================================");
    println!("  Nova OS Kernel - Phase 1 Booted!");
    println!("  Emotional Swarm-Based Operating System");
    println!("  Esslinger & Co. | v10.0 Aligned");
    println!("========================================");
    println!();
    println!("Serial output + println! macro working.");
    println!("Next steps: Bootloader integration + Memory management");
    println!();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    println!("{:?}", info);
    loop {}
}
