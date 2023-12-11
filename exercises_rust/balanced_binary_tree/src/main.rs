mod balanced_tree;
use balanced_tree::BinaryTree;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::BinaryTree;

    #[test]
    fn create_new_btree() {
        let btree = BinaryTree::new(1);
        assert_eq!(btree.value, 1);
    }

    #[test]
    fn insert_left() {
        let btree = BinaryTree::new(1).left(BinaryTree::new(2));
        if let Some(node) = btree.left {
            assert_eq!(node.value, 2);
        }
        assert_eq!(btree.right, None);
    }

    #[test]
    fn insert_right() {
        let btree = BinaryTree::new(1).right(BinaryTree::new(2));
        if let Some(node) = btree.right {
            assert_eq!(node.value, 2);
        }
        assert_eq!(btree.left, None);
    }

    #[test]
    fn insert() {
        let mut btree = BinaryTree::new(1);
        btree.insert(2);
        btree.insert(3);
        btree.insert(47);
        btree.insert(5);

        assert_eq!(
            btree,
            BinaryTree::new(1)
                .left(
                    BinaryTree::new(2)
                        .left(BinaryTree::new(47))
                        .right(BinaryTree::new(5))
                )
                .right(BinaryTree::new(3))
        );

        btree.insert(7);

        assert_eq!(
            btree,
            BinaryTree::new(1)
                .left(
                    BinaryTree::new(2)
                        .left(BinaryTree::new(47))
                        .right(BinaryTree::new(5))
                )
                .right(BinaryTree::new(3).left(BinaryTree::new(7)))
        )
    }

    #[test]
    fn create_btree_from_array() {
        let btree = BinaryTree::from(&[1, 2, 3, 47, 5, 6]);

        assert_eq!(
            btree,
            BinaryTree::new(1)
                .left(
                    BinaryTree::new(2)
                        .left(BinaryTree::new(47))
                        .right(BinaryTree::new(5))
                )
                .right(BinaryTree::new(3).left(BinaryTree::new(6)))
        )
    }
}