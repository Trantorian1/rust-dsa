pub struct LinkedDeque<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    size: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    // prev: Link<T>,
}

// impl<T> Drop for LinkedDeque<T> {
//     fn drop(&mut self) {
//         while self.pop_front().is_some() {}
//     }
// }

impl<T> LinkedDeque<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: std::ptr::null_mut(),
            size: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node::new(elem));

        let new_tail = match self.tail.take() {
            Some(tail) => {
                tail.next = Some(node);
                tail.next.as_deref_mut()
            }
            None => {
                self.head = Some(node);
                self.head.as_deref_mut()
            }
        };

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            match head.next {
                Some(next) => self.head = Some(next),
                None => self.tail = None,
            }

            head.elem
        })
    }

    // pub fn push_front(&mut self, elem: T) {
    //     let node = std::rc::Rc::new(std::cell::RefCell::new(Node::new(elem)));
    //
    //     match self.head.take() {
    //         Some(head) => {
    //             node.borrow_mut().next = Some(std::rc::Rc::clone(&head));
    //             head.borrow_mut().prev = Some(std::rc::Rc::clone(&node));
    //             self.head = Some(node);
    //         }
    //         None => {
    //             self.head = Some(std::rc::Rc::clone(&node));
    //             self.tail = Some(node);
    //         }
    //     };
    //
    //     self.size += 1;
    // }
    //
    // pub fn push_back(&mut self, elem: T) {
    //     let node = std::rc::Rc::new(std::cell::RefCell::new(Node::new(elem)));
    //
    //     match self.tail.take() {
    //         Some(tail) => {
    //             node.borrow_mut().prev = Some(std::rc::Rc::clone(&tail));
    //             tail.borrow_mut().next = Some(std::rc::Rc::clone(&node));
    //             self.tail = Some(node);
    //         }
    //         None => {
    //             self.head = Some(std::rc::Rc::clone(&node));
    //             self.tail = Some(node);
    //         }
    //     };
    //
    //     self.size += 1;
    // }
    //
    // pub fn pop_front(&mut self) -> Option<T> {
    //     self.head.take().map(|head| {
    //         match head.borrow_mut().next.take() {
    //             Some(next) => {
    //                 next.borrow_mut().prev = None;
    //                 self.head = Some(next);
    //             }
    //             None => {
    //                 self.tail = None;
    //             }
    //         };
    //
    //         self.size -= 1;
    //
    //         std::rc::Rc::try_unwrap(head)
    //             .ok()
    //             .unwrap()
    //             .into_inner()
    //             .elem
    //     })
    // }
    //
    // pub fn pop_back(&mut self) -> Option<T> {
    //     self.tail.take().map(|tail| {
    //         match tail.borrow_mut().prev.take() {
    //             Some(prev) => {
    //                 prev.borrow_mut().next = None;
    //                 self.tail = Some(prev);
    //             }
    //             None => {
    //                 self.head = None;
    //             }
    //         };
    //
    //         self.size -= 1;
    //
    //         std::rc::Rc::try_unwrap(tail)
    //             .ok()
    //             .unwrap()
    //             .into_inner()
    //             .elem
    //     })
    // }
    //
    // pub fn peek_front(&self) -> Option<std::cell::Ref<T>> {
    //     self.head
    //         .as_ref()
    //         .map(|head| std::cell::Ref::map(head.borrow(), |head| &head.elem))
    // }
    //
    // pub fn peek_back(&self) -> Option<std::cell::Ref<T>> {
    //     self.tail
    //         .as_ref()
    //         .map(|tail| std::cell::Ref::map(tail.borrow(), |head| &head.elem))
    // }
    //
    // pub fn peek_front_mut(&mut self) -> Option<std::cell::RefMut<T>> {
    //     self.head
    //         .as_ref()
    //         .map(|head| std::cell::RefMut::map(head.borrow_mut(), |head| &mut head.elem))
    // }
    //
    // pub fn peek_back_mut(&mut self) -> Option<std::cell::RefMut<T>> {
    //     self.tail
    //         .as_ref()
    //         .map(|tail| std::cell::RefMut::map(tail.borrow_mut(), |head| &mut head.elem))
    // }
}

// impl<T> IntoIterator for LinkedDeque<T> {
//     type Item = T;
//     type IntoIter = IntoIter<T>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         IntoIter(self)
//     }
// }
//
// pub struct IntoIter<T>(LinkedDeque<T>);
//
// impl<T> Iterator for IntoIter<T> {
//     type Item = T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.pop_front()
//     }
// }
//
// impl<T> DoubleEndedIterator for IntoIter<T> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         self.0.pop_back()
//     }
// }

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            next: None,
            prev: None,
        }
    }
}
