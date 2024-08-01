use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::count_seniors(
        vec![
            String::from("7868190130M7522"), String::from("5303914400F9211"), String::from("9273338290F4010")
        ],
    );
    println!("{res:?}");
}
