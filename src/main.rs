use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let test_case = vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]];
    let res = Solution::lucky_numbers(test_case);
    println!("{res:?}");
}
