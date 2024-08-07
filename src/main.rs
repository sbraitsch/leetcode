#![allow(unused)]
use structs::solution::Solution;

mod solutions;
mod structs;

fn main() {
    let res = Solution::number_to_words(1001);
    println!("{res:?}");
}
