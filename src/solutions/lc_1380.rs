use crate::structs::solution::Solution;
use std::cmp::max;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut lucky = Vec::with_capacity(1);
        let mut cmax = vec![0; matrix[0].len()];
        matrix
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(i, v)| {
                        cmax[i] = max(cmax[i], *v);
                        (*v, i)
                    })
                    .min()
                    .unwrap()
            })
            .max()
            .inspect(|(val, c_id)| {
                if cmax[*c_id] == *val {
                    lucky.push(*val);
                }
            });
        lucky
    }
}
