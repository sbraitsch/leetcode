use crate::structs::solution::Solution;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut restored = vec![vec![0; col_sum.len()]; row_sum.len()];
        for row_idx in 0..row_sum.len() {
            for col_idx in 0..col_sum.len() {
                let min_val = row_sum[row_idx].min(col_sum[col_idx]);
                row_sum[row_idx] -= min_val;
                col_sum[col_idx] -= min_val;
                restored[row_idx][col_idx] = min_val;
            }
        }
        restored
    }
}