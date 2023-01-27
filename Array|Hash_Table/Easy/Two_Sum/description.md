Given an array of integers `nums` and an integer `target`, return *indices of the two numbers such that they add up to `target`*.

You may assume that each input would have **exactly one solution**, and you may not use the *same* element twice.

You can return the answer in any order.

**Example 1:**
<pre>
<b>Input:</b> nums = [2,7,11,15], target = 9
<b>Output:</b> [0,1]
<b>Explanation:</b> Because nums[0] + nums[1] == 9, we return [0, 1].
</pre>

**Example 2:**
<pre>
<b>Input:</b> nums = [3,2,4], target = 6
<b>Output:</b> [1,2]
</pre>

**Example 3:**
<pre>
<b>Input:</b> nums = [3,3], target = 6
<b>Output:</b> [0,1]
</pre>

**Constraints:**
- `2 <= nums.length <= 104`
- `-109 <= nums[i] <= 109`
- `-109 <= target <= 109`
- **Only one valid answer exists.**

**Follow-up:** Can you come up with an algorithm that is less than `O(n²)` time complexity?

## Hint
1. A really brute force way would be to search for all possible pairs of numbers but that would be too slow. Again, it's best to try out brute force solutions for just for completeness. It is from these brute force solutions that you can come up with optimizations.
2. So, if we fix one of the numbers, say x, we have to scan the entire array to find the next number y which is value - x where value is the input parameter. Can we change our array somehow so that this search becomes faster?
3. The second train of thought is, without changing the array, can we use additional space somehow? Like maybe a hash map to speed up the search?

___
Similar Questions

3Sum Medium

4Sum Medium

Two Sum II - Input Array Is Sorted Medium

Two Sum III - Data structure design Easy

Subarray Sum Equals K Medium

Two Sum IV - Input is a BST Easy

Two Sum Less Than K Easy

Max Number of K-Sum Pairs Medium

Count Good Meals Medium

Count Number of Pairs With Absolute Difference K Easy

Number of Pairs of Strings With Concatenation Equal to Target Medium

Find All K-Distant Indices in an Array Easy

First Letter to Appear Twice Easy

Number of Excellent Pairs Hard

Number of Arithmetic Triplets Easy

Node With Highest Edge Score Medium

Check Distances Between Same Letters Easy

Find Subarrays With Equal Sum Easy

Largest Positive Integer That Exists With Its Negative Easy

Number of Distinct Averages Easy

___
Related Topics

<button name="button" onclick="https://leetcode.com/tag/array/">Array</button>

<button name="button" onclick="https://leetcode.com/tag/hash-table/">Hash Table</button>
