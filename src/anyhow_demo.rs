#[cfg(test)]
mod test {
    use anyhow::Result;

    fn divide(a: i32, b: i32) -> Result<f32> {
        if b == 0 {
            anyhow::bail!("Cannot divide by zero");
        }
    
        let result = (a as f32) / (b as f32);
        Ok(result)
    }

    #[test]
    fn anyhow_demo() {
        let result = divide(10, 2);

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
