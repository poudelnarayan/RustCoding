/* Two Sum
Easy
Topics
Companies
Hint
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

 

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
 */

 use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Initialize a HashMap to store the value and its index
    let mut map = HashMap::new();

    /*
    Iterate through the list of numbers.
    For each number, calculate the complement (i.e., target - num).
    Check if the complement is already in the HashMap.
    If it is, return the indices of the current number and its complement.
    If not, insert the current number and its index into the HashMap.
    */
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        map.insert(num, i);
    }

    // Return an empty vector if no solution is found
    vec![]
}

fn main() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    println!("{:?}", two_sum(nums1, target1)); // Output: [0, 1]

    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    println!("{:?}", two_sum(nums2, target2)); // Output: [1, 2]

    let nums3 = vec![3, 3];
    let target3 = 6;
    println!("{:?}", two_sum(nums3, target3)); // Output: [0, 1]
}
