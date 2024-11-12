// https://leetcode.com/problems/valid-triangle-number/

pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    let mut count = 0;
    nums.sort_unstable();


    for idx in (1..nums.len()).rev() {
        let mut left: usize = 0;
        let mut right = idx - 1;

        while left < right {
            let side = nums[idx];
            if nums[left] + nums[right] > side {
                count += right - left;
                right -= 1;
            } else {
                left += 1;
            }
        }
    }

    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_triangle_numbers() {
        assert_eq!(triangle_number(vec![11, 4, 9, 6, 15, 18]), 10);
        assert_eq!(triangle_number(vec![2, 2, 3, 4]), 3);
        assert_eq!(triangle_number(vec![4, 2, 3, 4]), 4);
    }
}