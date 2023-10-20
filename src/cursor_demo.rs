#[cfg(test)]
mod test {
    use std::io::{self, Write, Seek, SeekFrom};
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
}
