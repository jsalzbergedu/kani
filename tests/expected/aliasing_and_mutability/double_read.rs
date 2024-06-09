// Created by Jacob Salzberg

#[kani::requires(*y < i32::MAX - *x)]
#[kani::ensures(result == *x + *y)]
fn reader(x: &i32, y: &i32) -> i32 {
    return *x + *y;
}

#[kani::proof_for_contract(reader)]
fn main() {
    let x = 10;
    let x_ptr = &x as *const i32;
    assert!(unsafe{reader(&*x_ptr, &*x_ptr)} == 20);
}
