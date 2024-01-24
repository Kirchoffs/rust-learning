#[cfg(test)]
mod test {
    #[test]
    fn demo_alpha() {
        let a = 1;
        let b_ref = process_alpha(&a);
        println!("{}", b_ref);
    }

    fn process_alpha(a_ref: &i32) -> Box<i32> {
        let b = *a_ref + 42;
        Box::new(b)
    }

    #[test]
    fn demo_beta() {
        let a = 1;
        let b_ref = process_beta(&a);
        println!("{}", b_ref);
    }

    fn process_beta(a_ref: &i32) -> &'static i32 {
        let b = *a_ref + 42;
        Box::leak(Box::new(b))
    }
}