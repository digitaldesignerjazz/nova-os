/// Heap Allocator for Nova OS
///
/// Now supports proper contiguous frame allocation.

use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use crate::memory::{BitmapFrameAllocator, FrameAllocator, PAGE_SIZE};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Find and allocate a contiguous range of free frames
fn allocate_contiguous_frames(
    allocator: &mut BitmapFrameAllocator,
    frames_needed: usize,
) -> Option<usize> {
    if frames_needed == 0 {
        return None;
    }

    let total_frames = allocator.frame_count();

    // Scan for a contiguous block of free frames
    'outer: for start in 0..=(total_frames - frames_needed) {
        // Check if all frames in [start, start + frames_needed) are free
        for offset in 0..frames_needed {
            if !allocator.is_frame_free(start + offset) {
                continue 'outer;
            }
        }

        // Found a contiguous free region! Allocate all frames in it
        let heap_start = start * PAGE_SIZE;  // We need the virtual/physical address

        for offset in 0..frames_needed {
            // Allocate each frame (this also marks them as used)
            if allocator.allocate_frame().is_none() {
                // Should not happen if is_frame_free was correct
                return None;
            }
        }

        return Some(heap_start);
    }

    None
}

/// Initialize the heap using contiguous frames
pub unsafe fn init_heap(
    frame_allocator: &mut BitmapFrameAllocator,
    heap_size: usize,
) -> Result<(), &'static str> {
    let frames_needed = (heap_size + PAGE_SIZE - 1) / PAGE_SIZE;

    if let Some(heap_start) = allocate_contiguous_frames(frame_allocator, frames_needed) {
        ALLOCATOR.lock().init(heap_start, heap_size);
        Ok(())
    } else {
        Err("Not enough contiguous free frames for heap")
    }
}

// TODO:
// - Optimize contiguous search (current is O(n))
// - Add support for dynamic heap growth
// - Consider using a better data structure for free regions
