mod lib;
use crate::lib::lib_test;

use bindings::Windows::Win32::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};

fn main() {
  let a = "Test";
  let b: &str = "haha";
  lib_test();
  drop(a);

  println!("{}", triangle_area(2, 6, 7, 4, 12, 65));

  show();
}

// Tính diện tích
fn triangle_area(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> i32 {
  let first = x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2);
  first / 2
}

fn show() {
  unsafe {
    MessageBoxA(None, "Hello", "Test", MESSAGEBOX_STYLE::MB_OK);
  }
}