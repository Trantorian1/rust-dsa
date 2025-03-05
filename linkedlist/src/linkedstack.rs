pub struct LinkedStack {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl Drop for LinkedStack {
    /// This drops each node _iteratively_ since we are replacing `node.next` with [`Link::Empty`],
    /// meaning that the drop implementation for [`Box`] will not be called on it. Instead, each
    /// node is dropped when it goes out of scope at the end of each `while` iteration. Neat!
    fn drop(&mut self) {
        while let Link::More(mut node) = std::mem::replace(&mut self.head, Link::Empty) {
            self.head = std::mem::replace(&mut node.next, Link::Empty);
        }
    }
}

impl LinkedStack {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
