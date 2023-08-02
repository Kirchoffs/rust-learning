#[cfg(test)]
mod test {
    use std::thread;

    #[test]
    fn thread_demo() {
        thread::spawn(f);
        thread::spawn(f);

        println!("Hello from the main thread.");
    }

    fn f() {
        println!("Hello from another thread!");
    
        let id = thread::current().id();
        println!("This is my thread id: {id:?}");
    }
}
