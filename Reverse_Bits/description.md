Reverse bits of a given 32 bits unsigned integer.

**Note:**

- Note that in some languages, such as Java, there is no unsigned integer type. In this case, both input and output will be given as a signed integer type. They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.

- In Java, the compiler represents the signed integers using [2's complement notation](https://en.wikipedia.org/wiki/Two%27s_complement). Therefore, in **Example 2** above, the input represents the signed integer `-3` and the output represents the signed integer `-1073741825`.

**Example 1:**
<pre>
<b>Input:</b> n = 00000010100101000001111010011100
<b>Output:</b>    964176192 (00111001011110000010100101000000)
<b>Explanation:</b> The input binary string <b>00000010100101000001111010011100</b> represents the unsigned integer 43261596,
  so return 964176192 which its binary representation is <b>00111001011110000010100101000000</b>.
</pre>

**Example 2:**
<pre>
<b>Input:</b> n = 11111111111111111111111111111101
<b>Output:</b>   3221225471 (10111111111111111111111111111111)
<b>Explanation:</b> The input binary string <b>11111111111111111111111111111101</b> represents the unsigned integer 4294967293,
  so return 3221225471 which its binary representation is <b>10111111111111111111111111111111</b>.
</pre>

**Constraints:**
- The input must be a **binary string** of length `32`

**Follow up:** If this function is called many times, how would you optimize it?
