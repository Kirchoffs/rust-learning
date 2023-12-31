#[cfg(test)]
mod test {
    struct Counter {
        count: usize,
    }

    impl Counter {
        pub fn new() -> Self {
            Self {
                count: 0,
            }
        }
    }

    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
    
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn counter_iterator_alpha() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn counter_iterator_beta() {
        let mut counter = Counter::new();

        while let Some(num) = counter.next() {
            println!("{num}");
        }
    }
}
