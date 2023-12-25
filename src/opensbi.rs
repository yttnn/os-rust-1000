use core::arch::asm;

#[allow(dead_code)]
pub struct Sbiret {
  error: i32,
  value: i32,
}

#[allow(dead_code, unused_variables, unused_assignments)]
pub fn sbi_call(mut arg0: i32, mut arg1: i32, mut arg2: i32, mut arg3: i32,
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