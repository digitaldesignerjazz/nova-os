/// Interrupt handling for Nova OS

use x86_64::structures::idt::{InterruptDescriptorTable, PageFaultErrorCode};
use x86_64::VirtAddr;

use crate::println;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

/// Initialize and load the Interrupt Descriptor Table
pub fn init_idt() {
    unsafe {
        IDT.page_fault.set_handler_fn(page_fault_handler);

        // Load the IDT
        IDT.load();
    }
    println!("IDT initialized with page fault handler");
}

/// Page fault handler
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    println!("\n!!! PAGE FAULT !!!");
    println!("  Instruction pointer: {:#x}", stack_frame.instruction_pointer.as_u64());
    println!("  Faulting address:    {:#x}", x86_64::registers::control::Cr2::read().as_u64());
    println!("Kernel halting...");

    loop {}
}

// TODO:
// - Add handlers for other exceptions (double fault, general protection, etc.)
// - Set up proper TSS and interrupt stack
// - Handle page faults by mapping memory or killing tasks
