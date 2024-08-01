use crate::structs::solution::Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.into_iter().filter(|s| s.as_str()[11..13].parse::<u8>().is_ok_and(|num| num > 60)).count() as i32
    }
}