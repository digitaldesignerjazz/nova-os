/// Virtual Memory / Paging for Nova OS

use crate::memory::{Frame, FrameAllocator, PAGE_SIZE};

/// A virtual memory page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn containing_address(address: usize) -> Self {
        Page { number: address / PAGE_SIZE }
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }
}

/// Page table entry (simplified x86_64)
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_frame(&mut self, frame: Frame, flags: u64) {
        self.0 = (frame.start_address() as u64) | flags;
    }

    pub fn frame(&self) -> Option<Frame> {
        if self.is_unused() {
            None
        } else {
            Some(Frame::containing_address((self.0 & 0x000f_ffff_ffff_f000) as usize))
        }
    }
}

/// Very basic 4-level page table structure (stub)
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = PageTableEntry(0);
        }
    }
}

/// Basic paging manager
pub struct PagingManager {
    // In real implementation: current CR3 + active page tables
}

impl PagingManager {
    pub fn new() -> Self {
        PagingManager {}
    }

    pub fn map_page<A: FrameAllocator>(
        &mut self,
        _page: Page,
        _frame: Frame,
        _flags: u64,
        _allocator: &mut A,
    ) -> Result<(), &'static str> {
        // TODO: Walk/create page tables and set entry
        println!("[PAGING] map_page called (not fully implemented yet)");
        Ok(())
    }
}

// TODO:
// - Implement full 4-level page table walking + creation
// - Add page fault handler
// - Support higher-half kernel mapping
// - Identity map low memory + kernel
