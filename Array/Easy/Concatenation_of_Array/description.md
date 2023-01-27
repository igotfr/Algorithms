Given an integer array `nums` of length `n`, you want to create an array `ans` of length `2n` where `ans[i] == nums[i]` and `ans[i + n] == nums[i]` for `0 <= i < n` (**0-indexed**).

Specifically, `ans` is the **concatenation** of two `nums` arrays.

Return *the array* `ans`.

**Example 1:**
<pre>
<b>Input:</b> nums = [1,2,1]
<b>Output:</b> [1,2,1,1,2,1]
<b>Explanation:</b> The array ans is formed as follows:
- ans = [nums[0],nums[1],nums[2],nums[0],nums[1],nums[2]]
- ans = [1,2,1,1,2,1]
</pre>

**Example 2:**
<pre>
<b>Input:</b> nums = [1,3,2,1]
<b>Output:</b> [1,3,2,1,1,3,2,1]
<b>Explanation:</b> The array ans is formed as follows:
- ans = [nums[0],nums[1],nums[2],nums[3],nums[0],nums[1],nums[2],nums[3]]
- ans = [1,3,2,1,1,3,2,1]
</pre>

**Constraints:**
- `n == nums.length`
- `1 <= n <= 1000`
- `1 <= nums[i] <= 1000`

## Hint
Build an array of size 2 * n and assign num[i] to ans[i] and ans[i + n]

___
Related Topics

<button name="button" onclick="https://leetcode.com/tag/math/">Math</button>
