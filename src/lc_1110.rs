use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

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
    let mut del_map: HashMap<i32, bool> = HashMap::new();
    for delete in &to_delete {
        del_map.insert(*delete, true);
    }

    if !del_map.contains_key(&actual_root.borrow().val) {
        roots.push(Some(actual_root));
    }

    while let Some(node) = to_visit.pop() {
        let mut n = node.borrow_mut();

        if del_map.contains_key(&n.val) {
            if let Some(left) = n.left.take() {
                if !del_map.contains_key(&left.borrow().val) {
                    roots.push(Some(left.clone()));
                }
                to_visit.push(left.clone());
            }
            if let Some(right) = n.right.take() {
                if !del_map.contains_key(&right.borrow().val) {
                    roots.push(Some(right.clone()));
                }
                to_visit.push(right.clone());
            }
        } else {
            if let Some(left) = &n.left {
                if del_map.contains_key(&left.borrow().val) {
                    to_visit.push(n.left.take().expect("Hmm"));
                } else {
                    to_visit.push(left.clone());
                }
            }
            if let Some(right) = &n.right {
                if del_map.contains_key(&right.borrow().val) {
                    to_visit.push(n.right.take().expect("Hmm"));
                } else {
                    to_visit.push(right.clone());
                }
            }
        }
    };
    roots
}
