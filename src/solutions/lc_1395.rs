use std::cmp::Ordering;
use crate::structs::solution::Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..rating.len() {
            let a = rating[i];
            for j in i + 1..rating.len() {
                let b = rating[j];
                for k in j + 1..rating.len() {
                    let c = rating[k];
                    if c.cmp(&b) == b.cmp(&a) { res += 1; }
                }
            }
        }
        res
    }
}