#[path = "./opensbi.rs"]
mod opensbi;

pub fn putchar(ch: char) {
  opensbi::sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1);
}
