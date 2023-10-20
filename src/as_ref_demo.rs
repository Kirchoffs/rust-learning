#[cfg(test)]
mod test {
    #[test]
    fn as_ref_demo_hitotsu() {
        let s1 = String::from("Hello");
        let s2: &str = s1.as_ref();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    #[test]
    fn as_ref_demo_futatsu() {
        let integer_wrapper = IntegerWrapper { value: 42 };
        print_i32(integer_wrapper.as_ref());
        print_value(integer_wrapper);
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

    #[test]
    fn as_ref_demo_mittsu() {
        let t1 = String::from("Hello");
        print_str_ref(t1);

        let t2 = IntStrWrapper::new(42);
        print_str_ref(t2);
    }

    struct IntStrWrapper {
        val: i32,
        val_holder: String,
    }

    impl IntStrWrapper {
        fn new(val: i32) -> Self {
            Self { 
                val,
                val_holder: String::from(val.to_string()),
            }
        }
    }

    impl AsRef<str> for IntStrWrapper {
        fn as_ref(&self) -> &str {
            &self.val_holder
        }
    }

    fn print_str_ref<T: AsRef<str>>(t: T) {
        let t_ref = t.as_ref();
        println!("t_ref = {}", t_ref);
    }
}
