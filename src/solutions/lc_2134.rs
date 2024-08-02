use crate::structs::solution::Solution;
use std::cmp::min;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let window_size = nums.iter().sum::<i32>() as usize;
        let mut swap: i32 = window_size as i32 - nums[0..window_size].iter().sum::<i32>();
        let mut res = swap;
        for i in 1..nums.len() {
            swap += nums[i - 1] - nums[(i + (window_size - 1)) % nums.len()];
            res = min(res, swap);
        }
        res
    }
}
