/// Memory Management for Nova OS

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn containing_address(address: usize) -> Self {
        Frame { number: address / PAGE_SIZE }
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    pub fn number(&self) -> usize {
        self.number
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub struct BitmapFrameAllocator {
    bitmap: &'static mut [u8],
    frame_count: usize,
    next_free: usize,
}

impl BitmapFrameAllocator {
    pub unsafe fn new(bitmap: &'static mut [u8], frame_count: usize) -> Self {
        for byte in bitmap.iter_mut() {
            *byte = 0;
        }

        BitmapFrameAllocator {
            bitmap,
            frame_count,
            next_free: 0,
        }
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    /// Check if a specific frame is currently free
    pub fn is_frame_free(&self, frame_number: usize) -> bool {
        if frame_number >= self.frame_count {
            return false;
        }
        let byte_index = frame_number / 8;
        let bit_index = frame_number % 8;
        (self.bitmap[byte_index] & (1 << bit_index)) == 0
    }

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
                self.bitmap[byte_index] |= 1 << bit_index;
                self.next_free = i + 1;
                return Some(Frame { number: i });
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        let frame_number = frame.number();
        if frame_number >= self.frame_count {
            return;
        }

        let byte_index = frame_number / 8;
        let bit_index = frame_number % 8;
        self.bitmap[byte_index] &= !(1 << bit_index);

        if frame_number < self.next_free {
            self.next_free = frame_number;
        }
    }
}

// TODO: Consider adding a more efficient way to find contiguous regions (e.g. using bit operations)
