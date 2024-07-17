mod lc_1110;

use lc_1110::{TreeNode, del_nodes};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut root_node = TreeNode::new(1);
    let mut n2 = TreeNode::new(2);
    let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(4)));
    n2.left = Some(n4);
    n2.right = Some(n3);
    root_node.left = Some(Rc::new(RefCell::new(n2)));
    let res = del_nodes(Some(Rc::new(RefCell::new(root_node))), vec![2, 3]);
    println!("{res:?}");
}
