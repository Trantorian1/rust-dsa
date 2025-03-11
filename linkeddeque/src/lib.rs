pub struct LinkedDeque<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

type Link<T> = Option<std::rc::Rc<std::cell::RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Drop for LinkedDeque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> LinkedDeque<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let node = std::rc::Rc::new(std::cell::RefCell::new(Node::new(elem)));

        match self.head.take() {
            Some(head) => {
                node.borrow_mut().next = Some(std::rc::Rc::clone(&head));
                head.borrow_mut().prev = Some(std::rc::Rc::clone(&node));
                self.head = Some(node);
            }
            None => {
                self.head = Some(std::rc::Rc::clone(&node));
                self.tail = Some(node);
            }
        };

        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            match head.borrow_mut().next.take() {
                Some(next) => {
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                }
                None => {
                    self.tail = None;
                }
            };

            self.size -= 1;

            std::rc::Rc::try_unwrap(head)
                .ok()
                .unwrap()
                .into_inner()
                .elem
        })
    }
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            next: None,
            prev: None,
        }
    }
}
