impl Solution {
    //pub fn hammingWeight(mut n: u32) -> i32 {
        // Iterative
        /*let mut ans = 0;

        while n != 0 {
            ans += n & 1;
            n >>= 1;
        }

        return ans as i32*/
    //}

    pub fn hammingWeight(n: u32) -> i32 {
        // Functional 1
        return n.count_ones() as i32

        // Functional 2
        //return format!("{:b}", n).chars().filter(|x: &char| -> bool { x == &'1' }).count() as i32
    }
}
