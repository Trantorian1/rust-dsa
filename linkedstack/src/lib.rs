use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

pub struct LinkedStack<T> {
    head: Link<T>,
    size: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Clone> Clone for LinkedStack<T> {
    fn clone(&self) -> Self {
        let mut iter = self.iter().cloned();
        let head = iter.next().and_then(|elem| {
            let mut head = Some(Box::new(Node { elem, next: None }));
            let mut cursor = head.as_mut();

            for elem in iter {
                unsafe {
                    let inner = cursor.unwrap_unchecked();
                    inner.next = Some(Box::new(Node { elem, next: None }));
                    cursor = inner.next.as_mut();
                }
            }

            head
        });

        LinkedStack {
            head,
            size: self.size,
        }
    }
}

impl<T: PartialEq> PartialEq for LinkedStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && self
                .iter()
                .zip(other.iter())
                .fold_while(
                    true,
                    |_, (a, b)| {
                        if a == b { Continue(true) } else { Done(false) }
                    },
                )
                .into_inner()
    }
}

impl<T: Eq> Eq for LinkedStack<T> {}

impl<T> Drop for LinkedStack<T> {
    /// This drops each node _iteratively_ since we are replacing `node.next` with [`None`],
    /// meaning that the drop implementation for [`Box`] will not be called on it. Instead, each
    /// node is dropped when it goes out of scope at the end of each `while` iteration. Neat!
    fn drop(&mut self) {
        while let Some(mut node) = self.head.take() {
            self.head = node.next.take();
        }
    }
}

impl<T> Default for LinkedStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter {
            node: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        IterMut {
            node: self.head.as_deref_mut(),
        }
    }
}

impl<T> IntoIterator for LinkedStack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(LinkedStack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|node| {
            self.node = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    node: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod est {
    use dsa_util::DropCounter;

    use super::*;

    #[test]
    fn linked_stack_new() {
        assert_eq!(
            LinkedStack::<()>::new(),
            LinkedStack {
                head: None,
                size: 0
            }
        );
    }

    #[test]
    fn linked_stack_push_pop() {
        let mut linked_stack = LinkedStack::new();

        assert_eq!(linked_stack.pop(), None);
        assert_eq!(linked_stack.len(), 0);
        assert!(linked_stack.is_empty());

        for n in 0..10 {
            linked_stack.push(n);
        }

        assert_eq!(linked_stack.len(), 10);
        assert!(!linked_stack.is_empty());

        for n in (0..10).rev() {
            assert_eq!(linked_stack.pop(), Some(n));
        }

        assert_eq!(linked_stack.pop(), None);
        assert_eq!(linked_stack.len(), 0);
        assert!(linked_stack.is_empty())
    }

    #[test]
    fn linked_stack_peek() {
        let mut linked_stack = LinkedStack::new();
        assert_eq!(linked_stack.peek(), None);

        for n in 0..10 {
            linked_stack.push(n);
            assert_eq!(linked_stack.peek(), Some(&n));
        }
    }

    #[test]
    fn linked_stack_peek_mut() {
        let mut linked_stack = LinkedStack::new();

        for n in 0..10 {
            linked_stack.push(n);
            *linked_stack.peek_mut().unwrap() += 1;
            assert_eq!(linked_stack.peek(), Some(n + 1).as_ref());
        }
    }

    #[test]
    fn linked_stack_debug() {
        let mut linked_stack = LinkedStack::new();
        for n in 0..10 {
            linked_stack.push(n);
        }

        assert_eq!(
            &format!("{linked_stack:?}"),
            "[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]"
        );
    }

    #[test]
    fn linked_stack_clone() {
        let mut linked_stack = LinkedStack::new();
        for n in 0..10 {
            linked_stack.push(n)
        }

        let mut linked_stack_clone = linked_stack.clone();
        let mut head = linked_stack_clone.head.as_mut();

        while let Some(node) = head {
            node.elem += 1;
            head = node.next.as_mut();
        }

        for n in (0..10).rev() {
            assert_eq!(linked_stack.pop(), Some(n));
            assert_eq!(linked_stack_clone.pop(), Some(n + 1));
        }
    }

    #[test]
    fn linked_stack_drop() {
        let mut linked_stack = LinkedStack::new();
        let rc = std::rc::Rc::default();

        for n in 0..10 {
            linked_stack.push(DropCounter::new(&rc, vec![n]));
        }

        drop(linked_stack);
        assert_eq!(rc.get(), 10);
    }

    #[test]
    fn linked_stack_iter_simple() {
        let mut linked_stack = LinkedStack::new();
        for n in 0..10 {
            linked_stack.push(n);
        }

        let mut iter = linked_stack.iter();
        for n in (0..10).rev() {
            assert_eq!(iter.next(), Some(n).as_ref());
        }

        assert_eq!(iter.next(), None);
        drop(iter);

        for n in (0..10).rev() {
            assert_eq!(linked_stack.pop(), Some(n));
        }
        assert_eq!(linked_stack.pop(), None);
    }

    #[test]
    fn linked_stack_iter_mut() {
        let mut linked_stack = LinkedStack::new();
        for n in 0..10 {
            linked_stack.push(n);
        }

        for node in linked_stack.iter_mut() {
            *node += 1;
        }

        for n in (0..10).rev() {
            assert_eq!(linked_stack.pop(), Some(n + 1));
        }
        assert_eq!(linked_stack.pop(), None);
    }

    #[test]
    fn linked_stack_into_iter() {
        let mut linked_stack = LinkedStack::new();
        for n in 0..10 {
            linked_stack.push(n);
        }

        let mut iter = linked_stack.into_iter();
        for n in (0..10).rev() {
            assert_eq!(iter.next(), Some(n));
        }
        assert_eq!(iter.next(), None);
    }
}
