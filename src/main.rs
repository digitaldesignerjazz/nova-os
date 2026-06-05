#![no_std]
#![no_main]

use core::fmt::{self, Write};
use core::panic::PanicInfo;
use uart_16550::SerialPort;

mod memory;
mod allocator;
mod paging;
mod interrupts;
mod scheduler;

use memory::{BitmapFrameAllocator, FrameAllocator, Frame, PAGE_SIZE};
use allocator::init_heap;
use paging::{PagingManager, Page, PageTable, PageTableEntry};
use interrupts::init_idt;
use scheduler::{SelfImprovingScheduler, EmotionalState};

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

    let mut frame_allocator = init_frame_allocator();
    init_idt();

    unsafe {
        let heap_size = 1024 * 1024;
        let _ = init_heap(&mut frame_allocator, heap_size);
    }

    // === Emotional Scheduler + Propagation Demo ===
    println!("\n=== Emotional Runtime + Propagation Demo ===");

    let scheduler = SelfImprovingScheduler::new();

    let mut tasks = [
        scheduler.create_task(10),
        scheduler.create_task(8),
        scheduler.create_task(12),
        scheduler.create_task(9),
    ];

    // Evolve some emotional states through feedback
    scheduler.collect_feedback(&mut tasks[0], true, 20);   // -> Focused
    scheduler.collect_feedback(&mut tasks[2], true, 180);  // -> Curious

    println!("\nInitial emotional states:");
    for task in &tasks {
        println!("  Task {}: {:?}", task.id, task.emotional_state);
    }

    // Propagate emotion from the Focused task to others
    scheduler.propagate_emotion(&tasks[0], &mut tasks);

    println!("\nAfter emotional propagation from Task 0 (Focused):");
    for task in &tasks {
        println!("  Task {}: {:?}", task.id, task.emotional_state);
    }

    scheduler.schedule(&mut tasks);

    println!("\nFinal scheduled order (emotional state considered):");
    for task in &tasks {
        println!("  Task {}: priority={}, state={:?}", task.id, task.priority, task.emotional_state);
    }

    println!("\nEmotional propagation demo complete.");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    println!("{:?}", info);
    loop {}
}
