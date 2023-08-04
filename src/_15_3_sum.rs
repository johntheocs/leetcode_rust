/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = Vec::new();
        let len = nums.len();
        //nums.sort();

        for i in 0..len - 2 {
            let (mut j, mut k) = (i + 1, i + 2);

            while j < len - 1 && j < k {
                if nums[i] + nums[j] + nums[k] == 0 {
                    let mut v = vec![nums[i], nums[j], nums[k]];
                    v.sort();
                    if !output.contains(&v) {
                        output.push(v);
                    }
                }

                if k < len - 1 {
                    k += 1;
                } else {
                    j += 1;
                    k = j + 1;
                }
            }
        }

        output
    }
}
// @lc code=end
