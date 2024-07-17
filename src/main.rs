mod lc_2096;

use lc_2096::{TreeNode, get_directions};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut root_node = TreeNode::new(1);
    root_node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    get_directions(Some(Rc::new(RefCell::new(root_node))), 2, 1);
}
