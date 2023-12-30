#![no_std]
#![no_main]

extern crate alloc;

use core::{arch::asm, ptr::write_bytes};
mod print;
mod trap;
mod allocator;

extern "C" {
  static __stack_top: u8;
  static mut __bss: u8;
  static __bss_end: u8;
}


#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn boot() -> ! {
  asm!(
    "mv sp, {stack_top}",
    "j {kernel_main}",
    stack_top = in(reg) &__stack_top,
    kernel_main = sym kernel_main,
  );
  loop {}
}

#[no_mangle]
fn kernel_main() -> ! {
  unsafe {
    let addr_bss_start = &__bss as *const u8;
    let addr_bss_end = &__bss_end as *const u8;
    let bss_size = addr_bss_end as usize - addr_bss_start as usize;
    write_bytes(&mut __bss as *mut u8, 0, bss_size);
  }

  let mut v = alloc::vec::Vec::new();
  v.push(42);
  println!("{:p}", v.as_ptr());

  // unsafe {
  //   asm!("csrw stvec, {}", in(reg) trap::kernel_entry);
  //   asm!("unimp");
  // }

  loop {}
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}
