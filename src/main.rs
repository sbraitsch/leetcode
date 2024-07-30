use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::minimum_deletions(String::from("bbaaaaabb"));
    println!("{res:?}");
}
