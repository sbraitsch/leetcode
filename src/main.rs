mod solutions;
mod structs;
fn main() {
    let test_case = vec![vec![7, 8], vec![1,2]];
    structs::solution::Solution::lucky_numbers(test_case);
}
