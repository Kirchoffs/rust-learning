#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    struct IntegerWrapper {
        value: i32,
    }

    impl Borrow<i32> for IntegerWrapper {
        fn borrow(&self) -> &i32 {
            &self.value
        }
    }

    #[test]
    fn borrow_demo_hitotsu() {
        let integer_wrapper = IntegerWrapper { value: 42 };
        let integer_wrapper_ref: &i32 = integer_wrapper.borrow();
        println!("integer_wrapper_ref = {}", integer_wrapper_ref);
    }
}