#[cfg(test)]
mod test {
    #[test]
    fn iter_find_demo() {
        let numbers = vec![1, 3, 5, 4, 7, 8, 9];

        let result = numbers
            .iter()
            .find(|&&x| x % 2 == 0);
    
        match result {
            Some(&even) => {
                println!("Found the first even number: {}", even);
            }
            None => {
                println!("No even numbers found.");
            }
        }
    }
}