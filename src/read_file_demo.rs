#[cfg(test)]
mod test {
    use std::io::Read;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Error;
    use std::fs::File;

    #[test]
    fn read_file_demo_not_preserving_newline() {
        let file_path = "resources/input.txt";
        match open(file_path) {
            Ok(reader) => {
                // For reader.lines(), trait BufRead is required.
                for line in reader.lines() {
                    print!("{}", line.unwrap());
                }
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    #[test]
    fn read_file_demo_preserving_newline() -> Result<(), Error> {
        let file_path = "resources/input.txt";
        match open(file_path) {
            Ok(mut reader) => {
                let mut line = String::new();
                loop {
                    let bytes = reader.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }
                    print!("{}",line);
                }
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        Ok(())
    }

    #[test]
    fn read_file_demo_entire() -> Result<(), Error> {
        let file_path = "resources/input.txt";
        match open(file_path) {
            Ok(mut reader) => {
                let mut contents = String::new();
                reader.read_to_string(&mut contents)?;
                println!("{}", contents);
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        Ok(())
    }

    #[test]
    fn read_file_demo_specified_bytes() -> Result<(), Error> {
        let file_path = "resources/input.txt";
        let num_bytes = 5;
        match open(file_path) {
            Ok(reader) => {
                let mut handle = reader.take(num_bytes as u64);
                let mut buffer = vec![0; num_bytes];
                let bytes_read = handle.read(&mut buffer)?;
                println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        Ok(())
    }

    #[test]
    fn read_file_demo_specified_bytes_more() -> Result<(), Error> {
        let file_path = "resources/input.txt";
        let num_bytes = 5;
        match open(file_path) {
            Ok(reader) => {
                let bytes: Result<Vec<_>, _> = reader.bytes().take(num_bytes).collect();
                println!("{}", String::from_utf8_lossy(&bytes?));
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
        Ok(())
    }

    fn open(file_path: &str) -> Result<BufReader<File>, Error> {
        let file = File::open(file_path)?;
        Ok(BufReader::new(file))
    }
}