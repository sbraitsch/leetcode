mod lc_2196;

use lc_2196::create_binary_tree;

fn main() {
    let descriptions = vec![
        vec![85, 82, 1],
        vec![74, 85, 1],
        vec![39, 70, 0],
        vec![82, 38, 1],
        vec![74, 39, 0],
        vec![39, 13, 1],
    ];
    create_binary_tree(descriptions);
}
