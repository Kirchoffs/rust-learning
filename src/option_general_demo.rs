#[cfg(test)]
mod test {
    #[test]
    fn option_demo() {
        let mut x = Some(42);
        let y = x.take();
        
        assert_eq!(x, None);
        assert_eq!(y, Some(42));
    }
}
