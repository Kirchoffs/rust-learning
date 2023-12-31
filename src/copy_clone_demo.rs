#[cfg(test)]
mod test {
    #[test]
    fn copy_clone_demo_alpha() {
        #[derive(Debug, Copy, Clone)]
        struct SimpleStructCopyClone {
            a: i32,
            b: i32,
        }
        let x = SimpleStructCopyClone { a: 1, b: 2 };
        let y = x;
        println!("x: {:?}, y: {:?}", x, y);

        #[derive(Debug, Clone)]
        struct SimpleStructClone {
            a: i32,
            b: i32,
        }
        let x = SimpleStructClone { a: 1, b: 2 };
        let y = x.clone();
        println!("x: {:?}, y: {:?}", x, y);
    }

    #[test]
    fn copy_clone_demo_beta() {
        #[derive(Debug)]
        struct SimpleStructClone {
            a: i32,
            b: i32,
        }

        impl Clone for SimpleStructClone {
            fn clone(&self) -> Self {
                Self {
                    a: self.a + 1,
                    b: self.b + 1,
                }
            }
        }

        let x = SimpleStructClone { a: 1, b: 2 };
        let y = x.clone();
        println!("x: {:?}, y: {:?}", x, y);
    }
}
