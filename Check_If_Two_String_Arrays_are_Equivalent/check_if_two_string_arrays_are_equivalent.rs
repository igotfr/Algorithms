impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        // Functional 1
        /*let w1 = word1.iter().map(|s: &&str| -> std::slice::Iter<'_, u8> { s.as_bytes().iter() }).flatten();
        let w2 = word2.iter().map(|s: &&str| -> std::slice::Iter<'_, u8> { s.as_bytes().iter() }).flatten();
        w1.eq(w2)*/

        // Functional 2
        /*word1.iter().map(|s: &&str| -> std::str::Chars { s.chars() }).flatten()
            .eq(word2.iter().map(|s: &&str| -> std::str::Chars { s.chars() }).flatten())*/

        // Functional 3
        word1.concat() == word2.concat()
    }
}
