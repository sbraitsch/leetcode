#![allow(unused)]
mod solutions;
mod structs;

use structs::solution::Solution;

fn main() {
    let res = Solution::minimum_pushes(String::from("xyzxyzxyzxyz"));
    println!("{res:?}");
}
