use core::ptr::NonNull;

pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<NonNull<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            val: None,
            next: None,
        }
    }

    pub fn push_left(&mut self, element: T) {
        // TODO
    }

    pub fn pop_left(&mut self) -> Option<T> {
        // TODO
    }
}
