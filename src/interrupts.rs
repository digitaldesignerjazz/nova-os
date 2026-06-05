/// Interrupt handling for Nova OS (early stage)

use core::panic::PanicInfo;

/// Basic page fault handler
///
/// This will be called when a page fault occurs.
/// For now it just prints information and halts.
pub fn page_fault_handler(
    faulting_address: usize,
    error_code: u64,
) -> ! {
    println!("\n!!! PAGE FAULT !!!");
    println!("  Faulting address: {:#x}", faulting_address);
    println!("  Error code: {:#x}", error_code);
    println!("  Kernel halting...");

    loop {}
}

// TODO:
// - Register this handler in the IDT (Interrupt Descriptor Table)
// - Handle different types of page faults (present, write, user, etc.)
// - Possibly kill the offending task instead of halting
