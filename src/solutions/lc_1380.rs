use crate::structs::solution::Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut lucky = vec![];
        matrix
            .iter()
            .map(|row| row.iter().enumerate().map(|(i, v)| (*v, i)).min().unwrap())
            .max()
            .inspect(|(val, c_id)| {
                if !matrix.iter().any(|row| row[*c_id] > *val) {
                    lucky.push(*val);
                }
            });
        lucky
    }
}
