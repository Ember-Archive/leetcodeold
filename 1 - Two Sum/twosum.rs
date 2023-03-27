/* 
 * Author: Ember Hext
 * Problem: Two Sum
 * Link: https://leetcode.com/problems/two-sum/
 * 
 * This solution uses a HashMap, storing the number as the key
 * and the index as the value. This order is used because it allows
 * us to check if the number needed to sum to the target has already
 * been inserted into the HashMap.
 * 
 * This solution works in O(n) time.
 */

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut prev_nums: HashMap<&i32, i32> =                                 // The HashMap to store the previous values
            HashMap::with_capacity(nums.len());
                                                                                // we iterate over the vec of numbers, using enumerate
        for (index, value) in nums.iter().enumerate() {                         // so we can retrieve both their index and value,
            match prev_nums.get(&(target - value)) {                            // check if there is a match,
                Some(&diff_index) => return vec![index as i32, diff_index],     // and return the result if there is,
                None => prev_nums.insert(&value, index as i32),                 // otherwise we add the current number to the HashMap and iterate again
            };
        }

        unreachable!("The problem specification indicates there will always be a working pair.");
    }
}