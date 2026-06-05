/// Virtual Memory / Paging for Nova OS
///
/// This will become the virtual memory subsystem.
/// It sits on top of the physical frame allocator.

use crate::memory::{Frame, FrameAllocator};

/// Page size (4 KiB on x86_64)
pub const PAGE_SIZE: usize = 4096;

/// Represents a virtual page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn containing_address(address: usize) -> Self {
        Page { number: address / PAGE_SIZE }
    }
}

/// Page table entry flags (simplified)
#[derive(Debug, Clone, Copy)]
pub struct PageTableFlags {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    // Add more flags as needed (no_execute, etc.)
}

/// Very early paging manager skeleton
pub struct PagingManager {
    // In a real implementation this would hold:
    // - Pointer to current page table (CR3 on x86_64)
    // - Frame allocator reference
}

impl PagingManager {
    pub fn new() -> Self {
        PagingManager {}
    }

    /// Map a virtual page to a physical frame
    pub fn map_page<A: FrameAllocator>(
        &mut self,
        page: Page,
        frame: Frame,
        flags: PageTableFlags,
        frame_allocator: &mut A,
    ) -> Result<(), &'static str> {
        // TODO: Actual page table manipulation
        println!("TODO: Map virtual page {} to frame {}", page.number, frame.number());
        Ok(())
    }

    /// Unmap a virtual page
    pub fn unmap_page(&mut self, page: Page) -> Result<Frame, &'static str> {
        // TODO
        println!("TODO: Unmap virtual page {}", page.number);
        Err("not implemented")
    }
}

// TODO (Major next milestone):
// - Implement actual 4-level page tables for x86_64
// - Add page fault handler
// - Support identity mapping + higher-half kernel
// - Integrate with frame allocator for page table allocation
