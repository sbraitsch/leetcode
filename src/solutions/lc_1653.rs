use crate::Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut b_count = 0;
        let mut deletions = 0;

        for ch in s.chars() {
            if ch == 'a' {
                if b_count > 0 {
                    deletions += 1;
                    b_count -= 1;
                }
            } else if ch == 'b' {
                b_count += 1;
            }
        }

        deletions
    }
}
