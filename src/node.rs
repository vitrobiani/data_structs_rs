pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }

    pub fn with_next(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}
