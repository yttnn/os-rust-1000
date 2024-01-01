use core::arch::asm;

use crate::process::Context;

#[no_mangle]
pub unsafe extern "C" fn switch_context(prev: &mut Context, next: &Context) {
  asm!(
    "sw ra,  0  * 4(sp)",
    "sw sp,  1  * 4(sp)",
    "sw s0,  2  * 4(sp)",
    "sw s1,  3  * 4(sp)",
    "sw s2,  4  * 4(sp)",
    "sw s3,  5  * 4(sp)",
    "sw s4,  6  * 4(sp)",
    "sw s5,  7  * 4(sp)",
    "sw s6,  8  * 4(sp)",
    "sw s7,  9  * 4(sp)",
    "sw s8,  10 * 4(sp)",
    "sw s9,  11 * 4(sp)",
    "sw s10, 12 * 4(sp)",
    "sw s11, 13 * 4(sp)",
    "lw ra,  0  * 4(sp)",
    "lw sp,  1  * 4(sp)",
    "lw s0,  2  * 4(sp)",
    "lw s1,  3  * 4(sp)",
    "lw s2,  4  * 4(sp)",
    "lw s3,  5  * 4(sp)",
    "lw s4,  6  * 4(sp)",
    "lw s5,  7  * 4(sp)",
    "lw s6,  8  * 4(sp)",
    "lw s7,  9  * 4(sp)",
    "lw s8,  10  * 4(sp)",
    "lw s9,  11 * 4(sp)",
    "lw s10, 12 * 4(sp)",
    "lw s11, 13 * 4(sp)",
    "ret",
    options(noreturn),
  );
}