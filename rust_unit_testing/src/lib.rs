
// add a rule to allow only single digit integers (positive, zero and negative) otherwise, panic
/*
pub fn adder(x: i32, y: i32) -> i32 {
    x + y
}
*/

pub fn single_digit_adder(x: i8, y: i8) -> i8 {
    fn is_single_digit(x: i8) -> bool {
        x < 10 && x > -10
    }

    if !(is_single_digit(x)) || !(is_single_digit(y)) {
        panic!("Only single digit integers are allowed!");

    } else {
        x + y
    }
}

#[cfg(test)]
mod tests {
    // this brings everything from the parent's scope into this scope
    use super::*;

    #[test]
    fn it_adds() {
        let sum = single_digit_adder(4, 5);
        // in rust there are no conventions as to which left or right is expected and given
        assert_eq!(sum, 9);
    }

    #[test]
    #[should_panic]
    // this one will only pass ONLY if single_digit_adder panics, so it is expected to fail
    fn it_should_only_accept_single_digits() {
        single_digit_adder(11, 4);
    }

}