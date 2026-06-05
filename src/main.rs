#![no_std]
#![no_main]

use core::fmt::{self, Write};
use core::panic::PanicInfo;
use uart_16550::SerialPort;

mod memory;
mod allocator;
mod paging;
mod interrupts;

use memory::{BitmapFrameAllocator, FrameAllocator, Frame, PAGE_SIZE};
use allocator::init_heap;
use paging::{PagingManager, Page, PageTable, PageTableEntry};
use interrupts::init_idt;

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

static mut FRAME_BITMAP: [u8; 16 * 1024] = [0; 16 * 1024];

fn init_frame_allocator() -> BitmapFrameAllocator {
    unsafe {
        let memory_start = 0x0100_0000;
        let memory_end   = 0x0800_0000;
        let frame_count = (memory_end - memory_start) / PAGE_SIZE;

        let mut allocator = BitmapFrameAllocator::new(&mut FRAME_BITMAP, frame_count);

        for i in 0..64 {
            allocator.mark_frame_as_used(i);
        }

        println!("Frame allocator initialized.");
        println!("  Managing {} frames (~{} MiB)", frame_count, (frame_count * PAGE_SIZE) / (1024 * 1024));

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

    // === Frame Allocator ===
    let mut frame_allocator = init_frame_allocator();

    // === IDT (Page Fault Handler) ===
    init_idt();

    // === Heap ===
    unsafe {
        let heap_size = 1024 * 1024;
        if init_heap(&mut frame_allocator, heap_size).is_ok() {
            println!("Heap initialized successfully");
        }
    }

    // === Test Paging ===
    println!("\nTesting 4-level page table mapping...");

    // Create a page table (we'll use a simple one for testing)
    // In a real kernel we would have a proper PML4
    static mut TEST_PAGE_TABLE: PageTable = PageTable { entries: [PageTableEntry(0); 512] };

    unsafe {
        TEST_PAGE_TABLE.zero();

        let virtual_page = Page::containing_address(0x4000_0000); // Example virtual address
        if let Some(frame) = frame_allocator.allocate_frame() {
            let flags = PageTableEntry::PRESENT | PageTableEntry::WRITABLE;

            match PagingManager::map_page(
                &mut TEST_PAGE_TABLE,
                virtual_page,
                frame,
                flags,
                &mut frame_allocator,
            ) {
                Ok(()) => println!("  Successfully mapped virtual page to frame {}", frame.number()),
                Err(e) => println!("  Mapping failed: {}", e),
            }
        }
    }

    println!("\nPaging test complete!");
    println!("Next: Full bootloader integration + higher-half kernel");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    println!("{:?}", info);
    loop {}
}
