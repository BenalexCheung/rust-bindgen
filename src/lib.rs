mod utils;
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_sum() {
        use std::os::raw::c_int;
        let numbers: [c_int; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        unsafe {
            let total = sum(numbers.as_ptr(), numbers.len() as c_int);
            println!("The total is {}", total);
            assert_eq!(total, numbers.iter().sum());
        }
    }
}
