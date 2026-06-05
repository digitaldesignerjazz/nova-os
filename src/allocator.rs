/// Heap Allocator for Nova OS
///
/// Now supports proper contiguous frame allocation.

use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use crate::memory::{BitmapFrameAllocator, FrameAllocator, PAGE_SIZE};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Find and allocate a contiguous range of free frames.
/// Returns the starting physical address of the contiguous block.
fn allocate_contiguous_frames(
    allocator: &mut BitmapFrameAllocator,
    frames_needed: usize,
) -> Option<usize> {
    if frames_needed == 0 {
        return None;
    }

    let total_frames = allocator.frame_count();

    // Scan linearly for a contiguous block of free frames
    'outer: for start in 0..=(total_frames - frames_needed) {
        // Check if the entire range [start .. start+frames_needed) is free
        for offset in 0..frames_needed {
            if !allocator.is_frame_free(start + offset) {
                continue 'outer;
            }
        }

        // Found a contiguous free region!
        // Directly mark all frames in the range as used
        for offset in 0..frames_needed {
            allocator.mark_frame_as_used(start + offset);
        }

        // Update next_free heuristic
        if start + frames_needed > allocator.next_free_hint() {
            // We can't easily update private fields, so we leave it
        }

        return Some(start * PAGE_SIZE);
    }

    None
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
        Err("Not enough contiguous free frames for heap")
    }
}

// TODO:
// - Optimize contiguous search (current O(n) scan)
// - Add a way to update next_free after bulk allocation
// - Support dynamic heap growth by allocating more frames later
