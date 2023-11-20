#[cfg(test)]
mod test {
    #[test]
    fn map_err_demo() {
        sub_test().unwrap();

        let s = "apple";
        let r = s
            .parse::<i32>()
            .map_err(|e| e.to_string());
        assert_eq!(r, Err("invalid digit found in string".to_string()));
    }

    fn sub_test() -> Result<(), String> {
        let s = "42";
        let r = s
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        assert_eq!(r, 42);
        Ok(())
    }
}
