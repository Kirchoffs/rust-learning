#[cfg(test)]
mod test {
    use rand::Rng;

    /**
     * If an error occurs at any point during the chain, the error message will be printed, and the execution will stop. 
     * If no errors occur, the "Done!" message will be printed after the chain is complete.
     */
    #[test]
    fn chain_result_demo() {
        if let Err(err) = get_result_one().and_then(get_result_two) {
            println!("Error: {}", err);
        }
        println!("Done!")
    }

    fn get_result_one() -> Result<i32, String> {
        let mut rng = rand::thread_rng();

        let choice = rng.gen_range(0..2);
        if choice == 0 {
            Ok(42)
        } else {
            Err("Oops for step 1!".to_string())
        }
    }

    fn get_result_two(val: i32) -> Result<i32, String> {
        let mut rng = rand::thread_rng();

        let choice = rng.gen_range(0..2);
        if choice == 0 {
            Ok(val)
        } else {
            Err("Oops for step 2!".to_string())
        }
    }
}