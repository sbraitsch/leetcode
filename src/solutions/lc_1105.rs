use std::cmp::{max, min};
use crate::structs::solution::Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let arr_len = books.len() + 1;
        let mut height_map = vec![0; arr_len];

        for i in 1..arr_len {
            let mut thickness = 0;
            let mut height = 0;

            for j in (0..i).rev() {
                thickness += books[j][0];
                if thickness > shelf_width {
                    break;
                }
                height = max(height, books[j][1]);
                height_map[i] = if height_map[i] == 0 {
                    height_map[j] + height
                } else {
                    min(height_map[i], height_map[j] + height)
                }
            }
        }
        height_map[books.len()]
    }
}