# Solution
## Approach 1: Concatenate and Compare
### Intuition
We are given two arrays of strings. We need to find out if both arrays represent the same string, for this, the two strings formed by concatenating the strings in the respective arrays must be equal.

In this approach, we will do as the problem says. We will find the string represented by each array and then check if the two strings are equal or not. To find the string represented by the array, we just need to append all the strings present in it to one another in the same order they are present in the array.

### Algorithm
1. Iterate over strings present in the array `word1`, append each string to a string `word1Combined`.
2. Iterate over strings present in the array `word2`, append each string to a string `word2Combined`.
3. Compare the above strings and return true if both are the same, otherwise return false.

Note: In Java, we must use `StringBuilder` as strings are Immutable in Java.

### Implementation
#### C++
```c++
class Solution {
public:
    bool arrayStringsAreEqual(vector<string>& word1, vector<string>& word2) {
        // Creates a new string by combining all the strings in word1.
        string word1Combined;
        for (string s : word1) {
            word1Combined += s;
        }
        // Creates a new string by combining all the strings in word2.
        string word2Combined;
        for (string s : word2) {
            word2Combined += s;
        }
        // Returns true if both string are the same.
        return word1Combined == word2Combined;
    }
};
```
#### Java
```java
class Solution {
    public boolean arrayStringsAreEqual(String[] word1, String[] word2) {
        // Creates a new string by combining all the strings in word1.
        StringBuilder word1Combined = new StringBuilder();
        for (String s : word1) {
            word1Combined.append(s);
        }
        // Creates a new string by combining all the strings in word2.
        StringBuilder word2Combined = new StringBuilder();
        for (String s : word2) {
            word2Combined.append(s);
        }
        // Returns true if both string are the same.
        return word1Combined.compareTo(word2Combined) == 0;
    }
}
```

### Complexity Analysis
Here `N` is the number of strings in the list and `K` is the maximum length of a string in it.

- Time complexity: `O(N ∗ K)`
    - We iterate over each string in the arrays to append them. This cost us `O(N * K)` as we traversed over each character of the string to perform an append operation.
    - In the end, the comparison between the two strings also takes `O(N * K)` time.
    - Hence, the total time complexity is equal to `O(N * K)`.
- Space complexity: `O(N ∗ K)`

    We need to have two strings to store the strings represented by the arrays. Therefore, the total space complexity is equal to `O(N * K)`.

## Approach 2: Two Pointers
### Intuition
If we observe closely, we can notice that we need to compare each character at corresponding positions in the two arrays. Also, this comparison needs to be continuous over the strings in the array i.e., we can just assume the whole array to be a single string (what we did in the previous approach).

We can keep two pointers, one pointing to the first character of the first string in the array `word1` and the other pointing to the first character of the first string in the array `word2`. Then we will compare the characters at these indices and can return false if they aren't the same, otherwise, we will increment both pointers. Now it might be possible that after incrementing the pointers one or both of them have exhausted the whole string and are now pointing to the non-existing indices. We need to move to the next string in the array in such cases. Hence we need to have two more pointers that will be pointing to the strings in the two array lists.

### Algorithm
1. Initialize `word1Pointer` and `word2Pointer` to `0`. These pointers will be pointing to the current string in the array `word1` and `word2` respectively.

2. Initialize `string1Pointer` and `string2Pointer` to `0`. These pointers will be pointing to the current characters in the strings pointed by the above two pointers.

3. While we still have strings to iterate over in both the lists:

    - If the character at `string1Pointer` in the string at index `word1Pointer` in the list `word1` isn't equal to the character at `string2Pointer` in the string at index `word2Pointer` in the list `word2`, then return false. Otherwise, increment both the string pointers i.e., `string1Pointer` and `string2Pointer` to check the next characters.

    - If the pointer `string1Pointer` has reached the end of string then reset it to zero and increment the word pointer `word1Pointer`.

    - If the pointer `string2Pointer` has reached the end of string then reset it to zero and increment the word pointer `word2Pointer`.

4. Return true if the `word1Pointer` and `word2Pointer` has reached the end of array. This is important as it might happen that one of the list has no more strings but the other one still has some and in that case we must return false.

![image](blob:https://leetcode.com/c1b8271a-d0e8-43c2-b01e-215b454c93cb)


### Implementation
#### C++
```c++
class Solution {
public:
    bool arrayStringsAreEqual(vector<string>& word1, vector<string>& word2) {
        // Pointers to mark the current word in the given two lists.
        int word1Pointer = 0, word2Pointer = 0;
        // Pointers to mark the character in the string pointed by the above pointers.
        int string1Pointer = 0, string2Pointer = 0;
        
        // While we still have the string in any of the two given lists.
        while (word1Pointer < word1.size() && word2Pointer < word2.size()) {
            // If the characters at the two string are same, increment the string pointers
            // Otherwise return false.
            if (word1[word1Pointer][string1Pointer++] != word2[word2Pointer][string2Pointer++]) {
                return false;
            }
            // If the string pointer reaches the end of string in the list word1, 
            // Move to the next string in the list and, reset the string pointer to 0.
            if (string1Pointer == word1[word1Pointer].size()) {
                word1Pointer++;
                string1Pointer = 0;
            }
            // If the string pointer reaches the end of string in the list word2, 
            // Move to the next string in the list and, reset the string pointer to 0.
            if (string2Pointer == word2[word2Pointer].size()) {
                word2Pointer++;
                string2Pointer = 0;
            }
        }
        // Strings in both the lists should be traversed.
        return word1Pointer == word1.size() && word2Pointer == word2.size();
    }
};
```
#### Java
```java
class Solution {
    public boolean arrayStringsAreEqual(String[] word1, String[] word2) {
        // Pointers to mark the current word in the given two lists.
        int word1Pointer = 0, word2Pointer = 0;
        // Pointers to mark the character in the string pointed by the above pointers.
        int string1Pointer = 0, string2Pointer = 0;
        
        // While we still have the string in any of the two given lists.
        while (word1Pointer < word1.length && word2Pointer < word2.length) {
            // If the characters at the two string are same, increment the string pointers
            // Otherwise return false.
            if (word1[word1Pointer].charAt(string1Pointer++) != 
                word2[word2Pointer].charAt(string2Pointer++)) {
                return false;
            }
            // If the string pointer reaches the end of string in the list word1, 
            // Move to the next string in the list and, reset the string pointer to 0.
            if (string1Pointer == word1[word1Pointer].length()) {
                word1Pointer++;
                string1Pointer = 0;
            }
            // If the string pointer reaches the end of string in the list word2, 
            // Move to the next string in the list and, reset the string pointer to 0.
            if (string2Pointer == word2[word2Pointer].length()) {
                word2Pointer++;
                string2Pointer = 0;
            }
        }
        // Strings in both the lists should be traversed.
        return word1Pointer == word1.length && word2Pointer == word2.length;
    }
}
```

### Complexity Analysis
Here `N` is the number of strings in the list and `K` is the maximum length of a string in it.

- Time complexity: `O(N ∗ K)`

    We are iterating over each character of every string present in the given lists and hence the total time complexity will be equal to `O(N ∗ K)`.

- Space complexity: `O(1)`

    We don't require any extra space as we are not building any string as we did in the previous approach and therefore the total space complexity is equal to `O(1)`.
