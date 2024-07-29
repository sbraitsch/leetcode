use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::num_teams(vec![2,5,3,4,1]);
    println!("{res:?}");
}
