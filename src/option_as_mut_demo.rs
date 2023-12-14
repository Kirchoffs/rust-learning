struct Answer {
    pub val: i32
}

#[cfg(test)]
mod test {
    use super::Answer;

    #[test]
    fn option_demo() {
        let x = Answer { val: 42 };
        let mut x_option: Option<Answer> = Some(x);
        let x_option_ref: Option<&mut Answer>= x_option.as_mut();
        println!("{}", x_option_ref.unwrap().val);
    }
}
