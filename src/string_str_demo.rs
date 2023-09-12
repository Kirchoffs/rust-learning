#[cfg(test)]
mod test {
    #[test]
    fn string_str_demo_hitotsu() {
        let s: String = "abc".to_string();
        
        let rs_1: &str = s.as_str();

        let rs_2: &String = &s;
        let rs_3: &str = rs_2.as_str();

        let rs_4: &str = &s;

        assert_eq!(rs_1, rs_2);
        assert_eq!(rs_1, rs_3);
        assert_eq!(rs_1, rs_4);
    }
}