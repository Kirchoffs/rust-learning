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
}
