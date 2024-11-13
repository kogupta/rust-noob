// https://leetcode.com/problems/valid-triangle-number/

fn triangle_number(mut nums: Vec<i32>) -> i32 {
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

// https://leetcode.com/problems/sort-colors/
fn sort_colors(nums: &mut Vec<i32>) {
    let mut zeroes = 0;
    let mut ones = 0;

    for n in &mut *nums {
        if *n == 0 {
            zeroes += 1;
        } else if *n == 1 {
            ones += 1;
        }
        // rest are 2s
    }

    for idx in 0..zeroes {
        nums[idx] = 0;
    }

    for idx in zeroes..(zeroes + ones) {
        nums[idx] = 1;
    };

    for idx in (zeroes + ones)..nums.len() {
        nums[idx] = 2;
    };
}

// https://leetcode.com/problems/trapping-rain-water/
fn trap(height: Vec<i32>) -> i32 {
    // pointers
    let mut left = 0;
    let mut right = height.len() - 1;

    // max heights
    let mut left_max = height[left];
    let mut right_max = height[right];

    let mut count = 0;

    while left < right {
        // move towards higher height
        if left_max < right_max {
            left += 1;
            if height[left] > left_max {
                left_max = height[left];
            } else {
                count += left_max - height[left];
            }
        } else {
            right -= 1;
            if height[right] > right_max {
                right_max = height[right];
            } else {
                count += right_max - height[right];
            }
        }
    }

    count
}

// https://leetcode.com/problems/product-of-array-except-self/
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(nums.len());

    let mut prod = 1;

    // prefix products
    for n in &nums {
        res.push(prod);
        prod *= n;
    }

    prod = 1;
    for (idx, n) in nums.iter().enumerate().rev() {
        res[idx] *= prod;
        prod *= n;
    }

    res
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

    #[test]
    fn test_colours() {
        let mut xs = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut xs);
        assert_eq!(xs, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_water_trapping() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(trap(vec![3, 4, 1, 2, 2, 5, 1, 0, 2]), 10);
    }

    #[test]
    fn test_product() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }
}