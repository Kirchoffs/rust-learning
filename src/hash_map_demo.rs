#[cfg(test)]
mod test {
    #[test]
    fn map_demo() {
        use std::collections::HashMap;

        let mut m: HashMap<&str, i32> = HashMap::new();
        let s = "apple".to_string();
        let v = m.entry(&s).or_insert(42);
        *v += 1;
        assert_eq!(m["apple"], 43);
    }

    #[test]
    fn custom_key_demo() {
        use std::collections::HashMap;
        use std::hash::Hash;

        #[derive(Debug, Eq, PartialEq, Clone)]
        struct Person {
            name: String,
            age: u32,
        }

        impl Hash for Person {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.name.hash(state);
                self.age.hash(state);
            }
        }

        let mut score: HashMap<Person, i32> = HashMap::new();

        let person = Person {
            name: "Alice".to_string(),
            age: 32,
        };

        score.insert(person.clone(), 42);
        
        assert_eq!(score[&person], 42);
    }
}