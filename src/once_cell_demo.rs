#[cfg(test)]
mod test {
    #[test]
    fn lazy_initialization_demo() {
        use once_cell::sync::Lazy;
        use std::collections::HashMap;

        static GLOBAL_CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
            let mut config = HashMap::new();
            config.insert("key".to_string(), "rabbit".to_string());
            config.insert("value".to_string(), "42".to_string());
            println!("Config initialized!");
            config
        });

        println!("Accessing GLOBAL_CONFIG for the first time...");
        println!("{:?}", GLOBAL_CONFIG.get("key"));
        println!("Accessing GLOBAL_CONFIG for the second time...");
        println!("{:?}", GLOBAL_CONFIG.get("value"));
    }
}
