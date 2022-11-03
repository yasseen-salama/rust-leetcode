/**
 * [121] Best Time to Buy and Sell Stock
 *
 * You are given an array `prices` where `prices[i]` is the price of a given stock on the `i^th` day.
 * You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
 * Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return `0`.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: prices = [7,1,5,3,6,4]
 * Output: 5
 * Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
 * Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: prices = [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transactions are done and the max profit = 0.
 * 
 *  
 * Constraints:
 * 
 *     `1 <= prices.length <= 10^5`
 *     `0 <= prices[i] <= 10^4`
 * 
 * Problem link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
 * Discuss link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

use std::cmp; 
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut currentMax = 0;
        let mut rightPointer = 1;
        let mut leftPointer = 0;
        while rightPointer < prices.len() {
            if prices[leftPointer] < prices[rightPointer]{
                let profit =  prices[rightPointer] - prices[leftPointer]; 
                currentMax = cmp::max(profit, currentMax);
            }
            else {
                leftPointer = rightPointer; 
            }
            rightPointer = rightPointer+1
        }
        currentMax
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
    }
}
