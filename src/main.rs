#![no_std]
#![no_main]


#[no_mangle]
#[link_section = ".text.boot"]
pub extern  "C" fn boot() -> ! {
  loop {}
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
