struct LinkedRefCount<T> {
    head: Link<T>,
}

type Link<T> = Option<std::rc::Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkedRefCount<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn preprend(&self, elem: T) -> Self {
        Self {
            head: Some(std::rc::Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|head| head.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.elem)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter { head: &self.head }
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
