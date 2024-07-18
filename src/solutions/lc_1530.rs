use std::cell::RefCell;
use std::rc::Rc;
use crate::structs::tree_node::TreeNode;

pub fn solve() {
    println!("No local testcase implemented.")
}
pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    let rt = root.unwrap();
    let mut leafs = vec![];
    let mut path  = vec![];
    let mut open = vec![(0, rt.clone(), true)];

    while let Some((idx, rc_node, is_right)) = open.pop() {
        if idx >= path.len() { path.push(is_right) } else { path[idx] = is_right }
        let node = rc_node.borrow();

        let mut is_leaf = true;
        if let Some(left) = &node.left {
            open.push((idx + 1, left.clone(), false));
            is_leaf = false;
        }
        if let Some(right) = &node.right {
            open.push((idx + 1, right.clone(), true));
            is_leaf = false;
        }
        if is_leaf {
            let p = &path[..(idx+1)];
            leafs.push(p.to_vec());
        }
    }

    let mut good_leafs = 0;
    for i in 0..(leafs.len() - 1) {
        for j in (i + 1)..(leafs.len()) {
            if distance_between(&leafs[i], &leafs[j]) <= distance {
                good_leafs += 1;
            }
        }
    }
    good_leafs
}

fn distance_between(l1: &[bool], l2: &[bool]) -> i32 {
    let mut divergence =  0;
    for (c1, c2) in l1.iter().zip(l2.iter()) {
        if c1 != c2 {
            break;
        }
        divergence += 1;
    }
    ((l1.len() - divergence) + (l2.len() - divergence)) as i32
}