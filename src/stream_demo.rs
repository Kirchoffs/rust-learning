#[cfg(test)]
mod test {
    #[test]
    fn stream_demo_hitotsu() {
        let v: Vec<Result<i32, String>> = vec![Ok(1), Err("nope".to_string()), Ok(3)];

        v
            .into_iter()
            .map_while(Result::ok)
            .filter(|&x| x < 3)
            .for_each(|x| println!("{}", x));        
    }

    #[test]
    fn stream_demo_futatsu() {
        let v: Vec<Result<i32, String>> = vec![Ok(1), Err("nope".to_string()), Ok(3)];

        v
            .into_iter()
            .map(Result::ok)
            .filter(|&x| x.is_some() && x.unwrap() < 3)
            .for_each(|x| println!("{}", x.unwrap()));        
    }

    #[test]
    fn stream_demo_mittsu() {
        let v = [-1i32, 4, 0, 1];

        let mut iter = v
            .iter()
            .map_while(|&e| 16i32.checked_div(e));

        assert_eq!(iter.next(), Some(-16));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), Some(16));
    }

    #[test]
    fn stream_demo_yottsu() {
        let  v= [-1i32, 4, 0, 1];

        let mut iter = v
            .iter()
            .map(|e| 16i32.checked_div(*e))
            .take_while(|e| e.is_some())
            .map(|e| e.unwrap());
        
        assert_eq!(iter.next(), Some(-16));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}