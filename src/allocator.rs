/// Heap Allocator for Nova OS
///
/// Now integrated with our BitmapFrameAllocator.

use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use crate::memory::{BitmapFrameAllocator, FrameAllocator, PAGE_SIZE};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize the heap by allocating frames from the frame allocator
pub unsafe fn init_heap(
    frame_allocator: &mut BitmapFrameAllocator,
    heap_size: usize, // in bytes
) -> Result<(), &'static str> {
    let frames_needed = (heap_size + PAGE_SIZE - 1) / PAGE_SIZE;

    // Allocate contiguous frames for the heap
    let mut heap_start: Option<usize> = None;
    let mut current_frame: Option<crate::memory::Frame> = None;

    for i in 0..frames_needed {
        if let Some(frame) = frame_allocator.allocate_frame() {
            let addr = frame.start_address();
            if i == 0 {
                heap_start = Some(addr);
            }
            // TODO: In a real system we would ensure frames are contiguous
            // For now we just take whatever frames we get
        } else {
            return Err("Not enough free frames for heap");
        }
    }

    if let Some(start) = heap_start {
        ALLOCATOR.lock().init(start, heap_size);
        Ok(())
    } else {
        Err("Failed to allocate heap frames")
    }
}

// TODO:
// - Ensure allocated frames are physically contiguous (important!)
// - Support growing the heap later by allocating more frames
// - Add better error handling and logging
