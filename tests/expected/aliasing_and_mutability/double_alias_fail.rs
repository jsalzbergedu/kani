// Created by Jacob Salzberg

#[kani::modifies(x)]
#[kani::modifies(y)]
#[kani::requires(*x == x_old && *y == y_old)]
#[kani::ensures((y_old < 100 && *y == 0) || (y_old >= 100 && *y == y_old))]
fn reader_writer(x: &mut i32, y: &mut i32, x_old: i32, y_old: i32) {
    assert_eq!(*x, x_old);
    assert_eq!(*y, y_old);
    if *y < 100 {
        *x = 0;
    } else {
        *x = 1;
    }
    *y = *y * *x;
    return;
}

// The above SHOULD optimize to if *y < 100 0 else *y, but an unsafe write will ruin this.
#[kani::proof_for_contract(reader_writer)]
fn main() {
    let mut x = 1000;
    let x_ptr = &mut x as *mut i32;
    unsafe{reader_writer(&mut *x_ptr, &mut *x_ptr, *x_ptr, *x_ptr) }
    assert!(x == 1);
}
