
use std::cmp::Ordering;

// https://leetcode.com/problems/search-in-rotated-sorted-array/description/
pub fn search(xs: Vec<i32>, target: i32) -> i32 {
    const BINARY_SEARCH_THRESHOLD: usize = 4;

    if xs.is_empty() {
        -1
    } else if xs.len() <= BINARY_SEARCH_THRESHOLD {
        linear_search(&xs, target)
    } else {
        let lo = 0;
        let hi = xs.len() - 1;

        let first = xs[lo];
        let last = xs[hi];

        if last > first {
            // no rotation
            binary_search(&xs, lo, hi, target)
        } else {
            let min_index = find_min_index(&xs);

            match target.cmp(&first) {
                Ordering::Less => binary_search(&xs, min_index, hi, target),
                Ordering::Equal => 0,
                Ordering::Greater => binary_search(&xs, 0, min_index - 1, target),
            }
        }
    }
}

fn find_min_index(xs: &[i32]) -> usize {
    let mut lo = 0;
    let mut hi = xs.len() - 1;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        let middle = xs[mid];
        // println!("lo: {lo}, hi: {hi}, mid: {mid}, middle: {middle}");

        if middle > xs[hi] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    lo
}

fn binary_search(xs: &[i32], from: usize, to: usize, target: i32) -> i32 {
    let mut lo = from as i32;
    let mut hi = to as i32;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let middle = xs[mid as usize];

        // println!("lo: {lo}, hi: {hi}, mid: {mid}, middle: {middle}");

        match target.cmp(&middle) {
            Ordering::Less => {
                hi = mid - 1;
            }
            Ordering::Equal => {
                return mid;
            }
            Ordering::Greater => {
                lo = mid + 1;
            }
        }
    }

    -1
}

fn linear_search(xs: &[i32], target: i32) -> i32 {
    for (idx, x) in xs.iter().enumerate() {
        if *x == target {
            return idx as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let xs = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(find_min_index(&xs), 4);

        assert_eq!(binary_search(&xs, 0, 3, 4), 0);
        assert_eq!(binary_search(&xs, 0, 3, 5), 1);
        assert_eq!(binary_search(&xs, 0, 3, 6), 2);
        assert_eq!(binary_search(&xs, 0, 3, 7), 3);

        assert_eq!(binary_search(&xs, 4, 6, 0), 4);
        assert_eq!(binary_search(&xs, 4, 6, 1), 5);
        assert_eq!(binary_search(&xs, 4, 6, 2), 6);

        assert_eq!(search(xs.clone(), 0), 4);
        assert_eq!(search(xs.clone(), 1), 5);
        assert_eq!(search(xs.clone(), 2), 6);
        assert_eq!(search(xs.clone(), 4), 0);
        assert_eq!(search(xs.clone(), 5), 1);
        assert_eq!(search(xs.clone(), 6), 2);
        assert_eq!(search(xs.clone(), 7), 3);

        assert_eq!(search(xs.clone(), 3), -1);

        assert_eq!(find_min_index(&[4, 5, 1, 2, 3]), 2);

        assert_eq!(search(vec![1,2,3,6,7,9], 0), -1);
        assert_eq!(search(vec![1,2,3,6,7,9], 1), 0);
        assert_eq!(search(vec![1,2,3,6,7,9], 2), 1);
        assert_eq!(search(vec![1,2,3,6,7,9], 3), 2);
        assert_eq!(search(vec![1,2,3,6,7,9], 4), -1);
        assert_eq!(search(vec![1,2,3,6,7,9], 5), -1);
        assert_eq!(search(vec![1,2,3,6,7,9], 6), 3);
        assert_eq!(search(vec![1,2,3,6,7,9], 7), 4);
        assert_eq!(search(vec![1,2,3,6,7,9], 9), 5);
    }

}

