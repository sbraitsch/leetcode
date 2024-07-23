use crate::Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut freq = [0; 201];
        nums.iter().for_each(|num| freq[(100 + num) as usize] += 1);
        nums.sort_unstable_by_key(|&num| (freq[(100 + num) as usize], Reverse(num)));
        nums
    }
}
