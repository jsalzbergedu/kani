// From the stacked borrows paper.
fn example1(x: &mut i32, y: &mut i32) -> i32 {
    *x = 42;
    *y = 13;
    return *x;
}

#[kani::proof]
fn main() {
    let mut local = 5;
    let raw_pointer = &mut local as *mut i32;
    let result = unsafe { example1(&mut *raw_pointer, &mut *raw_pointer) };
    assert!(local == 13);
}
