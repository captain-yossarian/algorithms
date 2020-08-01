type Leaf<T: Ord + Clone> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T: Ord + Clone> {
    pub value: T,
    pub left: Leaf<T>,
    pub right: Leaf<T>,
}

impl<T: Ord + Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn wrap(value: T) -> Leaf<T> {
        Some(Box::new(Node::new(value)))
    }

    pub fn add(&mut self, value: T) {
        if value > self.value {
            match self.left {
                Some(ref mut node) => node.add(value),
                None => self.left = Node::wrap(value),
            }
        } else {
            match self.right {
                Some(ref mut node) => node.add(value),
                None => self.right = Node::wrap(value),
            }
        }
    }

    pub fn find(&mut self, value: T) -> bool {
        if value == self.value {
            return true;
        }

        if value > self.value {
            match self.left {
                Some(ref mut node) => node.find(value),
                None => false,
            }
        } else {
            match self.right {
                Some(ref mut node) => node.find(value),
                None => false,
            }
        }
    }
}
