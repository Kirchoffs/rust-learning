#[cfg(test)]
mod test {
    #[test]
    fn vec_demo() {
        let v = vec!["tom".to_string(), "ben".to_string(), "bill".to_string()];
        // Wrong usage: let e = v[1];
        let e = &v[1];
        println!("e = {}", e);
    }
}