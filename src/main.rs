use structs::solution::Solution;
mod solutions;
mod structs;
fn main() {
    let res = Solution::min_height_shelves(
        vec![
            vec![1, 1],
            vec![2, 3],
            vec![2, 3],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 2],
        ],
        4,
    );
    println!("{res:?}");
}
