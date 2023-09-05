#[cfg(test)]
mod test {
    #[test]
    fn option_copied_demo() {
        let maybe_num: Option<&i32> = Some(&42);
        let copied_num: Option<i32> = maybe_num.copied();
        
        match copied_num {
            Some(num) => println!("Copied value: {}", num),
            None => println!("No value"),
        }
    }

    #[test]
    fn option_cloned_demo() {
        let mut map = std::collections::HashMap::new();
        map.insert(1, vec![1]);

        let one_vec_option: Option<&Vec<i32>> = map.get(&1);
        let one_vec: Option<Vec<i32>> = one_vec_option.cloned();
        println!("one_vec: {:?}", one_vec);
    }
}