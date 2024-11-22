// https://leetcode.com/problems/find-the-duplicate-number/
fn find_duplicate(nums: Vec<i32>) -> i32 {
    // this is a problem of linked linked cycle finding
    // each items in array is pointing to next node index
    // find cycle - then find cycle/loop start index/value

    let mut slow = 0;
    let mut fast = 0;
    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;

        if slow == fast {
            break;
        }
    }

    // loop found - pointers on intersection point
    // move a pointer to head - then move both in step 1
    slow = 0;
    while slow != fast {
        slow = nums[slow] as usize;
        fast = nums[fast] as usize;
    }

    slow as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(find_duplicate(vec![3, 3, 3, 3]), 3);
    }
}
