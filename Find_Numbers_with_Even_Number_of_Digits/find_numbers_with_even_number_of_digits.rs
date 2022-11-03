impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        // Functional
        return nums.iter()
            //.filter(|&x: &&i32| -> bool { x.to_string().len() % 2 == 0 })
            .filter(|&&x: &&i32| -> bool { (x as f32).log10() as u32 % 2 != 0 })
            .count() as i32

        // Iterative
        /*let mut count = 0;

        for &n in nums.iter() {
            //if n.to_string().len() % 2 == 0 {
            if (n as f32).log10() as u32 % 2 != 0 {
                count += 1;
            }
        }

        return count*/
    }
}
