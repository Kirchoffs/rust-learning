#[cfg(test)]
mod test {
    #[test]
    fn binary_search() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 9];

        let t1 = 4;
        match v.binary_search_by(|e| {
            e.cmp(&t1)
        }) {
            Ok(equal_index) => {
                println!("Equal index: {}", equal_index);
            },
            Err(insert_index) => {
                println!("Insert index: {}", insert_index);
            }
        };

        let t2 = 8;
        match v.binary_search_by(|e| {
            e.cmp(&t2)
        }) {
            Ok(equal_index) => {
                println!("Equal index: {}", equal_index);
            },
            Err(insert_index) => {
                println!("Insert index: {}", insert_index);
            }
        };

        let t3 = 11;
        match v.binary_search_by(|e| {
            e.cmp(&t3)
        }) {
            Ok(equal_index) => {
                println!("Equal index: {}", equal_index);
            },
            Err(insert_index) => {
                println!("Insert index: {}", insert_index);
            }
        };
    }
}
