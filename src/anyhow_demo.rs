#[cfg(test)]
mod test {
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
}
