#[cfg(test)]
mod test {
    use std::io;
    use std::num;
    use std::fs::File;
    use std::io::Read;

    #[derive(Debug)]
    struct AppError {
        category: String,
        message: String,
    }
    
    impl From<io::Error> for AppError {
        fn from(error: io::Error) -> Self {
            AppError {
                category: "io".to_string(),
                message: error.to_string(),
            }
        }
    }
    
    impl From<num::ParseIntError> for AppError {
        fn from(error: num::ParseIntError) -> Self {
            AppError {
                category: "parse".to_string(),
                message: error.to_string(),
            }
        }
    }

    #[test]
    fn customized_error_read_file() {        
        fn read_file_contents() -> Result<String, AppError> {
            let mut file = File::open("unexisted_file.txt")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }
        
        match read_file_contents() {
            Ok(contents) => println!("{}", contents),
            Err(error) => println!("{} - {}", error.category, error.message),
        }
    }

    #[test]
    fn customized_error_parse_int() {
        fn parse_int_from_string() -> Result<i32, AppError> {
            let number_str = "e";
            let number = number_str.parse::<i32>()?;
            Ok(number)
        }
        
        match parse_int_from_string() {
            Ok(number) => println!("{}", number),
            Err(error) => println!("{} - {}", error.category, error.message),
        }
    }
}
