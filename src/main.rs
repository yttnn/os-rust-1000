#![no_std]
#![no_main]

extern crate alloc;

use core::{arch::asm, ptr::write_bytes};
use alloc::boxed::Box;
use os_rust_1000::paging::Page;
use os_rust_1000::println;
use os_rust_1000::process;
use os_rust_1000::trap;

extern "C" {
  static __stack_top: u8;
  static mut __bss: u8;
  static __bss_end: u8;
}

unsafe fn proc_a_entry() {
  println!("starting process A");
  loop {
    println!("A");
    process::yield_process();

    for _ in 0..30000000 {
      asm!("nop");
    }
  }
}

unsafe fn proc_b_entry() {
  println!("starting process B");
  loop {
    println!("B");
    process::yield_process();

    for _ in 0..30000000 {
      asm!("nop");
    }
  }
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

  unsafe {
    asm!("csrw stvec, {}", in(reg) trap::kernel_entry);
    process::init();
    process::create_process(proc_a_entry as u32).expect("Faild To Create Process");
    process::create_process(proc_b_entry as u32).expect("Faild To Create Process");
    process::yield_process();
  }

  // let x = Box::new(Page::new());
  // let y = Box::new(Page::new());
  // let z = Box::new(Page::new());
  // println!("raw: {:?}", Box::into_raw(x));
  // println!("raw: {:?}", Box::into_raw(y));
  // println!("raw: {:?}", Box::into_raw(z));

  loop {}
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}
