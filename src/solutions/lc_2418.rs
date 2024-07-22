use crate::structs::solution::Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut tuple: Vec<(String, i32)> = names.into_iter().zip(heights.into_iter()).collect();
        tuple.sort_by_key(|(_, h)| *h);
        tuple.into_iter().rev().map(|(n, _)| n).collect()
    }
}