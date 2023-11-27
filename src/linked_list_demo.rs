#[cfg(test)]
mod test {
    use std::{rc::Rc, cell::RefCell};

    #[derive(Debug)]
    struct SimpleNode {
        id: usize,
        next: Option<Rc<SimpleNode>>,
    }

    impl SimpleNode {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                next: None,
            }
        }

        pub fn update_next(&mut self, next: Rc<SimpleNode>) {
            self.next = Some(next);
        }

        pub fn get_next(&self) -> Option<Rc<SimpleNode>> {
            self.next
                .as_ref()
                .map(|v| v.clone())
        }
    }

    #[test]
    fn simple_node_test() {
        let mut node1 = SimpleNode::new(1);
        let mut node2 = SimpleNode::new(2);
        let mut node3 = SimpleNode::new(3);
        let node4 = SimpleNode::new(4);

        node3.update_next(Rc::new(node4));
        node1.update_next(Rc::new(node3));
        node2.update_next(node1.get_next().unwrap());
        println!("node1: {:?}, node2: {:?}", node1, node2);

        // Below will cause a panic because node3 is not a mutable reference:
        // let node5 = SimpleNode::new(5);
        // let node3 = node1.get_next().unwrap();
        // node3.update_next(Rc::new(node5));
        // println!("node1: {:?}, node2: {:?}", node1, node2);
    }

    #[derive(Debug)]
    struct Node {
        id: usize,
        next: Option<Rc<RefCell<Node>>>,
    }

    impl Node {
        pub fn new(id: usize) -> Self {
            Self {
                id,
                next: None,
            }
        }
    
        pub fn update_next(&mut self, downstream: Rc<RefCell<Node>>) {
            self.next = Some(downstream);
        }
    
        pub fn get_next(&self) -> Option<Rc<RefCell<Node>>> {
            self.next.as_ref().map(|v| v.clone())
        }
    }

    #[test]
    fn node_test() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);

        node3.update_next(Rc::new(RefCell::new(node4)));
        node1.update_next(Rc::new(RefCell::new(node3)));
        node2.update_next(node1.get_next().unwrap());
        println!("node1: {:?}, node2: {:?}", node1, node2);

        let node5 = Node::new(5);
        let node3 = node1.get_next().unwrap();
        node3.borrow_mut().update_next(Rc::new(RefCell::new(node5)));
        println!("node1: {:?}, node2: {:?}", node1, node2);
    }
}
