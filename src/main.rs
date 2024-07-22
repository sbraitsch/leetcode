use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::sort_people(vec![String::from("Mary"), String::from("John"), String::from("Emma")], vec![180, 165, 170]);
    println!("{res:?}");
}
