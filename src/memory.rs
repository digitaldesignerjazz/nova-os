/// Memory Management for Nova OS
///
/// This module provides physical memory frame allocation.
/// Currently implements a simple BitmapFrameAllocator.

pub const PAGE_SIZE: usize = 4096;

/// Represents a physical memory frame (4 KiB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    /// Create a frame that contains the given physical address
    pub fn containing_address(address: usize) -> Self {
        Frame {
            number: address / PAGE_SIZE,
        }
    }

    /// Returns the starting physical address of this frame
    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    /// Returns the frame number
    pub fn number(&self) -> usize {
        self.number
    }
}

/// Trait for frame allocators
pub trait FrameAllocator {
    /// Allocate a free physical frame
    fn allocate_frame(&mut self) -> Option<Frame>;

    /// Deallocate (free) a physical frame
    fn deallocate_frame(&mut self, frame: Frame);
}

/// A simple bitmap-based frame allocator
///
/// This is a basic but functional implementation suitable for early kernel development.
/// It uses a bitmap where each bit represents one frame.
pub struct BitmapFrameAllocator {
    bitmap: &'static mut [u8],
    frame_count: usize,
    next_free: usize, // optimization: start searching from here
}

impl BitmapFrameAllocator {
    /// Create a new bitmap frame allocator
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `bitmap` points to valid memory that can be used as bitmap
    /// - `frame_count` correctly represents the number of frames to manage
    pub unsafe fn new(bitmap: &'static mut [u8], frame_count: usize) -> Self {
        // Clear the bitmap (mark all frames as free)
        for byte in bitmap.iter_mut() {
            *byte = 0;
        }

        BitmapFrameAllocator {
            bitmap,
            frame_count,
            next_free: 0,
        }
    }

    /// Mark a specific frame as used (e.g. for kernel code, BIOS regions, etc.)
    pub fn mark_frame_as_used(&mut self, frame_number: usize) {
        if frame_number >= self.frame_count {
            return;
        }
        let byte_index = frame_number / 8;
        let bit_index = frame_number % 8;
        self.bitmap[byte_index] |= 1 << bit_index;
    }
}

impl FrameAllocator for BitmapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        for i in self.next_free..self.frame_count {
            let byte_index = i / 8;
            let bit_index = i % 8;

            if (self.bitmap[byte_index] & (1 << bit_index)) == 0 {
                // Found a free frame
                self.bitmap[byte_index] |= 1 << bit_index; // mark as used
                self.next_free = i + 1;
                return Some(Frame { number: i });
            }
        }

        // No free frames found
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        let frame_number = frame.number();
        if frame_number >= self.frame_count {
            return;
        }

        let byte_index = frame_number / 8;
        let bit_index = frame_number % 8;

        // Clear the bit (mark as free)
        self.bitmap[byte_index] &= !(1 << bit_index);

        // Update next_free for better performance
        if frame_number < self.next_free {
            self.next_free = frame_number;
        }
    }
}

// TODO (Next steps):
// - Integrate with actual memory map from bootloader (multiboot2 / UEFI)
// - Add support for reserved regions (kernel, BIOS, ACPI, etc.)
// - Replace with a more sophisticated allocator when needed
// - Add virtual memory / paging support on top of frame allocator
