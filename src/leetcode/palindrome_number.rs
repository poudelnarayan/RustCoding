/*
9. Palindrome Number
Easy
Topics
Companies
Hint
Given an integer x, return true if x is a
palindrome
, and false otherwise.



Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
*/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // If x is negative or ends with 0 (but is not 0), it cannot be a palindrome
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut x = x;
        let mut reversed_half = 0;

        // Reversing the second half of the number
        while x > reversed_half {
            reversed_half = reversed_half * 10 + x % 10;
            x /= 10;
        }

        // For odd length numbers, we can remove the middle digit by reversed_half / 10
        x == reversed_half || x == reversed_half / 10
    }
}

fn main() {
    // Test cases
    let test_cases = vec![121, -121, 10, 0, 12321, 1221];

    for &x in &test_cases {
        let result = Solution::is_palindrome(x);
        println!("Is {} a palindrome? {}", x, result);
    }
}
