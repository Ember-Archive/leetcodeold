/* 
 * Author: Ember Hext
 * Problem: Two Sum
 * Link: https://leetcode.com/problems/longest-substring-without-repeating-characters/
 * 
 * This solution uses a sliding window approach to find the
 * longest substring without repeating characters. It stores the
 * count of each character in a vector, and a pointer to both the
 * left and right index of the current substring. The while loop
 * removes characters from the left of the substring until there
 * are no repeating characters left, and the length of each valid
 * 
 * The vector method over a HashMap works similarly but with
 * slightly better performance.
 * 
 * This solution works in O(n) time.
 */

use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest_substring: usize = 0;
        let mut left_index: usize = 0;
        let mut right_index: usize = 0;
        let mut count: Vec<i32> = vec![0; 256];
        
        for c in s.bytes().map(|byte| usize::from(byte)) {
            count[c] += 1;

            while count[c] > 1 {
                count[s.bytes.nth(left_index).unwrap() as usize] -= 1;
                left_index += 1;
            }

            longest_substring = cmp::max(longest_substring, right_index - left_index + 1);
            right_index += 1;
        }

        return longest_substring as i32;
    }
}