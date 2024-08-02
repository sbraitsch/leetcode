use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::min_swaps(vec![1]);
    println!("{res:?}");
}
