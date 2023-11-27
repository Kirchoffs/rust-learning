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

    #[test]
    fn for_int_vec_demo() {
        let numbers = vec![1, 3, 5, 4, 7, 8, 9];

        for number in numbers {
            println!("First round: {}", number);
        }

        // We cannot use numbers again because it has been moved into the for loop,
        // although the number type is i32 which is Copy trait.
        // for number in numbers {
        //     println!("Second round: {}", number);
        // }
    }

    #[test]
    fn iter_fold_demo() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = numbers
            .into_iter()
            .fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);

        let numbers = vec![1, 2, 3, 4, 5];
        let sum = numbers
            .iter()
            .fold(0, |acc, &x| acc + x);
        assert_eq!(sum, 15);
    }
}