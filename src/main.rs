use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::second_minimum(6, vec![vec![1, 2], vec![2,3], vec![2,4], vec![2,5], vec![5,6], vec![4,1], vec![5,3]], 457, 953);
    println!("{res:?}");
}
