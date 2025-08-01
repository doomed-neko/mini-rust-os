use core::{alloc::GlobalAlloc, ptr::null_mut};

use super::align_up;

use crate::allocator::Locked;

#[derive(Default)]
pub struct BumpAllocator {
    next: usize,
    heap_start: usize,
    heap_end: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Creates a new empty bump allocator.
    pub const fn new() -> Self {
        BumpAllocator {
            next: 0,
            heap_start: 0,
            heap_end: 0,
            allocations: 0,
        }
    }

    /// Initializes the bump allocator with the given heap bounds.
    ///
    /// # Safety
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut bump = self.lock();
        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return null_mut(),
        };
        if alloc_end > bump.heap_end {
            return null_mut();
        }

        bump.next = alloc_end;
        bump.allocations += 1;

        alloc_start as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        let mut bump = self.lock();
        bump.allocations -= 1;
        if _layout.size() + _ptr as usize == bump.next {
            bump.next = _ptr as usize;
        }
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
