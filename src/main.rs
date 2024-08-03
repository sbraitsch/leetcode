use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::can_be_equal(vec![1,2,3,4], vec![2,4,1,3]);
    println!("{res:?}");
}
