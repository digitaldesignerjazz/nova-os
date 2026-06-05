/// Virtual Memory / Paging for Nova OS

use crate::memory::{Frame, FrameAllocator, PAGE_SIZE};

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

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn is_unused(&self) -> bool { self.0 == 0 }

    pub fn set_frame(&mut self, frame: Frame, flags: u64) {
        self.0 = (frame.start_address() as u64) | flags;
    }

    pub fn frame(&self) -> Option<Frame> {
        if self.is_unused() { None } else {
            Some(Frame::containing_address((self.0 & 0x000f_ffff_ffff_f000) as usize))
        }
    }
}

/// A single page table (512 entries)
#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = PageTableEntry(0);
        }
    }

    pub fn set_entry(&mut self, index: usize, entry: PageTableEntry) {
        self.entries[index] = entry;
    }
}

/// Basic 4-level paging manager (early implementation)
pub struct PagingManager {
    // In a real kernel we would store the current PML4 physical address here
}

impl PagingManager {
    pub fn new() -> Self {
        PagingManager {}
    }

    /// Map a single page (simplified - does not yet create intermediate tables)
    pub fn map_page<A: FrameAllocator>(
        &mut self,
        page: Page,
        frame: Frame,
        flags: u64,
        _allocator: &mut A,
    ) -> Result<(), &'static str> {
        // TODO: Walk the 4-level page tables and create missing ones
        // For now we just print what we would do
        println!(
            "[PAGING] Mapping virtual page {} -> physical frame {}",
            page.number, frame.number()
        );
        Ok(())
    }
}

// TODO (Real implementation needed):
// - Add function to get/create Page Table at each level (PML4, PDPT, PD, PT)
// - Allocate frames for new page tables using the frame allocator
// - Implement proper page table walking
// - Add support for huge pages (2MiB / 1GiB)
