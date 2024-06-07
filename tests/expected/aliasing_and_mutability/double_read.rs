// Created by Jacob Salzberg

fn reader(x: &i32, y: &i32) -> i32 {
    return *x + *y;
}

#[kani::proof]
fn main() {
    let x = 10;
    let x_ptr = &x as *const i32;
    assert!(unsafe{reader(&*x_ptr, &*x_ptr)} == 20);
}
