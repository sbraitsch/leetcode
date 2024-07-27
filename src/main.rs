use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::minimum_cost( String::from("aabbddccbc"), String::from("abbbaabaca"), vec!['a', 'b', 'c', 'b', 'a', 'd'], vec!['d', 'c', 'b', 'd', 'b', 'b'], vec![3,8,7,6,7,10]);
    println!("{res:?}");
}
