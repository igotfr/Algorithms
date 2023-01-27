Write an algorithm to determine if a number `n` is happy.

A **happy number** is a number defined by the following process:

- Starting with any positive integer, replace the number by the sum of the squares of its digits.
- Repeat the process until the number equals 1 (where it will stay), or it **loops endlessly in a cycle** which does not include 1.
- Those numbers for which this process **ends in 1** are happy.

Return `true` *if* `n` *is a happy number, and* `false` *if not*.

**Example 1:**
<pre>
<strong>Input:</strong> n = 19  
<strong>Output:</strong> true
<strong>Explanation:</strong>
1² + 9² = 82
8² + 2² = 68
6² + 8² = 100
1² + 0² + 0² = 1
</pre>

**Example 2:**
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> false
</pre>

**Constraints:**
- <code>1 <= n <= 2<sup>31</sup> - 1</code>

___
Similar Questions

Linked List Cycle Easy

Add Digits Easy

Ugly Number Easy

Sum of Digits of String After Convert Easy

Minimum Addition to Make Integer Beautiful Medium

Smallest Value After Replacing With Sum of Prime Factors Medium

Count the Digits That Divide a Number Easy

___
Related Topics

<button name="button" onclick="https://leetcode.com/tag/hash-table/">Hash Table</button>

<button name="button" onclick="https://leetcode.com/tag/math/">Math</button>

<button name="button" onclick="https://leetcode.com/tag/two-pointers/">Two Pointers</button>
