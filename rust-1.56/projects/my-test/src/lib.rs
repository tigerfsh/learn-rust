#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert!(10 < 15, "left = {}, right = {}", 10, 15);
        assert_eq!(result, 4, "We are testing it_works with {} and {}", result, 4);
    }

    #[test]
    fn another() {
        // panic!("Error.");
    }
}
