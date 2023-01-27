impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Brute Force
        /*let nums_size = nums.len();

        for z in 0..nums_size {
            for y in z + 1..nums_size {
                if nums[z] == target - nums[y] {
                    return Vec::from([z as i32, y as i32])
                }
            }
        }
        unreachable!();*/

        // Two-pass Hash Table
        /*let mut b: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

        for i in 0..nums.len() {
            b.insert(nums[i], i);
        }
        for i in 0..nums.len() {
            let complement: i32 = target - nums[i];

            if let Some(&j) = b.get(&complement) {
                if i != j {
                    return Vec::from([i as i32, j as i32])
                }
            }
        }
        unreachable!();*/

        // One-pass Hash Table
        let b: std::collections::HashMap<i32, usize> = nums.iter().enumerate().map(|(j, x): (usize, &i32)| (target - x, j)).collect();
        for (i, a) in nums.iter().enumerate() {
            if let Some(&j) = b.get(a) {
                if i != j {
                    return Vec::from([i as i32, j as i32])
                }
            }
        }
        unreachable!();
    }
}
