/**
 * [217] Contains Duplicate
 *
 * Given an integer array `nums`, return `true` if any value appears at least twice in the array, and return `false` if every element is distinct.
 *  
 * Example 1:
 * Input: nums = [1,2,3,1]
 * Output: true
 * Example 2:
 * Input: nums = [1,2,3,4]
 * Output: false
 * Example 3:
 * Input: nums = [1,1,1,3,3,4,3,2,4,2]
 * Output: true
 *  
 * Constraints:
 * 
 *     `1 <= nums.length <= 10^5`
 *     `-10^9 <= nums[i] <= 10^9`
 * 
 * Problem link: https://leetcode.com/problems/contains-duplicate/
 * Discuss link: https://leetcode.com/problems/contains-duplicate/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hashSet: HashSet<i32> = HashSet::new();
        for i in 0..nums.len(){
            if !hashSet.contains(&nums[i]) {
                hashSet.insert(nums[i]);
            } else {
                return true;
            }
        }
        false 
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert_eq!(Solution::contains_duplicate(vec![1]), false);
        assert_eq!(Solution::contains_duplicate(vec![]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }
}
