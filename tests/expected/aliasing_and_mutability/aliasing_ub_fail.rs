// Created by Jacob Salzberg
static mut MY_POINTER: Option<*mut i32> = None;

fn reference_dies() {
  let mut x: i32 = 2;
  unsafe {
    MY_POINTER = Some(&mut x as *mut i32);
    println!("use to ensure that compiler doesn't optimize to Some(0), {}", &mut x);
  }
}

#[kani::proof]
fn main() {
  reference_dies();
  unsafe {
    match MY_POINTER {
        None => { unreachable!() },
        Some(x) => { let _y = *x; },
    }
  }
  assert!(true);
}
