/// Virtual Memory / Paging for Nova OS
/// Now includes real 4-level page table walking.

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

    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER_ACCESSIBLE: u64 = 1 << 2;
}

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
}

/// 4-level page table indices (x86_64)
pub fn pml4_index(addr: usize) -> usize { (addr >> 39) & 0x1ff }
pub fn pdpt_index(addr: usize) -> usize { (addr >> 30) & 0x1ff }
pub fn pd_index(addr: usize)   -> usize { (addr >> 21) & 0x1ff }
pub fn pt_index(addr: usize)   -> usize { (addr >> 12) & 0x1ff }

/// Real page table walking + mapping
pub struct PagingManager;

impl PagingManager {
    /// Map a virtual page to a physical frame (creates page tables as needed)
    pub fn map_page<A: FrameAllocator>(
        page_table: &mut PageTable, // Usually the PML4
        page: Page,
        frame: Frame,
        flags: u64,
        allocator: &mut A,
    ) -> Result<(), &'static str> {
        let addr = page.start_address();

        // Walk the 4 levels
        let pml4 = page_table;
        let pdpt = Self::get_or_create_table(&mut pml4.entries[pml4_index(addr)], allocator)?;
        let pd   = Self::get_or_create_table(&mut pdpt.entries[pdpt_index(addr)], allocator)?;
        let pt   = Self::get_or_create_table(&mut pd.entries[pd_index(addr)], allocator)?;

        // Set the final entry
        pt.entries[pt_index(addr)].set_frame(frame, flags | PageTableEntry::PRESENT);

        Ok(())
    }

    /// Get existing page table or create a new one
    fn get_or_create_table<A: FrameAllocator>(
        entry: &mut PageTableEntry,
        allocator: &mut A,
    ) -> Result<&'static mut PageTable, &'static str> {
        if let Some(frame) = entry.frame() {
            // Already exists
            unsafe { return Ok(&mut *(frame.start_address() as *mut PageTable)); }
        }

        // Allocate a new frame for the page table
        let frame = allocator.allocate_frame()
            .ok_or("Out of memory for page table")?;

        let table = unsafe { &mut *(frame.start_address() as *mut PageTable) };
        table.zero();

        // Point the parent entry to the new table
        entry.set_frame(frame, PageTableEntry::PRESENT | PageTableEntry::WRITABLE);

        Ok(table)
    }
}

// TODO:
// - Add unmap support
// - Add support for huge pages
// - Flush TLB after mapping
// - Proper recursive page table mapping for easier access
