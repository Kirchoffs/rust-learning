#[cfg(test)]
mod test {
    use bytes::{BytesMut, BufMut};

    #[test]
    fn vec_u8_demo() {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(b"Hello, ");
        buffer.extend_from_slice(b"World!");

        let s = String::from_utf8(buffer).expect("Found invalid UTF-8");

        println!("{}", s);
    }

    #[test]
    fn bytes_mut_demo() {
        let mut buffer = BytesMut::with_capacity(64);

        buffer.put_slice(b"Hello, ");
        buffer.put_slice(b"World!");
    
        let s = String::from_utf8(buffer.to_vec()).expect("Found invalid UTF-8");
        println!("{}", s);
    }
}
