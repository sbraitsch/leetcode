use std::rc::Rc;
use std::cell::RefCell;
use crate::structs::solution::Solution;
use crate::structs::tree_node::TreeNode;

impl Solution {
    /// 2096
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        let mut path: Vec<bool> = vec![];
        let mut to_visit = vec![(0, root.unwrap(), true)];
        let mut to_start = vec![];
        let mut to_dest = vec![];

        while let Some((idx, node, is_right)) = to_visit.pop() {
            if idx >= path.len() { path.push(is_right) } else { path[idx] = is_right }
            let node = node.borrow();
            if node.val == start_value {
                let p = &path[..(idx+1)];
                to_start = p.to_vec();
            } else if node.val == dest_value {
                let p = &path[..(idx+1)];
                to_dest = p.to_vec();
            }
            if !to_start.is_empty() && !to_dest.is_empty() {
                break;
            }

            if let Some(left) = &node.left {
                to_visit.push((idx + 1, left.clone(), false));
            }
            if let Some(right) = &node.right {
                to_visit.push((idx + 1, right.clone(), true));
            }

        }

        let mut divergence =  0;
        for (c1, c2) in to_start.iter().zip(to_dest.iter()) {
            if c1 != c2 {
                break;
            }
            divergence += 1;
        }
        let mut res = String::new();
        for _ in 0..&to_start.len() - divergence {
            res.push('U');
        }

        for b in &to_dest[divergence..]{
            if *b {
                res.push('R');
            } else {
                res.push('L');
            }
        }
        res
    }
}

