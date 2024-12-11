use crate::node::Node;

pub struct LinkedList<T> {
    pub len: i32,
    pub head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn add(&mut self, data: T) {
        let new_node = Box::new(Node::with_next(data, self.head.take()));
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn get(&self, i: i32) -> Option<&Node<T>> {
        let mut n: Option<&Node<T>> = self.head.as_deref();
        let mut j: i32 = 0;
        while let Some(node) = n {
            if j == i {
                return Some(node);
            }
            n = node.next.as_deref();
            j += 1;
        }
        None
    }

    pub fn remove(&mut self, i: i32) -> Option<T> {
        if i < 0 {
            return None;
        }

        if i == 0 {
            if let Some(mut head_node) = self.head.take() {
                self.head = head_node.next.take();
                return Some(head_node.data);
            }
        }

        let mut cur = self.head.as_mut();
        let mut j: i32 = 0;
        while let Some(node) = cur {
            if j == i - 1 {
                if let Some(mut node_to_remove) = node.next.take() {
                    node.next = node_to_remove.next.take();
                    return Some(node_to_remove.data);
                }
                return None;
            }
            cur = node.next.as_mut();
            j += 1;
        }
        None
    }

    pub fn print(&self, print_func: fn(&T)) {
        let mut cur: Option<&Node<T>> = self.head.as_deref();
        while let Some(node) = cur {
            print_func(&node.data);
            cur = node.next.as_deref();
        }
    }
}
