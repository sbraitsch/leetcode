use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::restore_matrix(vec![5,7,10], vec![8,6,8]);
    println!("{res:?}");
}
