#[cfg(test)]
mod test {
    #[test]
    fn map_err_demo() {
        let s = "42";
        let r = s.parse::<i32>().map_err(|e| e.to_string());
        assert_eq!(r, Ok(42));

        let s = "apple";
        let r = s.parse::<i32>().map_err(|e| e.to_string());
        assert_eq!(r, Err("invalid digit found in string".to_string()));
    }
}
