#![no_std]
#![no_main]

use core::{arch::asm, ptr::write_bytes};

extern "C" {
  static __stack_top: u8;
  static mut __bss: u8;
  static __bss_end: u8;
}

struct Sbiret {
  error: i32,
  value: i32,
}

fn sbi_call(mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32,
            mut arg4: i32, mut arg5: i32, mut fid: i32, mut eid: i32) -> Sbiret {
  unsafe {
    asm!(
      "ecall",
      inout("a0") arg0,
      inout("a1") arg1,
      out("a2") arg2,
      out("a3") arg3,
      out("a4") arg4,
      out("a5") arg5,
      out("a6") fid,
      out("a7") eid,
    );
  }

  Sbiret{ error: arg0, value: arg1 }
}

fn putchar(ch: char) {
  sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1);
}

#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern  "C" fn boot() -> ! {
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

  let s = "\nHello World!!\n";
  for i in s.chars() {
    putchar(i);
  }

  loop {}
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
