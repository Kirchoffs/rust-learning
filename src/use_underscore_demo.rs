use std::fs::File;
use std::io::Read;

trait Write {
    fn read_file_then_write_to_console(&self);
}

impl Write for String {
    fn read_file_then_write_to_console(&self) {
        let mut file = File::open(self).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
}

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use std::io::{Write as _, Read as _};
    use super::Write;

    #[test]
    fn use_underscore_demo() {
        let file_name = String::from("example.txt");

        let mut file = File::create(&file_name).unwrap();
        let data = b"Hello, Write trait!";
        file.write_all(data).unwrap();

        let mut file = File::open(&file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, data.to_vec().into_iter().map(|c| c as char).collect::<String>());

        write_to_console(&file_name);

        fs::remove_file(&file_name).unwrap();
    }

    fn write_to_console<T: Write>(t: &T) {
        t.read_file_then_write_to_console();
    }
}
