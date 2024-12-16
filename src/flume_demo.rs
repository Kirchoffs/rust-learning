#[cfg(test)]
mod test {
    #[test]
    fn unbounded_single_sender_single_receiver_demo() {
        use flume::{unbounded, Receiver, Sender};

        let (tx, rx): (Sender<i32>, Receiver<i32>) = unbounded();

        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[tokio::test]
    async fn unbounded_single_sender_single_receiver_async_demo() {
        use flume::{unbounded, Receiver, Sender};

        let (tx, rx) = unbounded();

        let sender = tokio::spawn(async move {
            tx.send_async(42).await.unwrap();
        });

        let receiver = tokio::spawn(async move {
            assert_eq!(rx.recv_async().await.unwrap(), 42);
        });

        sender.await.unwrap();
        receiver.await.unwrap();
    }

    #[test]
    fn unbounded_multiple_senders_single_receiver_demo() {
        use flume::{unbounded, Receiver, Sender};
        use std::thread;

        let (tx, rx) = unbounded();

        let tx1 = tx.clone();
        let tx2 = tx.clone();
    
        let producer1 = thread::spawn(move || {
            tx1.send("Message from producer 1").unwrap();
        });
    
        let producer2 = thread::spawn(move || {
            tx2.send("Message from producer 2").unwrap();
        });
    
        producer1.join().unwrap();
        producer2.join().unwrap();
    
        for message in rx.iter().take(2) {
            println!("Received: {}", message);
        }
    }
}
