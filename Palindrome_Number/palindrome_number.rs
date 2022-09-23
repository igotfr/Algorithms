impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        // Iterative
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false
        }

        let mut reverted_number = 0;
        while x > reverted_number {
            reverted_number = reverted_number * 10 + x % 10;
            x /= 10;
        }

        return x == reverted_number || x == reverted_number / 10
    }

    /*pub fn is_palindrome(x: i32) -> bool {
        // Functional
        return x.to_string().chars().rev().collect::<String>() == x.to_string()
    }*/
}
