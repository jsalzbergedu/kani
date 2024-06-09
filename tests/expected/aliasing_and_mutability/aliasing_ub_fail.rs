use std::ptr::addr_of_mut;
// Created by Jacob Salzberg
// Requires -Zfunction-contracts
static mut MY_POINTER: Option<*mut i32> = None;

#[kani::modifies(addr_of_mut!(MY_POINTER))]
#[kani::requires(unsafe {MY_POINTER == None})]
#[kani::ensures(unsafe {MY_POINTER.is_some()})]
fn reference_dies() {
  let mut x: i32 = 2;
  unsafe {
    MY_POINTER = Some(&mut x as *mut i32);
    println!("use to ensure that compiler doesn't optimize to Some(0), {}", &mut x);
  }
}

#[kani::proof_for_contract(reference_dies)]
fn main() {
  reference_dies();
  unsafe {
    match MY_POINTER {
        None => { () },
        Some(x) => { let _y = *x; },
    }
  }
  assert!(true);
}
