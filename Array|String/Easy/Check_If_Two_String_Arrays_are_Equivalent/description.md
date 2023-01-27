Given two string arrays `word1` and `word2`, return `true` *if the two arrays **represent** the same string, and* `false` *otherwise.*

A string is **represented** by an array if the array elements concatenated **in order** forms the string.

**Example 1:**
<pre>
<b>Input:</b> word1 = ["ab", "c"], word2 = ["a", "bc"]
<b>Output:</b> true
<b>Explanation:</b>
word1 represents string "ab" + "c" -> "abc"
word2 represents string "a" + "bc" -> "abc"
The strings are the same, so return true.
</pre>

**Example 2:**
<pre>
<b>Input:</b> word1 = ["a", "cb"], word2 = ["ab", "c"]
<b>Output:</b> false
</pre>

**Example 3:**
<pre>
<b>Input:</b> word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
<b>Output:</b> true
</pre>

**Constraints:**
- `1 <= word1.length, word2.length <= 10³`
- `1 <= word1[i].length, word2[i].length <= 10³`
- `1 <= sum(word1[i].length), sum(word2[i].length) <= 10³`
- `word1[i]` and `word2[i]` consist of lowercase letters.

## Hint
1. Concatenate all strings in the first array into a single string in the given order, the same for the second array.
2. Both arrays represent the same string if and only if the generated strings are the same.

___
Similar Questions

Check if an Original String Exists Given Two Encoded Strings Hard

___
Related Topics

<button name="button" onclick="https://leetcode.com/tag/array/">Array</button>

<button name="button" onclick="https://leetcode.com/tag/string/">String</button>
