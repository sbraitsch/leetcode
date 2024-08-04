use crate::Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut sums = (0..n as usize).fold(vec![], |mut v, i| {
            let mut sum = 0;
            (i..n as usize).for_each(|j| {
                sum += nums[j];
                v.push(sum);
            });
            v
        });
        sums.sort_unstable();
        sums.into_iter()
            .skip(left as usize - 1)
            .take((right - left + 1) as usize).fold(0, |acc, v| (acc + v) % 1_000_000_007i32)
    }
}

