#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn map_demo() {
        let mut m: HashMap<&str, i32> = HashMap::new();
        let s = "apple".to_string();
        let v = m.entry(&s).or_insert(42);
        *v += 1;
        assert_eq!(m["apple"], 43);
    }
}