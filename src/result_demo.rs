#[cfg(test)]
mod test {
    use rand::Rng;

    /**
     * If an error occurs at any point during the chain, the error message will be printed, and the execution will stop. 
     * If no errors occur, the "Done!" message will be printed after the chain is complete.
     */
    #[test]
    fn chain_result_demo_hitotsu() {
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

    #[test]
    fn result_demo_futatsu() {
        let result: Result<i32, String> = Ok(42);
        let option = result.ok();

        assert_eq!(option, Some(42));
    }

    #[test]
    fn result_as_ref_demo() {
        let result: Result<i32, String> = Ok(42);
        let result_ref = &result;
        let result_ref_as_ref = result_ref.as_ref();
        assert_eq!(result_ref_as_ref, Ok(&42));
    }

    #[test]
    fn result_map_demo() {
        let rf = |s: &str| s.chars().count();
        let r1: Result<&str, &str> = Ok("42");
        let r2 = r1.map(rf);
        assert_eq!(r2, Ok(2));

        let re = |s: &str| -> isize { s.parse().unwrap() };
        let e1: Result<&str, &str> = Err("42");
        let e2 = e1.map_err(re);
        assert_eq!(e2, Err(42));
    }

    #[test]
    fn result_bool_demo() {
        let r1: Result<i32, &str> = Ok(42);
        let r2: Result<i32, &str> = Err("42");
        let r3 = r1.or(r2);
        assert_eq!(r3, Ok(42));
    }
}