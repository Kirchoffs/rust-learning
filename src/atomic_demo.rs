#[cfg(test)]
mod test {
    // Memory ordering:
    // Ordering::Relaxed: No guarantees are made about the ordering of operations/reads and writes.
    // Ordering::Release: All writes before the barrier are visible to other threads.
    // Ordering::Acquire: All reads after the barrier are guaranteed to see the writes before the barrier.
    // Ordering::AcqRel: Combines Acquire and Release.
    // Ordering::SeqCst: All reads and writes are guaranteed to be in a single total order.

    use std::{sync::atomic::{AtomicU64, Ordering}, thread::{self, JoinHandle}};

    #[test]
    fn atomic_demo() {
        let num_times = 10000000;
        let num_threads = 10;

        static COUNTER: AtomicU64 = AtomicU64::new(0);

        fn add_num_times(num_times: u64) -> JoinHandle<()> {
            thread::spawn(move || {
                for _ in 0..num_times {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            })
        }

        let mut threads = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            threads.push(add_num_times(num_times));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(COUNTER.load(Ordering::Relaxed), num_times * num_threads as u64);
    }
}