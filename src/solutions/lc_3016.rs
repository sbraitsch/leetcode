use crate::Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts: Vec<i32> = word
            .chars()
            .into_iter()
            .fold(vec![0; 26], |mut acc, c| {
                acc[((c as u8) - 97) as usize] += 1;
                acc
            })
            .into_iter()
            .filter(|&num| num > 0)
            .collect();

        counts.sort_unstable();
        counts.reverse();
        counts.chunks(8).enumerate().fold(0, |acc, (idx, chunk)| {
            acc + (idx as i32 + 1) * chunk.into_iter().sum::<i32>()
        })
    }
}
