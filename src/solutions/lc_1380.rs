use crate::structs::solution::Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let col_max: Vec<i32> = (0..matrix[0].len())
            .map(|c| matrix.iter().map(|row| row[c]).max().unwrap())
            .collect();

        matrix
            .iter()
            .map(|row| *row.iter().min().unwrap())
            .filter(|v| col_max.contains(v))
            .collect()
    }
}
