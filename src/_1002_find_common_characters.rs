/*
 * @lc app=leetcode id=1002 lang=rust
 *
 * [1002] Find Common Characters
 */

// @lc code=start
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        use std::collections::{HashSet,HashMap};

        let mut map : HashMap::<String, i8> = HashMap::<String, i8>::new();
        let words_len = words.len() as i8;

        for word in words {
            let mut char_set: HashSet::<char> = HashSet::<char>::new();
            for c in word.chars() {
                if char_set.contains(&c) {
                    continue;
                } else {
                    char_set.insert(c);
                }
            }
            for ch in char_set {
                let c = ch.to_string();
                let count = map.get(&c).unwrap_or(&0) + 1;
                map.insert(c, count);
            }
        }

        map.iter().filter(|(_key, value) | **value == words_len as i8).map(|(key, _value)| key.clone()).collect()
    }
}
// @lc code=end

