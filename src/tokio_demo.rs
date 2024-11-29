#[cfg(test)]
mod test {
    use tokio::time;
    use std::thread;

    async fn do_something(number: i8) -> i8 {
        println!("number {} is running", number);
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        return 2;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn alpha_tokio_demo() {
        let now = time::Instant::now();
        let future_one = do_something(1);
        let outcome = future_one.await;
        println!("time elapsed {:?}", now.elapsed()); // Elapsed time is 2 seconds
        println!("Here is the outcome: {}", outcome);
    }

    // Future does not execute until awaited
    #[tokio::test(flavor = "multi_thread")]
    async fn beta_tokio_demo() {
        let now = time::Instant::now();
        let future_one = do_something(1);
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        let outcome = future_one.await;
        println!("time elapsed {:?}", now.elapsed()); // Elapsed time is 4 seconds
        println!("Here is the outcome: {}", outcome);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn gamma_tokio_demo() {
        let now = time::Instant::now();
        let future_one = tokio::spawn(do_something(1));
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        let outcome = future_one.await.unwrap();
        println!("time elapsed {:?}", now.elapsed());
        println!("Here is the outcome: {}", outcome);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn delta_tokio_demo() {
        let now = time::Instant::now();
        let future_one = tokio::spawn(do_something(1));
        let future_two = tokio::spawn(do_something(2));
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        future_one.await.unwrap();
        future_two.await.unwrap();
        println!("time elapsed {:?}", now.elapsed());
    }

    // Distinguish tokio::time::sleep from std::thread::sleep
    async fn do_something_optimized(number: i8) -> i8 {
        println!("number {} is running", number);
        let two_seconds = time::Duration::new(2, 0);
        tokio::time::sleep(two_seconds).await;
        return 2
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn epsilon_tokio_demo() {
        let now = time::Instant::now();
        let future_one = tokio::spawn(do_something_optimized(1));
        let future_two = tokio::spawn(do_something_optimized(2));
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        future_one.await.unwrap();
        future_two.await.unwrap();
        println!("time elapsed {:?}", now.elapsed());
    }
}