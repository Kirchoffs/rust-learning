#[cfg(test)]
mod test {
    use core::panic;

    #[test]
    #[should_panic]
    fn test() {
        panic!("Hello, panic!");
    }

    #[test]
    #[should_panic(expected = "Hello")]
    fn test_expected() {
        panic!("Hello, world!");
    }
}