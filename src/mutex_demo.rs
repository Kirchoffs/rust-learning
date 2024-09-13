#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::thread;

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

    #[test]
    fn multithread_mutex_demo() {
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
