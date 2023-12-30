use core::{alloc::{GlobalAlloc, Layout}, ptr, cell::UnsafeCell};

extern "C" {
  static __free_ram: u8;
  static __free_ram_end: u8;
}

struct BumpPointerAlloc {
  head: UnsafeCell<*const u8>,
  end: *const u8,
}

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
  head: UnsafeCell::new(unsafe {&__free_ram} as *const u8),
  end: unsafe {&__free_ram_end} as *const u8,
};

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let head = self.head.get();

    let align = layout.align();
    let res = *head as usize % align;
    let start = if res == 0 { *head } else { *head.add(align - res) };

    let next_head = start.add(layout.size());
    if next_head > self.end {
      ptr::null_mut()
    } else {
      *head = next_head;
      start as *mut u8
    }
  }

  unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}