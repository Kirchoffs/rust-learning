#[cfg(test)]
mod test {
    #[test]
    fn unsafe_hello_world_demo() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);

            *r2 = 6;
            println!("num is: {}", num);
        }
    }

    unsafe fn dangerous() {
        println!("This is a dangerous function.");
    }
    unsafe fn dangerous_wrapper() {
        dangerous();
    }
    #[test]
    fn unsafe_function_demo() {
        unsafe {
            dangerous_wrapper();
        }
    }

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        assert!(mid <= len);

        let ptr = values.as_mut_ptr();
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    #[test]
    fn unsafe_encapsulate_demo() {
        let mut values = [1, 2, 3, 4, 5];
        let (left, right) = split_at_mut(&mut values, 3);

        println!("left: {:?}", left);
        println!("right: {:?}", right);
    }

    static mut COUNTER: i32 = 0;
    fn add_to_count(inc: i32) {
        unsafe {
            COUNTER += inc;
        }
    }
    #[test]
    fn unsafe_static_demo() {
        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}
