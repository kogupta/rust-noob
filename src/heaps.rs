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
    }
}
