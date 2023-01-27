Given an integer `x`, return `true` *if `x` is a [palindrome,](## "Palindrome\n An integer is a palindrome when it reads the same forward and backward. For example, 121 is a palindrome while 123 is not.") and* `false` *otherwise*.

An integer is a **palindrome** when it reads the same backward as forward.
- For example, `121` is a palindrome while `123` is not.

**Example 1:**

<pre>
<strong>Input:</strong> x = 121
<strong>Output:</strong> true
<strong>Explanation:</strong> 121 reads as 121 from left to right and from right to left.
</pre>

**Example 2:**
<pre>
<strong>Input:</strong> x = -121
<strong>Output:</strong> false
<strong>Explanation:</strong> From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
</pre>

**Example 3:**
<pre>
<strong>Input:</strong> x = 10
<strong>Output:</strong> false
<strong>Explanation:</strong> Reads 01 from right to left. Therefore it is not a palindrome.
</pre>

**Constraints:**

- `-2³¹ <= x <= 2³¹ - 1`

**Follow up:** Could you solve it without converting the integer to a string?

## Hint
Beware of overflow when you reverse the integer.

___
Similar Questions
Palindrome Linked List             Easy
Find Palindrome With Fixed Length  Medium
Strictly Palindromic Number        Medium

___
Related Topics

<button onclick="https://leetcode.com/tag/math/">Math</button>
