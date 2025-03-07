pub struct DropCounter<T> {
    _val: T,
    counter: std::rc::Rc<std::cell::Cell<usize>>,
}

impl<T> Drop for DropCounter<T> {
    fn drop(&mut self) {
        let count = self.counter.get() + 1;
        self.counter.set(count);
    }
}

impl<T> DropCounter<T> {
    pub fn new(cell: &std::rc::Rc<std::cell::Cell<usize>>, val: T) -> Self {
        Self {
            _val: val,
            counter: std::rc::Rc::clone(cell),
        }
    }

    pub fn count(&self) -> usize {
        self.counter.get()
    }
}
