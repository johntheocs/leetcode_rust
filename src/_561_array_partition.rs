/*
 * @lc app=leetcode id=561 lang=rust
 *
 * [561] Array Partition
 */

// @lc code=start
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut total = 0;
        for i in 0..sorted_nums.len()/2 {
            total += sorted_nums[i * 2];
        } 

        total
    }
}
// @lc code=end

