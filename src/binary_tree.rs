use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub fn tree_sum(root: TreeNodeRef) -> i32 {
    let mut sum = 0i32;
    let mut stack = vec![root];
    while !stack.is_empty() {
        let current: TreeNodeRef = stack.pop().unwrap();
        sum += current.borrow().val;

        if let Some(left) = &current.borrow().left {
            stack.push(left.clone());
        };
        if let Some(right) = &current.borrow().right {
            stack.push(right.clone());
        };
    }
    sum
}

pub fn tree_sum_recur(root: Option<&TreeNodeRef>) -> i32 {
    if let Some(root) = root{
        return root.borrow().val
            + tree_sum_recur(root.borrow().left.as_ref())
            + tree_sum_recur(root.borrow().right.as_ref());
    };
    0
}



#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::binary_tree::{tree_sum, tree_sum_recur, TreeNode};

    #[test]
    fn basic_stack_sum() {
        let r2l = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }));
        let r2r = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));
        let r1l = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(r2l),
            right: None,
        }));
        let root = TreeNode {
            val: 2,
            left: Some(r1l),
            right: Some(r2r)
        };
        assert_eq!(10, tree_sum(Rc::new(RefCell::new(root.clone()))));
        assert_eq!(10, tree_sum_recur(Some(&Rc::new(RefCell::new(root)))));
    }
}