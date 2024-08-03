use crate::structs::solution::Solution; 

impl Solution {
    pub fn can_be_equal(target: Vec<i32>,arr: Vec<i32>) -> bool {
        let mut counter = vec![0; 1001];
        target.into_iter().for_each(|n| counter[n as usize] += 1);
        arr.into_iter().try_for_each(|n| {
            let count = &mut counter[n as usize];
            if *count == 0 {
                Err(())
            } else {
                *count -= 1;
                Ok(())
            }
        }).is_ok()
    }
}
