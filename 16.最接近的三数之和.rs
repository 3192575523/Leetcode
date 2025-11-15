/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 *
 * https://leetcode.cn/problems/3sum-closest/description/
 *
 * algorithms
 * Medium (44.83%)
 * Likes:    1767
 * Dislikes: 0
 * Total Accepted:    667.5K
 * Total Submissions: 1.5M
 * Testcase Example:  '[-1,2,1,-4]\n1'
 *
 * 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
 * 
 * 返回这三个数的和。
 * 
 * 假定每组输入只存在恰好一个解。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [-1,2,1,-4], target = 1
 * 输出：2
 * 解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2)。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [0,0,0], target = 1
 * 输出：0
 * 解释：与 target 最接近的和是 0（0 + 0 + 0 = 0）。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 3 <= nums.length <= 1000
 * -1000 <= nums[i] <= 1000
 * -10^4 <= target <= 10^4
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut res = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let mut sum = nums[i] + nums[left] + nums[right];
                if (sum - target).abs() < (res - target).abs() {
                    res = sum;
                } else if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                }
                if sum == target {
                    return sum;
                }
            }
        }
        res
    }
}
// @lc code=end

