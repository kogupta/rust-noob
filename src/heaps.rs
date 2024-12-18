use std::cmp::Ordering;

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

#[derive(Debug, PartialEq, Eq)]
struct Count(i32, i32);

impl PartialOrd for Count {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Count {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}
// https://leetcode.com/problems/top-k-frequent-elements/
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::BinaryHeap;
    use std::collections::HashMap;

    let k = k as usize;

    let mut counts: HashMap<i32, i32> = HashMap::new();

    for n in nums {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut heap: BinaryHeap<Count> = BinaryHeap::with_capacity(k + 1);

    for (n, count) in counts {
        let c = Count(n, count);
        heap.push(c);
        if heap.len() > k {
            heap.pop();
        }
    }

    let mut res: Vec<i32> = Vec::with_capacity(k);
    for Count(n, _count) in heap {
        res.push(n);
    }
    res.reverse();

    res
}

// previous problem - with sort - kinda cheating
fn top_k_frequent2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let k = k as usize;

    let mut counts: HashMap<i32, i32> = HashMap::new();

    for n in nums {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
    }

    // reverse map of count -> item
    let mut x: Vec<(i32, i32)> = counts
        .iter()
        .map(|(item, count)| (-count, *item))
        .collect::<Vec<_>>();
    x.sort_unstable_by_key(|(count, _)| *count);

    x.iter().take(k).map(|(_, item)| *item).collect::<Vec<_>>()
}

// https://leetcode.com/problems/find-k-closest-elements/
fn find_closest_elements(arr: Vec<i32>, k: i32, target: i32) -> Vec<i32> {
    // array is sorted - binary search time
    // solution: has to be a contiguous window of length k
    // key idea: reduce search space from right by k items (width of solution window)

    let k = k as usize;

    let mut lo = 0;
    let mut hi = arr.len() - k;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        // both arr[mid] and arr[mid + k] CANNOT be in solution
        //      -> window length > k
        // so search between this window
        // target > (arr[mid] + arr[mid + k])/2  -> loss of precision due to integer math
        // or, 2*target > arr[mid] + arr[mid + k]
        // or, target - arr[mid] > arr[mid + k] - target

        if target - arr[mid] > arr[mid + k] - target {
            // search right
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    // window found - take next k elements
    arr[lo..lo + k].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }

    #[test]
    fn test_k_most_frequent() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);

        assert_eq!(top_k_frequent2(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent2(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_k_closest() {
        assert_eq!(
            find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            find_closest_elements(vec![1, 1, 2, 3, 4, 5], 4, 1),
            vec![1, 1, 2, 3]
        );
        assert_eq!(
            find_closest_elements(vec![-1, 1, 1, 4, 6, 8, 10], 3, 11),
            vec![6, 8, 10]
        );
        assert_eq!(
            find_closest_elements(vec![-1, 1, 1, 4, 6, 8, 10], 3, 5),
            vec![4, 6, 8]
        );
    }
}
