use core::fmt::{self, Write};

use crate::opensbi;

pub fn putchar(ch: char) {
  opensbi::sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1);
}

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
  () => ($crate::print!("\n"));
  ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
  let mut writer = SbiWriter {};
  writer.write_fmt(args).unwrap();
}

struct SbiWriter;

impl Write for SbiWriter {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for c in s.chars() {
      putchar(c);
    }
    Ok(())
  }
}