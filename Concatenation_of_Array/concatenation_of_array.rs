impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        // Functional 1
        //return [&*nums, &*nums].concat()

        // Functional 2
        return nums.repeat(2)
    }

    /*pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        // Iterative 1
        /*let n = nums.len();
        for i in 0..n {
            nums.push(nums[i]);
        }*/
        return nums

        // Functional 1
        nums.extend(nums.clone());
        return nums
    }*/
}
