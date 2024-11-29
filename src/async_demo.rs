#[cfg(test)]
mod test {
    use std::process::Output;
    use std::future::Future;
    use futures::executor::block_on; // cargo add futures

    #[test]
    fn async_hello_world_block_on() {
        async fn hello_world() {
            println!("Hello world!");
        }

        let future = hello_world();
        block_on(future);
    }

    #[tokio::test]                   // cargo add tokio
    async fn async_hello_world_await() {
        async fn hello_world() {
            println!("Hello world!");
        }

        let future = hello_world();
        future.await;
    }

    #[tokio::test]
    async fn async_more_demo() {
        struct Song {
            author: String,
            name: String,
        }

        async fn learn_song() -> Song {
            println!("I'm learning a song");

            Song {
                author: "The Beatles".to_string(),
                name: "Yellow Submarine".to_string(),
            }
        }

        async fn sing_song(song: Song) {
            println!("I'm singing {} by {}", song.name, song.author);
        }

        async fn learn_and_sing() {
            let song = learn_song().await;
            sing_song(song).await;
        }

        async fn dance() {
            println!("I'm dancing");
        }

        let f1 = learn_and_sing();
        let f2 = dance();
        futures::join!(f1, f2);
    }

    #[tokio::test]
    async fn async_lifecycle_demo() {
        async fn borrow(x: &i32) -> i32 {
            *x
        }

        // fn bad() -> impl Future<Output = i32> {
        //     let x = 42;
        //     borrow(&x)  --> x does not live long enough
        // }

        fn good() -> impl Future<Output = i32> {
            async {
                let x = 42;
                borrow(&x).await
            }
        }
    }

    #[tokio::test]
    async fn async_stream_demo() {
        use futures::channel::oneshot;
        use futures::stream::{self, StreamExt, TryStreamExt};

        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let (_, rx3) = oneshot::channel();

        let stream = stream::iter(vec![rx1, rx2, rx3]);
        let stream_fut = stream.map(Ok).try_for_each_concurrent(
            2,
            |rx| async move {
                let res: Result<(), oneshot::Canceled> = rx.await;
                res
            }
        );

        tx1.send(()).unwrap();
        drop(tx2);

        assert_eq!(Err(oneshot::Canceled), stream_fut.await);
    }
}