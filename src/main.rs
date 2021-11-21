use std::os::raw::c_int;
use rustffi::*;

extern fn callback(a: i32) {
    println!("I'm called from C with value {0}", a+1);
}

fn main() {
    let numbers: [c_int; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    unsafe {
        register_callback(Some(callback));
        let total = sum(numbers.as_ptr(), numbers.len() as c_int);
        println!("The total is {}", total);
        trigger_callback(); // Triggers the callback

        assert_eq!(total, numbers.iter().sum());
    }
}