#[cfg(test)]
mod test {
    use std::{sync::Arc, thread, time::Duration};

    #[test]
    fn arc_demo() {
        let arc_example = Arc::new("Arc Examples");

        for _ in 0 .. 10 {
            let arc_example = Arc::clone(&arc_example);

            thread::spawn(move || {
                println!("{:?}", arc_example);
            });
        }

        thread::sleep(Duration::from_secs(1));
    }
}
