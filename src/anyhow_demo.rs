#[cfg(test)]
mod test {
    use std::any;

    use anyhow::Result;

    fn divide_alpha(a: i32, b: i32) -> Result<f32> {
        if b == 0 {
            anyhow::bail!("Cannot divide by zero");
        }
    
        let result = (a as f32) / (b as f32);
        Ok(result)
    }

    #[test]
    fn anyhow_alpha_demo() {
        let result = divide_alpha(10, 2);

        match result {
            Ok(value) => {
                println!("Result: {}", value);
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    fn divide_beta(a: i32, b: i32) -> Result<f32, String> {
        if b == 0 {
            return Err(String::from("Cannot divide by zero"));
        }
    
        let result = (a as f32) / (b as f32);
        Ok(result)
    }

    #[test]
    fn anyhow_beta_demo() {
        let result = divide_beta(10, 0);

        match result {
            Ok(value) => {
                println!("Result: {}", value);
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    fn parse_number(s: &str) -> Result<i32> {
        s.parse::<i32>()
            .map_err(|err| anyhow::anyhow!(err))
    }

    #[test]
    fn anyhow_gamma_demo() {
        let str = "x";
        let result = parse_number(str);
        assert!(result.is_err());
        println!("Error: {}", result.err().unwrap());
    }

    fn process_data(data: &str) -> Result<String> {
        if data.is_empty() {
            Err(anyhow::anyhow!("Data is empty, cannot process"))
        } else if data == "invalid" {
            Err(anyhow::anyhow!("Invalid data provided"))
        } else {
            let result = format!("Processed: {}", data);
            Ok(result)
        }
    }

    #[test]
    fn anyhow_delta_demo() {
        let data = "invalid";
        let processed_data = process_data(data);
        assert!(processed_data.is_err());
    }
}
