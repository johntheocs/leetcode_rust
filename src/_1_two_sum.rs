/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map : HashMap<i32, i32> = HashMap::new();

        for index in 0..nums.len() {
            let value = target - nums[index];

            if map.contains_key(&value) {
                return vec![index as i32, *map.get(&value).unwrap()];
            }

            map.insert(nums[index], index as i32);
        }

        return vec![];
    }
}
// @lc code=end

