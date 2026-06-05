/// Basic memory management for Nova OS (Phase 1)
///
/// This module will eventually contain:
/// - Frame allocator (physical memory)
/// - Virtual memory / paging
/// - Heap allocator

use core::sync::atomic::{AtomicUsize, Ordering};

/// Simple bump allocator for early kernel use (very basic)
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end: heap_start + heap_size,
            next: AtomicUsize::new(heap_start),
        }
    }

    pub fn allocate(&self, size: usize, align: usize) -> Option<*mut u8> {
        let mut current = self.next.load(Ordering::Relaxed);

        // Align the pointer
        current = (current + align - 1) & !(align - 1);

        let next = current + size;
        if next > self.heap_end {
            return None;
        }

        self.next.store(next, Ordering::Relaxed);
        Some(current as *mut u8)
    }
}

// TODO (Phase 1):
// - Replace BumpAllocator with a proper frame allocator
// - Add support for linked_list_allocator or buddy allocator for heap
// - Implement paging / virtual memory
