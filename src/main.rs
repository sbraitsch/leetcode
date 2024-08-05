#![allow(unused)]
use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::kth_distinct(vec!["d".to_string(), "b".to_string(), "c".to_string(), "b".to_string(), "c".to_string(), "a".to_string()], 2);
    println!("{res:?}");
}
