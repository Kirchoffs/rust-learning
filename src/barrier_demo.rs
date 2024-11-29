#[cfg(test)]
mod test {
    use std::sync::Barrier;
    use std::sync::Arc;

    #[test]
    fn barrier_demo() {
        let mut handlers = Vec::with_capacity(6);
        let barrier = Arc::new(Barrier::new(6));

        for _ in 0..6 {
            let barrier = Arc::clone(&barrier);
            let handler = std::thread::spawn(move || {
                println!("Before wait");
                barrier.wait();
                println!("After wait");
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }
}
