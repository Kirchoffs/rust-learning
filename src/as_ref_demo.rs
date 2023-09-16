#[cfg(test)]
mod test {
    #[test]
    fn as_ref_demo_hitotsu() {
        let s1 = String::from("Hello");
        let s2: &str = s1.as_ref();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    struct IntegerWrapper {
        value: i32,
    }

    impl AsRef<i32> for IntegerWrapper {
        fn as_ref(&self) -> &i32 {
            &self.value
        }
    }

    fn print_value<T: AsRef<i32>>(val: T) {
        println!("val = {}", val.as_ref());
    }

    fn print_i32(val: &i32) {
        println!("val = {}", val);
    }

    fn p(s: &str) {
        println!("s = {}", s);
    }

    #[test]
    fn as_ref_demo_futatsu() {
        let integer_wrapper = IntegerWrapper { value: 42 };
        print_i32(integer_wrapper.as_ref());
        print_value(integer_wrapper);
    }
}