use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let counts = arr.iter().fold(HashMap::new(), |mut acc, s| {
            *acc.entry(s).or_insert(0) += 1;
            acc
        });
        
        arr.iter().filter(|s| counts[s] == 1).nth((k - 1) as usize).unwrap_or(&String::new()).to_owned()
    }
}
