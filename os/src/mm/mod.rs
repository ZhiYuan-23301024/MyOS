mod heap_allocator;
mod page_table;
mod address;
mod frame_allocator;

use address::{VPNRange, StepByOne};

pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};

pub use frame_allocator::{FrameTracker, frame_alloc};

pub use page_table::{PageTableEntry};

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}
