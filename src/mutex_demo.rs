#[cfg(test)]
mod test {
    use std::sync::Mutex;

    #[test]
    fn mutex_demo() {
        let shared_data = Mutex::new(0);

        {
            let mut data = shared_data.lock().unwrap();
            *data += 42;
        }
    
        let data = shared_data.lock().unwrap();
        println!("Final data: {}", *data);
    }
}