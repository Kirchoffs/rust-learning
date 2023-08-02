#[cfg(test)]
mod test {
    use std::{sync::{RwLock, Arc}, thread};

    #[test]
    fn rwlock_demo() {
        let shared_data = Arc::new(RwLock::new(vec![]));

        let mut handles = vec![];
        for i in 0 .. 5 {
            let data = Arc::clone(&shared_data);
            let handle = thread::spawn(move || {
                let read_data = data.read().unwrap();
                println!("Thread {}: Shared data: {:?}", i, *read_data);
                drop(read_data);

                let mut write_data = data.write().unwrap();
                write_data.push(i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let mut data = shared_data.write().unwrap();
        data.push(42);
        println!("Modified data: {:?}", *data);
    }
}
