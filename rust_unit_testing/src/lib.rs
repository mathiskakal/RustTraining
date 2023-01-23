pub fn add(left: usize, right: usize) -> usize {
    left + right
}


// This attribute tells cargo to run the following code only during testing
#[cfg(test)]
// This is where tests are written
mod tests {
    use super::*;

    // This attribute tells cargo that below is a test
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
