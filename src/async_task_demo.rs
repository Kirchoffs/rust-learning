#[cfg(test)]
mod test {
    #[test]
    fn smol_simple_demo() {
        use std::sync::Arc;
        use std::sync::Mutex;
        use smol::future;

        let queue = Arc::new(Mutex::new(Vec::new()));

        let (task, handle) = async_task::spawn(async {
            println!("Hello from the async task!");
            println!("The return value will be 42");
            42
        }, {
            println!("Task scheduled!");
            let queue = Arc::clone(&queue);
            move |schedule_task| {
                println!("Task started!");
                queue.lock().unwrap().push(schedule_task);
            }
        });

        println!("Task created and we are going to schedule it!");
        task.schedule();

        println!("We are going to pop the task from the queue and run it!");
        while let Some(task) = queue.lock().unwrap().pop() {
            task.run();
        }

        println!("We are going to block on the handle to get the result!");
        let result = future::block_on(handle);
        println!("Result: {}", result);
    }
}
