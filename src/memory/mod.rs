mod area_frame_allocator;
mod paging;

use self::paging::PhysicalAddress;

pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::test_paging;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

pub const PAGE_SIZE: usize = 4096;

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
    
    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
