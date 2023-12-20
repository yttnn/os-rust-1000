#![no_std]
#![no_main]


#[no_mangle]
#[link_section = ".text.boot"]
pub extern  "C" fn boot() -> ! {
  loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
