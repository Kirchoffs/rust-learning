#[cfg(test)]
mod test {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    #[derive(Hash)]
    struct Person {
        name: String,
        age: u32,
    }
    
    #[test]
    fn default_hasher_demo() {
        let person = Person {
            name: String::from("Alice"),
            age: 32,
        };
    
        // SipHash-1-3 is the default hasher for HashMap and HashSet
        let mut hasher = DefaultHasher::new();
        
        person.hash(&mut hasher);
        
        let hash_value = hasher.finish();
    
        println!("Hash value: {}", hash_value);
    }    
}
