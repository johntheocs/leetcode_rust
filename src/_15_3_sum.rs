/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        let mut output: Vec<Vec<i32>> = Vec::new();
        let len = nums.len();

        let mut map = HashMap::<i32, usize>::new();

        for i in 2..len {
            map.insert(nums[i], i);
        }

        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                let offset = 0 - nums[i] - nums[j];
                if map.contains_key(&offset) {
                    let index = *map.get(&offset).unwrap();

                    if index > j {
                        let mut v = vec![nums[i], nums[j], nums[index]];
                        v.sort();
                        if !output.contains(&v) {
                            output.push(v);
                        }
                    }
                }
            }
        }

        output
    }
}
// @lc code=end
