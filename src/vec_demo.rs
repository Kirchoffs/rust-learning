#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn vec_demo_alpha() {
        let v = vec!["tom".to_string(), "ben".to_string(), "bill".to_string()];
        // Wrong usage: let e = v[1];
        let e1 = &v[1];
        println!("e1 = {}", e1);

        let e2 = v.get(1);
        match e2 {
            Some(e) => println!("e2 = {}", e),
            None => println!("no such element at index 1"),
        }
    }

    #[test]
    fn vec_demo_beta() {
        let v = vec!["tom".to_string(), "ben".to_string(), "bill".to_string()];
        let idx = 1;
        match v.get(idx) {
            Some(e) => println!("e = {}", e),
            None => println!("no such element at index {}", idx),
        }
    }

    #[test]
    fn integer_vec_demo_alpha() {
        let v = vec![1, 2, 3, 4, 5];

        for e in v.iter() {
            println!("e = {}", e + 42);
        }

        for &e in v.iter() {
            println!("e = {}", e + 42);
        }

        // v is moved here
        for e in v.into_iter() {
            println!("e = {}", e + 42);
        }

        // Wrong usage: use of moved value: v
        // for e in v {
        //     println!("e = {}", e + 42);
        // }
    }

    #[test]
    fn integer_vec_demo_beta() {
        let v = vec![1, 2, 3, 4, 5];

        for e in v.iter() {
            println!("e = {}", e + 42);
        }

        for &e in v.iter() {
            println!("e = {}", e + 42);
        }

        // v is moved here
        for e in v {
            println!("e = {}", e + 42);
        }

        // Wrong usage: use of moved value: v
        // for e in v.into_iter() {
        //     println!("e = {}", e + 42);
        // }
    }

    #[test]
    fn vec_to_map_demo() {
        let v1 = vec![("tom", 42), ("ben", 24), ("bill", 36)];
        let m1: HashMap<_, _> = v1.into_iter().collect();
        println!("m1 = {:?}", m1);

        let v2 = vec![("tom", 42), ("ben", 24), ("bill", 36)];
        let mut m2 = HashMap::new();
        for e in &v2 {
            m2.insert(&e.0, e.1);
        }
        println!("m2 = {:?}", m2);
    }
}