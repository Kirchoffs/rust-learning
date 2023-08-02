#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn arc_mutex_demo() {
        let shared_data = Arc::new(Mutex::new(vec![]));

        let mut handles = vec![];
        for i in 0 .. 5 {
            let data = Arc::clone(&shared_data);
            let handle = thread::spawn(move || {
                let mut data = data.lock().unwrap();
                data.push(i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let data = shared_data.lock().unwrap();
        println!("Final data: {:?}", *data);
    }
}
