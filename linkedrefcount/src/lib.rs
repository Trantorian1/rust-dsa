use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

struct LinkedRefCount<T> {
    head: Link<T>,
    size: usize,
}

type Link<T> = Option<std::sync::Arc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedRefCount<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Clone> Clone for LinkedRefCount<T> {
    fn clone(&self) -> Self {
        let mut iter = self.iter().cloned();
        let head = iter.next().and_then(|elem| {
            let mut head = Some(std::sync::Arc::new(Node { elem, next: None }));
            let mut cursor = head.as_mut();

            for elem in iter {
                unsafe {
                    // sadly `get_mut_unchecked` is a nightly-only api as of now
                    // https://doc.rust-lang.org/std/sync/struct.Arc.html#method.get_mut_unchecked
                    let inner =
                        std::sync::Arc::get_mut(cursor.unwrap_unchecked()).unwrap_unchecked();
                    inner.next = Some(std::sync::Arc::new(Node { elem, next: None }));
                    cursor = inner.next.as_mut();
                }
            }

            head
        });

        Self {
            head,
            size: self.size,
        }
    }
}

impl<T: PartialEq> PartialEq for LinkedRefCount<T> {
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

impl<T: Eq> Eq for LinkedRefCount<T> {}

impl<T> Drop for LinkedRefCount<T> {
    fn drop(&mut self) {
        while let Some(head) = self.head.take() {
            if let Ok(mut node) = std::sync::Arc::try_unwrap(head) {
                self.head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<T> Default for LinkedRefCount<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedRefCount<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn preprend(&self, elem: T) -> Self {
        Self {
            head: Some(std::sync::Arc::new(Node {
                elem,
                next: self.head.clone(),
            })),
            size: self.size + 1,
        }
    }

    pub fn tail(&self) -> Self {
        self.head
            .as_ref()
            .map(|head| Self {
                head: head.next.clone(),
                size: self.size - 1,
            })
            .unwrap_or(Self {
                head: None,
                size: 0,
            })
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.elem)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter { head: &self.head }
    }
}

impl<T> FromIterator<T> for LinkedRefCount<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let (head, size) = iter
            .next()
            .map(|elem| {
                let mut head = Some(std::sync::Arc::new(Node { elem, next: None }));
                let mut cursor = head.as_mut();
                let mut size = 1;

                for elem in iter {
                    unsafe {
                        // sadly `get_mut_unchecked` is a nightly-only api as of now
                        // https://doc.rust-lang.org/std/sync/struct.Arc.html#method.get_mut_unchecked
                        let inner =
                            std::sync::Arc::get_mut(cursor.unwrap_unchecked()).unwrap_unchecked();
                        inner.next = Some(std::sync::Arc::new(Node { elem, next: None }));
                        cursor = inner.next.as_mut();
                    }
                    size += 1;
                }

                (head, size)
            })
            .unwrap_or((None, 0));

        Self { head, size }
    }
}

struct Iter<'a, T> {
    head: &'a Link<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.as_ref().map(|head| {
            self.head = &head.next;
            &head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn linked_ref_count_new() {
        assert_eq!(
            LinkedRefCount::<()>::new(),
            LinkedRefCount {
                head: None,
                size: 0
            }
        )
    }

    #[test]
    fn linked_ref_count_prepend() {
        let node_1 = LinkedRefCount::new().preprend(0);
        let node_2_a = node_1.preprend(1);
        let node_2_b = node_1.preprend(2);
        let node_3 = node_2_a.preprend(3);

        assert_eq!(node_1.head.as_ref().unwrap().elem, 0);
        assert_eq!(node_2_a.head.as_ref().unwrap().elem, 1);
        assert_eq!(node_2_b.head.as_ref().unwrap().elem, 2);
        assert_eq!(node_3.head.as_ref().unwrap().elem, 3);

        assert_eq!(node_1.size, 1);
        assert_eq!(node_2_a.size, 2);
        assert_eq!(node_2_b.size, 2);
        assert_eq!(node_3.size, 3);

        assert!(std::sync::Arc::ptr_eq(
            node_2_a.head.as_ref().unwrap().next.as_ref().unwrap(),
            node_1.head.as_ref().unwrap()
        ));

        assert!(std::sync::Arc::ptr_eq(
            node_2_b.head.as_ref().unwrap().next.as_ref().unwrap(),
            node_1.head.as_ref().unwrap()
        ));

        assert!(std::sync::Arc::ptr_eq(
            node_3.head.as_ref().unwrap().next.as_ref().unwrap(),
            node_2_a.head.as_ref().unwrap()
        ));
    }

    #[test]
    fn linked_ref_count_tail() {
        let node_1 = LinkedRefCount::new().preprend(0);
        let node_2_a = node_1.preprend(1);
        let node_2_b = node_1.preprend(2);
        let node_3 = node_2_a.preprend(3);

        assert_eq!(
            node_1.tail(),
            LinkedRefCount {
                head: None,
                size: 0
            }
        );

        let tail_1_a = node_2_a.tail();
        let tail_1_b = node_2_b.tail();
        let tail_2_a = node_3.tail();

        assert_eq!(tail_1_a.size, 1);
        assert_eq!(tail_1_b.size, 1);
        assert_eq!(tail_2_a.size, 2);

        assert!(std::sync::Arc::ptr_eq(
            tail_1_a.head.as_ref().unwrap(),
            node_1.head.as_ref().unwrap()
        ));

        assert!(std::sync::Arc::ptr_eq(
            tail_1_b.head.as_ref().unwrap(),
            node_1.head.as_ref().unwrap()
        ));

        assert!(std::sync::Arc::ptr_eq(
            tail_2_a.head.as_ref().unwrap(),
            node_2_a.head.as_ref().unwrap()
        ));
    }

    // #[test]
    // fn linked_ref_count_head() {
    //     let node_1 = LinkedRefCount::new().preprend(0);
    //     let node_2_a = node_1.preprend(1);
    //     let node_2_b = node_1.preprend(2);
    //     let node_3 = node_2_a.preprend(3);
    //
    //
    // }
}
