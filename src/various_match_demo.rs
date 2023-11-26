#[cfg(test)]
mod test {
    struct IntWrapper {
        value: i32,
    }

    #[test]
    fn match_box() {
        let x = IntWrapper { value: 42 };

        let y = Box::new(x);
        match y.as_ref() {
            IntWrapper { value } => {
                println!("{value}");
            },
        }

        match *y {
            IntWrapper { value } => {
                println!("{value}");
            },
        }
    }
}
