#[cfg(test)]
mod test {
    #[test]
    fn closure_demo() {
        let name = String::from("ChatGPT");
        let mut cnt = 1;

        let mut closure = move || {
            println!("Hello, {}, you are No.{}.", name, cnt);
            cnt += 1;
        };
    
        closure();
        closure();
    }
}
