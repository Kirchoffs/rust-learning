#[cfg(test)]
mod test {
    use std::rc::Rc;

    #[test]
    fn rc_demo() {
        let rc_examples = "Rc Examples".to_string();
        {
            println!("--- rc_a is created ---");            
            let rc_a: Rc<String> = Rc::new(rc_examples);
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            println!();
            
            {
                println!("--- rc_a is cloned to rc_b ---");
                let rc_b: Rc<String> = rc_a.clone(); // or Rc::clone(&rc_a);
                println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
                println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
                println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
                println!();
                
                println!("Value of rc_a: {}", rc_a);
                println!("Value of rc_b: {}", rc_b);
                println!();
                
                println!("--- rc_b is dropped out of scope ---");
                println!();
            }
            
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            println!();
            
            println!("--- rc_a is dropped out of scope ---");
        }
    }
}
