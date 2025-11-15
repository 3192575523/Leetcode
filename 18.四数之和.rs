/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 *
 * https://leetcode.cn/problems/4sum/description/
 *
 * algorithms
 * Medium (36.88%)
 * Likes:    2116
 * Dislikes: 0
 * Total Accepted:    747.8K
 * Total Submissions: 2M
 * Testcase Example:  '[1,0,-1,0,-2,2]\n0'
 *
 * 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a],
 * nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：
 * 
 * 
 * 0 <= a, b, c, d < n
 * a、b、c 和 d 互不相同
 * nums[a] + nums[b] + nums[c] + nums[d] == target
 * 
 * 
 * 你可以按 任意顺序 返回答案 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,0,-1,0,-2,2], target = 0
 * 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [2,2,2,2,2], target = 8
 * 输出：[[2,2,2,2]]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 200
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = vec![];
        if nums.len() < 4 {
            return res;
        }
        let target = target as i64;
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;   
            }
            for j in i + 1..nums.len() - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;   
                }
                let mut left = j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum: i64 = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;
                    if sum == target {
                        if left == j + 1 || nums[left] != nums[left - 1] {
                            res.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        }
                        left += 1;
                        right -= 1;
                    } else if sum < target {
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }
        return res
    }
}
// @lc code=end

