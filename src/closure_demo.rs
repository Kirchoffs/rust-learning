#[cfg(test)]
mod test {
    #[test]
    fn borrow_demo() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");
    
        let borrows = || println!("From closure: {list:?}");
    
        println!("Before calling closure: {list:?}");
        borrows();
        println!("After calling closure: {list:?}");
    }

    #[test]
    fn borrow_mutably_demo() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");
    
        let mut borrows_mutably = || list.push(7);
    
        borrows_mutably();
        println!("After calling closure: {list:?}");
    }

    #[test]
    fn move_demo() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");
    
        let moves = move || println!("From closure: {list:?}");
        moves();
    }

    #[test]
    fn closure_in_struct_demo() {
        struct Cacher<T, E>
        where
            T: Fn(E) -> E,
            E: Copy
        {
            query: T,
            value: Option<E>,
        }
        
        impl<T, E> Cacher<T, E>
        where
            T: Fn(E) -> E,
            E: Copy
        {
            fn new(query: T) -> Cacher<T, E> {
                Cacher {
                    query,
                    value: None,
                }
            }
        
            fn value(&mut self, arg: E) -> E {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.query)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

        let mut cacher = Cacher::new(|a| a + 1);
        let v1 = cacher.value(1);
        println!("v1: {v1}");
        let v2 = cacher.value(1);
        println!("v2: {v2}");
    }
}
