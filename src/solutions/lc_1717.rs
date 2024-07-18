use crate::structs::solution::Solution;

impl Solution {
    /// 1717
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        return if x > y {
            let (first_pass_score, pass_stack) = remove_pat(s.as_bytes(), (b'a', b'b'), x);
            first_pass_score + remove_pat(&pass_stack, (b'b', b'a'), y).0
        } else {
            let (first_pass_score, pass_stack) = remove_pat(s.as_bytes(), (b'b', b'a'), y);
            first_pass_score + remove_pat(&pass_stack, (b'a', b'b'), x).0
        }
    }
}

fn remove_pat(chars: &[u8], pat: (u8, u8), val: i32) -> (i32, Vec<u8>) {
    let mut stack = vec![];
    let mut acc = 0;
    for &c in chars {
        if stack.last() == Some(&pat.0) && c == pat.1 {
            stack.pop();
            acc += val;
        } else {
            stack.push(c);
        }
    }
    (acc, stack)
}
