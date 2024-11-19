#[cfg(test)]
mod test {
    #[test]
    fn box_demo() {
        let mut x = Box::new(42);
        println!("x: {}", x);
        {
            let y = Box::new(89);
            x = y.clone(); // It will copy the value of y to x
        }
        assert_eq!(x, Box::new(89));
    }
}
