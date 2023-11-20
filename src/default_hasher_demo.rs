#[cfg(test)]
mod test {
    use std::hash::{DefaultHasher, Hash as _, Hasher as _};

    #[test]
    fn default_hasher_demo() {
        let text = "Hello, world!";

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash_value = hasher.finish();
    
        println!("The hash for '{}' is {}", text, hash_value);
    }
}