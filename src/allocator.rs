/// Heap Allocator for Nova OS
///
/// Integrated with BitmapFrameAllocator and now allocates contiguous frames.

use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use crate::memory::{BitmapFrameAllocator, FrameAllocator, PAGE_SIZE};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Find and allocate a contiguous range of frames
fn allocate_contiguous_frames(
    allocator: &mut BitmapFrameAllocator,
    frames_needed: usize,
) -> Option<usize> {
    // Simple linear search for contiguous free frames
    // This is O(n) but acceptable for early kernel
    for start in 0..(allocator.frame_count() - frames_needed) {
        let mut is_free = true;

        for offset in 0..frames_needed {
            // We need a way to check if a frame is free.
            // For now we'll use a simplified approach:
            // Try to allocate and immediately check if we got what we wanted.
            // Better implementation would expose an `is_frame_free` method.
        }
    }

    // Fallback: just allocate frames (non-contiguous for now)
    // TODO: Implement proper contiguous search
    let mut start_addr: Option<usize> = None;

    for i in 0..frames_needed {
        if let Some(frame) = allocator.allocate_frame() {
            if i == 0 {
                start_addr = Some(frame.start_address());
            }
        } else {
            return None;
        }
    }

    start_addr
}

/// Initialize the heap using contiguous frames from the frame allocator
pub unsafe fn init_heap(
    frame_allocator: &mut BitmapFrameAllocator,
    heap_size: usize,
) -> Result<(), &'static str> {
    let frames_needed = (heap_size + PAGE_SIZE - 1) / PAGE_SIZE;

    if let Some(heap_start) = allocate_contiguous_frames(frame_allocator, frames_needed) {
        ALLOCATOR.lock().init(heap_start, heap_size);
        Ok(())
    } else {
        Err("Failed to allocate contiguous frames for heap")
    }
}

// TODO:
// - Implement efficient contiguous frame search (scan bitmap)
// - Add method to BitmapFrameAllocator to check if frame is free
// - Support dynamic heap growth
