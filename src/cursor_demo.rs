#[cfg(test)]
mod test {
    use std::io::{self, Write, Read, Seek, SeekFrom};
    use std::io::Cursor;

    fn write_ten_bytes_at_end<W: Write + Seek>(mut writer: W) -> io::Result<()> {
        writer.seek(SeekFrom::End(-10))?;
    
        for i in 0 .. 10 {
            writer.write(&[i])?;
        }
    
        Ok(())
    }
    

    #[test]
    fn cursor_demo() {
        let mut buff = Cursor::new(vec![0; 15]);
        write_ten_bytes_at_end(&mut buff).unwrap();
        assert_eq!(&buff.get_ref()[5..15], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn cursor_write_demo() {
        let mut buffer = Cursor::new(Vec::new());

        buffer.write_all(b"Hello, world!").unwrap();
    
        let written_data = buffer.into_inner();
    
        let written_string = String::from_utf8(written_data).unwrap();
        assert_eq!(written_string, "Hello, world!");
    }

    #[test]
    fn cursor_read_demo() {
        let data = b"Hello, world!";
        let mut cursor = Cursor::new(data);
    
        let mut buffer = Vec::new();
    
        cursor.read_to_end(&mut buffer).unwrap();
    
        let read_string = String::from_utf8(buffer).unwrap();
        assert_eq!(read_string, "Hello, world!");
    }
}
