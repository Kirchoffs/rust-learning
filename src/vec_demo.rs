#[cfg(test)]
mod test {
    #[test]
    fn vec_demo_alpha() {
        let v = vec!["tom".to_string(), "ben".to_string(), "bill".to_string()];
        // Wrong usage: let e = v[1];
        let e = &v[1];
        println!("e = {}", e);
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
}