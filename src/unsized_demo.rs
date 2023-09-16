#[cfg(test)]
mod test {
    #[test]
    fn test_unsized_demo() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let reference: &[i32] = &arr[1..4];
        println!("{:?}", reference);
    }
}