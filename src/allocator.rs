/// Heap Allocator for Nova OS
///
/// Uses linked_list_allocator on top of our frame allocator.

use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize the heap allocator
///
/// This should be called after the frame allocator is ready.
pub unsafe fn init_heap(heap_start: usize, heap_size: usize) {
    ALLOCATOR.lock().init(heap_start, heap_size);
    // Note: In a real implementation we would allocate frames from
    // the frame allocator to back this heap region.
}

// TODO:
// - Integrate properly with BitmapFrameAllocator to allocate backing frames
// - Add better error handling
// - Support growing the heap dynamically
