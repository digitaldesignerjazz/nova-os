#![no_std]
#![no_main]

use core::fmt::{self, Write};
use core::panic::PanicInfo;
use uart_16550::SerialPort;

mod memory;
use memory::{BitmapFrameAllocator, FrameAllocator, Frame, PAGE_SIZE};

/// Global serial port
static mut SERIAL: Option<SerialPort> = None;

fn init_serial() {
    unsafe {
        let mut serial = SerialPort::new(0x3F8);
        serial.init();
        SERIAL = Some(serial);
    }
}

fn serial_print(args: fmt::Arguments) {
    unsafe {
        if let Some(ref mut serial) = SERIAL {
            let _ = serial.write_fmt(args);
        }
    }
}

#[macro_export]
macro_rules! println {
    () => (serial_print(format_args!("\n")));
    ($($arg:tt)*) => (serial_print(format_args!("{}\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (serial_print(format_args!($($arg)*)));
}

/// Static bitmap for frame allocator (supports up to ~128 MiB of RAM)
static mut FRAME_BITMAP: [u8; 16 * 1024] = [0; 16 * 1024];

/// Initialize the frame allocator with a stub memory map
fn init_frame_allocator() -> BitmapFrameAllocator {
    unsafe {
        // For now we use a hardcoded memory region (16 MiB - 128 MiB)
        // This will be replaced with real memory map from bootloader later
        let memory_start = 0x0100_0000; // 16 MiB
        let memory_end   = 0x0800_0000; // 128 MiB

        let frame_count = (memory_end - memory_start) / PAGE_SIZE;

        let mut allocator = BitmapFrameAllocator::new(&mut FRAME_BITMAP, frame_count);

        // Mark the first few frames as used (kernel area, etc.)
        // In a real system we would get this from the bootloader memory map
        for i in 0..64 {
            allocator.mark_frame_as_used(i);
        }

        println!("Frame allocator initialized.");
        println!("  Managing {} frames ({} MiB)", frame_count, (frame_count * PAGE_SIZE) / (1024 * 1024));

        allocator
    }
}

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

    // Initialize frame allocator
    let mut frame_allocator = init_frame_allocator();

    // Test frame allocation
    println!("Testing frame allocation...");
    if let Some(frame) = frame_allocator.allocate_frame() {
        println!("  Allocated frame #{}", frame.number());
    }
    if let Some(frame) = frame_allocator.allocate_frame() {
        println!("  Allocated frame #{}", frame.number());
    }

    println!();
    println!("Frame allocator working!");
    println!("Next: Heap allocator + Virtual memory");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    println!("{:?}", info);
    loop {}
}
