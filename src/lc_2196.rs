use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
            right: None,
        }
    }
}

pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut tree_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
    let mut parent_map: HashMap<i32, u8> = HashMap::new();
    for desc in descriptions {
        let parent = desc[0];
        let child = desc[1];
        let left = desc[2] == 1;

        parent_map.entry(parent).or_insert(0);
        *parent_map.entry(child).or_insert(0) += 1;

        let p = tree_map
            .entry(parent)
            .or_insert(Rc::new(RefCell::new(TreeNode::new(parent))))
            .clone();
        let c = tree_map
            .entry(child)
            .or_insert(Rc::new(RefCell::new(TreeNode::new(child))))
            .clone();

        if left {
            p.borrow_mut().left = Some(c);
        } else {
            p.borrow_mut().right = Some(c);
        }
    }
    let root = parent_map.into_iter().find(|(_, v)| *v == 0).unwrap().0;
    let result = Rc::clone(&tree_map[&root]);
    Some(result)
}
