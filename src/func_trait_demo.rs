use std::rc::Rc;

trait Hello {
    fn hello(&self);
}

struct Chinese;

impl Hello for Chinese {
    fn hello(&self) {
        println!("Nihao!");
    }
}

fn say_hello_v1<T: Hello>(t: &T) {
    t.hello();
}

fn say_hello_v2(t: &dyn Hello) {
    t.hello();
}

fn say_hello_v3(t: impl Hello) {
    t.hello();
}

fn say_hello_v4(t: Box<dyn Hello>) {
    t.hello();
}

fn say_hello_v5(t: Rc<dyn Hello>) {
    t.hello();
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::Chinese;
    use super::{say_hello_v1, say_hello_v2, say_hello_v3, say_hello_v4, say_hello_v5};

    #[test]
    fn func_trait_demo() {
        let chinese1 = Chinese;
        say_hello_v1(&chinese1);

        let chinese2 = Chinese;
        say_hello_v2(&chinese2);

        let chinese3 = Chinese;
        say_hello_v3(chinese3);

        let chinese4 = Chinese;
        say_hello_v4(Box::new(chinese4));

        let chinese5 = Chinese;
        say_hello_v5(Rc::new(chinese5));
    }
}
