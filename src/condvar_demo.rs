#[cfg(test)]
mod test {
    #[test]
    fn condvar_demo() {
        use std::sync::{Arc, Condvar, Mutex};
        use std::thread;

        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        let thread = thread::spawn(move || {
            let (lock, cvar) = &*pair_clone;
            let mut started = lock.lock().unwrap();
            println!("Begin change started to true");
            *started = true;
            cvar.notify_one();
        });

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        println!("End change started to true");

        thread.join().unwrap();
    }
}
