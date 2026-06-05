/// Heap Allocator for Nova OS
///
/// Supports proper contiguous physical frame allocation.

use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use crate::memory::{BitmapFrameAllocator, FrameAllocator, PAGE_SIZE};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Find a contiguous range of free frames and mark them as used.
/// Returns the starting physical address of the allocated block.
fn allocate_contiguous_frames(
    allocator: &mut BitmapFrameAllocator,
    frames_needed: usize,
) -> Option<usize> {
    if frames_needed == 0 {
        return None;
    }

    let total_frames = allocator.frame_count();

    // Linear scan for contiguous free frames
    'outer: for start in 0..=(total_frames - frames_needed) {
        for offset in 0..frames_needed {
            if !allocator.is_frame_free(start + offset) {
                continue 'outer;
            }
        }

        // Found contiguous free region - mark all frames as used
        for offset in 0..frames_needed {
            allocator.mark_frame_as_used(start + offset);
        }

        return Some(start * PAGE_SIZE);
    }

    None
}

/// Initialize the heap allocator with frames from the frame allocator
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
// - Optimize search (current is simple linear scan)
// - Track next_free after bulk allocation for better performance
// - Support growing the heap dynamically
