use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
 
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

}

pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let actual_root = root.unwrap();
    let mut to_visit = vec![actual_root.clone()];
    let mut roots = vec![];
    if !to_delete.contains(&actual_root.borrow().val) {
        roots.push(actual_root);
    }

    while let Some(node) = to_visit.pop() {
        let mut n = node.borrow_mut();

        if to_delete.contains(&n.val) {
            if let Some(left) = n.left.take() {
                if !to_delete.contains(&left.borrow().val) {
                    roots.push(left.clone());
                }
                to_visit.push(left.clone());
            }
            if let Some(right) = n.right.take() {
                if !to_delete.contains(&right.borrow().val) {
                    roots.push(right.clone());
                }
                to_visit.push(right.clone());
            }
        } else {
            if let Some(left) = &n.left {
                if to_delete.contains(&left.borrow().val) {
                    to_visit.push(n.left.take().expect("Hmm"));
                } else {
                    to_visit.push(left.clone());
                }
            }
            if let Some(right) = &n.right {
                if to_delete.contains(&right.borrow().val) {
                    to_visit.push(n.right.take().expect("Hmm"));
                } else {
                    to_visit.push(right.clone());
                }
            }
        }
    };
    roots.iter().map(|r| Some(r.clone())).collect()
}
