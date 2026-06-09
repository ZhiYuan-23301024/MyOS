mod heap_allocator;
mod page_table;

use page_table::{PTEFlags};

pub use page_table::{PageTableEntry};

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}

