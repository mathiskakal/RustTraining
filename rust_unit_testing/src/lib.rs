pub fn adder(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    // this brings everything from the parent's scope into this scope
    use super::*;

    #[test]
    fn it_adds() {
        let sum = adder(4, 5);
        // in rust there are no conventions as to which left or right is expected and given
        assert_eq!(sum, 9);
    }

}