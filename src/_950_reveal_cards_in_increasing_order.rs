/*
 * @lc app=leetcode id=950 lang=rust
 *
 * [950] Reveal Cards In Increasing Order
 */

// @lc code=start
impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;

        let mut sorted_deck = deck.clone();
        sorted_deck.sort_by(|a, b| b.cmp(a));
        let len = sorted_deck.len();

        let mut output: VecDeque<i32> = VecDeque::new();
        for i in 0..len {
            if output.len() > 1 {
                let value = output.pop_back().unwrap();
                output.push_front(value);
            }

            output.push_front( sorted_deck[ i]);
        }

        Vec::from(output)
    }
}
// @lc code=end

