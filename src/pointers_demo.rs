#[cfg(test)]
mod test {
    #[test]
    fn reference_address() {
        let mut x = 42;
        let y = &x;
        println!("{:p}", y);
        let z = &mut x;
        println!("{:p}", z);
    }

    #[test]
    fn raw_pointer() {
        // &i32
        // &mut i32
        // *const i32
        // *mut i32

        let mut x = 42;
        let x_ptr = &mut x as *mut i32;

        let y = Box::new(42);
        let y_ptr = Box::into_raw(y);

        unsafe {
            *x_ptr += *y_ptr;
        }
        assert_eq!(84, x);

        println!("{:p}", &x);
        let z_ptr = &mut x as *const i32;
        println!("{:p}", z_ptr);
    }

    #[test]
    fn box_pointer() {
        let arr = [-1; 32];
        let arr_copy = arr;
        println!("{}", arr.len());
        println!("{}", arr_copy.len());

        let arr_box = Box::new([-1; 32]);
        let arr_box_copy = arr_box;
        // println!("{}", arr_box.len());
        println!("{}", arr_box_copy.len());
    }

    #[test]
    fn box_leak() {
        let s = gen_string();
        println!("{}", s);

        let s = gen_static_str();
        println!("{}", s);
    }

    fn gen_string() -> String {
        let mut s = String::new();
        s.push_str("hello, world");
        
        s
    }

    fn gen_static_str() -> &'static str{
        let mut s = String::new();
        s.push_str("hello, world");
    
        Box::leak(s.into_boxed_str())
    }

    #[test]
    fn cell_demo() {
        use std::cell::Cell;

        let data = Cell::new(42);

        let ref_alpha = &data;
        let ref_beta = &data;

        println!("inner value: {}", data.get());
        
        ref_alpha.set(89);
        println!("inner value: {}", data.get());

        ref_beta.set(97);
        println!("inner value: {}", data.get());
    }

    #[test]
    fn ref_cell_demo() {
        use std::cell::RefCell;

        struct Immutable {
            inner: RefCell<i32>,
        }

        impl Immutable {
            fn add(&self) {
                let mut inner_mut_ref = self.inner.borrow_mut();
                *inner_mut_ref += 1;
            }
        }

        let data = Immutable {
            inner: RefCell::new(-1),
        };

        *data.inner.borrow_mut() += 1;
        
        let inner_ref = data.inner.borrow();
        println!("inner: {}", inner_ref);
        drop(inner_ref);
        
        let mut inner_mut_ref = data.inner.borrow_mut();
        *inner_mut_ref += 1;
        println!("inner: {}", inner_mut_ref);
        drop(inner_mut_ref);

        println!("inner: {}", data.inner.borrow());
        
        data.add();
        print!("inner: {}", data.inner.borrow());
    }
}
