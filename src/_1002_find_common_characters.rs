/*
 * @lc app=leetcode id=1002 lang=rust
 *
 * [1002] Find Common Characters
 */

// @lc code=start
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
      let char_to_index = |c: char|(c as u8 - 'a' as u8) as usize;
      let index_to_string = |c: usize|char::from(c as u8 + 'a' as u8).to_string();

      let mut output : Vec<String> = vec![];
      let mut min_freq : Vec<u8> = vec![std::u8::MAX; 26];

      for word in words {
        let mut char_freq : Vec<u8> = vec![0; 26];
        for c in word.chars() {
            char_freq[char_to_index(c)] += 1;
        }

        for index in 0..min_freq.len() {
            min_freq[index] = std::cmp::min(min_freq[index], char_freq[index]);
        }
      }

      for index in 0..min_freq.len() {
        while min_freq[index] > 0 {
            output.push(index_to_string(index));
            min_freq[index] -= 1;
        }
      }

      output
    }
}
// @lc code=end

