#[cfg(test)]
mod test {
    use std::{thread, time};
    use std::thread::JoinHandle;

    fn f() {
        println!("Hello from another thread!");
    
        let id = thread::current().id();
        println!("This is my thread id: {id:?}");
    }

    #[test]
    fn thread_demo() {
        thread::spawn(f);
        thread::spawn(f);

        println!("Hello from the main thread.");
    }

    fn do_something(number: i8) -> i8 {
        println!("number {} is running", number);
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        return 2
    }

    #[test]
    fn thread_join_demo() {
        let now = time::Instant::now();

        let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
        let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
        let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));
        let result_one = thread_one.join();
        let result_two = thread_two.join();
        let result_three = thread_three.join();
        println!("time elapsed {:?}", now.elapsed());
        println!("result {}", result_one.unwrap() + result_two.unwrap() + result_three.unwrap());
    }
}
