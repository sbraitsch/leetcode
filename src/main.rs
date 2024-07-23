use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]);
    println!("{res:?}");
}
