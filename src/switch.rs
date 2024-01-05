use core::arch::asm;

use crate::process::Context;

#[no_mangle]
#[naked]
#[allow(unused_variables)]
pub unsafe extern "C" fn switch_context(prev: &mut Context, next: &Context) {
  asm!(
    "sw ra,  0  * 4(a0)",
    "sw sp,  1  * 4(a0)",
    "sw s0,  2  * 4(a0)",
    "sw s1,  3  * 4(a0)",
    "sw s2,  4  * 4(a0)",
    "sw s3,  5  * 4(a0)",
    "sw s4,  6  * 4(a0)",
    "sw s5,  7  * 4(a0)",
    "sw s6,  8  * 4(a0)",
    "sw s7,  9  * 4(a0)",
    "sw s8,  10 * 4(a0)",
    "sw s9,  11 * 4(a0)",
    "sw s10, 12 * 4(a0)",
    "sw s11, 13 * 4(a0)",
    "lw ra,  0  * 4(a1)",
    "lw sp,  1  * 4(a1)",
    "lw s0,  2  * 4(a1)",
    "lw s1,  3  * 4(a1)",
    "lw s2,  4  * 4(a1)",
    "lw s3,  5  * 4(a1)",
    "lw s4,  6  * 4(a1)",
    "lw s5,  7  * 4(a1)",
    "lw s6,  8  * 4(a1)",
    "lw s7,  9  * 4(a1)",
    "lw s8,  10 * 4(a1)",
    "lw s9,  11 * 4(a1)",
    "lw s10, 12 * 4(a1)",
    "lw s11, 13 * 4(a1)",
    "ret",
    options(noreturn),
  );
}