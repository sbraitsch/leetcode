use std::rc::Rc;
use std::cell::RefCell;
use crate::structs::tree_node::TreeNode;

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

    fn build_path(p1: &[bool], p2: &[bool]) -> String {
        let mut divergence =  0;
        for (c1, c2) in p1.iter().zip(p2.iter()) {
            if c1 != c2 {
                break;
            }
            divergence += 1;
        }
        let mut res = String::new();
        for _ in 0..p1.len() - divergence {
            res.push('U');
        }

        for b in &p2[divergence..]{
            if *b {
                res.push('R');
            } else {
                res.push('L');
            }
        }
        res
    }

    build_path(&to_start, &to_dest)
}
