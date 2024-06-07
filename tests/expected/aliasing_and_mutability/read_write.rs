// Created by Jacob Salzberg

fn reader_writer(x: &mut i32, y: &i32) -> i32 {
    if *y < 100 {
        *x = 0;
    } else {
        *x = 1;
    }
    return *y * *x;
}
// The above SHOULD optimize to if *y < 100 0 else *y, but an unsafe write will ruin this.

#[kani::proof]
fn main() {
    let mut x = 1000;
    let x_ptr = &mut x as *mut i32;
    assert!(unsafe{reader_writer(&mut *x_ptr, &*x_ptr)} == 1);
}
