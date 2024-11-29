#[cfg(test)]
mod test {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread::{self, sleep};
    use std::time::Duration;

    #[test]
    fn condvar_demo() {
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

    #[test]
    fn complex_condval_demo() {
        let flag_mu = Arc::new(Mutex::new(false));
        let cond = Arc::new(Condvar::new());
        let flag_mu_clone = Arc::clone(&flag_mu);
        let cond_clone = Arc::clone(&cond);

        let thread = thread::spawn(move || {
            let mut flag = *flag_mu_clone.lock().unwrap();
            let mut counter = 0;

            while counter < 3 {
                while !flag {
                    flag = *cond_clone.wait(flag_mu_clone.lock().unwrap()).unwrap();
                }

                {
                    flag = false;
                    *flag_mu_clone.lock().unwrap() = false;
                }

                counter += 1;
                println!("Innner loop counter: {}", counter);
            }
        });
        
        let mut counter = 0;
        loop {
            sleep(Duration::from_millis(1000));
            *flag_mu.lock().unwrap() = true;
            counter += 1;
            if counter > 3 {
                break;
            }
            println!("Outer loop counter: {}", counter);
            cond.notify_one();
        }

        thread.join().unwrap();
        println!("{:?}", flag_mu.lock().unwrap());
    }
}
