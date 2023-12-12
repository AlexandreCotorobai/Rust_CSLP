use core::ptr::NonNull;

pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<NonNull<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { val: None, next: None }
    }

    pub fn push_left(&mut self, element: T) {
        let node = Box::new(LinkedList { val: Some(element), next: self.next.take() });

        if self.next.is_none() {
            let ptr: NonNull<LinkedList<T>> = Box::leak(node).into();
            self.next = Some(ptr);
        } else {
            let mut ptr: NonNull<LinkedList<T>> = Box::leak(node).into();
            
            unsafe {
                ptr.as_mut().next = self.next;
            }

            self.next = Some(ptr);
        }
    }

    pub fn pop_left(&mut self) -> Option<T> {

        if self.next.is_none() {
            return None;
        }

        let mut ptr: NonNull<LinkedList<T>> = self.next.unwrap();
        
        let one_node: bool = unsafe {
            ptr.as_mut().next.is_none()
        };

        if one_node {
            let next_box = unsafe { Box::from_raw(ptr.as_ptr()) };

            self.next = None;
            next_box.val

        } else {
            let next_of_next = unsafe { ptr.as_mut().next };
            let next_box = unsafe { Box::from_raw(ptr.as_ptr()) };
            self.next = next_of_next;
            next_box.val
        }
    }
}