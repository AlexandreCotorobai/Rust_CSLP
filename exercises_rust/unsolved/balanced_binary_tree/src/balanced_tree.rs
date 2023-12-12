use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct BinaryTree<T> {
    pub value: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn insert(&mut self, new_value: T) {
        // TODO
    }

    pub fn from(new_values: &[T]) -> Self {
        let (first, rest) = new_values.split_first().unwrap();
        let mut root: BinaryTree<T> = BinaryTree::new(*first);

        for value in rest {
            root.insert(*value);
        }
        root
    }
}
