#[cfg(test)]
mod test {
    #[test]
    fn customized_useless_future_demo() {
        use std::time::{Duration, Instant};
        use std::thread;
        use std::fmt::Debug;

        trait SimpleFuture {
            type Output: Debug;
            fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
        }

        enum Poll<T> {
            Ready(T),
            Pending,
        }

        struct SleepFuture {
            duration: Duration,
            start: Option<Instant>,
        }

        impl SleepFuture {
            fn new(duration: Duration) -> Self {
                Self {
                    duration,
                    start: None,
                }
            }
        }

        impl SimpleFuture for SleepFuture {
            type Output = ();

            fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
                let now = Instant::now();
                if let Some(start) = self.start {
                    if now.duration_since(start) >= self.duration {
                        return Poll::Ready(());
                    }
                } else {
                    self.start = Some(now);
                    wake();
                }
                Poll::Pending
            }
        }

        fn run_simple_future<F: SimpleFuture>(mut future: F) {
            let wake = || println!("Waking up the executor!");
            loop {
                match future.poll(wake) {
                    Poll::Ready(output) => {
                        println!("Future completed with output: {:?}", output);
                        break;
                    }
                    Poll::Pending => {
                        println!("Future is pending...");
                    }
                }

                thread::sleep(Duration::from_millis(256));
            }
        }

        let sleep = SleepFuture::new(Duration::from_secs(1));
        run_simple_future(sleep);
    }
}
