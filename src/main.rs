#![no_std]
#![no_main]

extern crate alloc;

use core::{arch::asm, ptr::write_bytes};
use os_rust_1000::allocator;
use os_rust_1000::print::putchar;
use os_rust_1000::println;
use os_rust_1000::process::{Process, ProcessManager};
use os_rust_1000::trap;

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

  let procs = ProcessManager::new();

  unsafe {
    asm!("csrw stvec, {}", in(reg) trap::kernel_entry);
  }

  procs.run();

  loop {}
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}
