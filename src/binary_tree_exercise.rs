use std::cell::RefCell;
use std::ptr::null;
use std::rc::Rc;

/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Subtree<T> {
    fn insert(&mut self, val: T) {
        match &mut self.0 {
            None => {
                self.0 = Some(Box::new(Node {
                    value: val,
                    left: Subtree(None),
                    right: Subtree(None),
                }));
            }
            Some(node) => {
                if (node.value > val) {
                    node.right.insert(val)
                } else if node.value < val {
                    node.left.insert(val)
                }
            }
        }
    }
}


/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

// Implement `new`, `insert`, `len`, and `has`.

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root: Subtree(None) }
    }
    // think about binary search tree?
    fn insert(&mut self, val: T) {
        self.root.insert(val)
    }

    fn len(&self) -> i32 {
        recur_sum(&self.root)
    }

    fn has(&self, val: &T) -> bool {
        recur_has(&self.root, val)
    }
}

fn recur_sum<T: Ord>(root: &Subtree<T>) -> i32 {
    if root.0.is_none() {
        return 0;
    }
    let mut sum = 1;
    let left_node = &root.0.as_ref().unwrap().left;
    let right_node = &root.0.as_ref().unwrap().right;
    sum = sum + recur_sum(left_node) + recur_sum(right_node);
    sum
}

fn recur_has<T: Ord>(root: &Subtree<T>, val: &T) -> bool {
    if root.0.is_none() {
        return false;
    }
    if (root.0.as_ref().unwrap().value.eq(val)) {
        return true;
    }
    recur_has(&root.0.as_ref().unwrap().left, val) || recur_has(&root.0.as_ref().unwrap().right, val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}