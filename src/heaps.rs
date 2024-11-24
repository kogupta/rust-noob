// https://leetcode.com/problems/kth-largest-element-in-an-array/
fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let k = k as usize;

    let mut heap = BinaryHeap::with_capacity(k + 1);
    for n in nums.iter().take(k) {
        heap.push(Reverse(n));
    }

    for n in nums.iter().skip(k) {
        heap.push(Reverse(n));
        let _ = heap.pop();
    }

    *heap.pop().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}
