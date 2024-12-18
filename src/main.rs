use rand::seq::IndexedRandom;
use std::collections::HashMap;

mod bin_search;
mod boofus;
mod doofus;
mod dp;
mod epi_strings;
mod heaps;
mod intervals;
mod linked_list;
mod rotate_sorted_array;
mod sliding_windows;
mod stacks;
mod valid_triangle_numbers;

fn main() {
    println!("Hello, world!");
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
    println!("{:?}", two_sum(vec![3, 3], 6));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut kv: HashMap<i32, usize> = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        kv.insert(*value, index);
    }

    for (index, value) in nums.iter().enumerate() {
        let req = target - value;
        if let Some(idx) = kv.get(&req) {
            if *idx != index {
                return vec![index as i32, *idx as i32];
            }
        }
    }

    vec![]
}

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;

    nums.sort_unstable();

    let mut result: Vec<(i32, i32, i32)> = Vec::new();

    let p = nums.len() - 2;

    for idx in 0..p {
        if idx > 0 && nums[idx] == nums[idx - 1] {
            continue;
        }
        if nums[idx] > 0 {
            break;
        }

        let mut left = idx + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[idx] + nums[left] + nums[right];

            match sum.cmp(&0) {
                Ordering::Equal => {
                    result.push((nums[idx], nums[left], nums[right]));

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    left += 1;

                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    right -= 1;
                }
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
            }
        }
    }

    result.iter().map(|(a, b, c)| vec![*a, *b, *c]).collect()
}

// https://leetcode.com/problems/container-with-most-water/
fn container_most_water(heights: Vec<i32>) -> i32 {
    use std::cmp::{max, min};

    let mut max_area = 0;
    let mut left = 0;
    let mut right = heights.len() - 1;

    while left < right {
        let height = min(heights[left], heights[right]);
        let width = (right - left) as i32;

        max_area = max(max_area, height * width);

        // prune lower heights
        while left < right && heights[left] <= height {
            left += 1;
        }
        while left < right && heights[right] <= height {
            right -= 1;
        }
    }

    max_area
}

// https://leetcode.com/problems/unique-substrings-in-wraparound-string
fn find_substring_in_wraparound_string(s: String) -> i32 {
    use std::cmp::max;

    // value at each index denote the length of the substring
    // example: zab, array looks like: z:1, a:2, b:3
    // example: baz, array looks like: z:1, a:1, b:1 since the substrings are not contiguous
    // example: abz, array looks like: z:1, a:1, b:2 since "ab" is contiguous
    // example: abcrt, array looks like: a:1, b:2, c: 3, r:1, t:1
    let mut substring_lengths: [usize; 26] = [0; 26];

    let mut prev = s.as_bytes()[0];
    let mut len = 1;
    substring_lengths[(prev - b'a') as usize] = 1;

    for curr in s.as_bytes().iter().skip(1) {
        // is adjacent with prev
        if prev + 1 == *curr || curr + 25 == prev {
            len += 1;
        } else {
            len = 1;
        }

        let idx = (curr - b'a') as usize;
        substring_lengths[idx] = max(substring_lengths[idx], len);

        prev = *curr;
    }

    substring_lengths.iter().sum::<usize>() as i32
}

// https://leetcode.com/problems/find-unique-binary-string/
fn find_different_binary_string(nums: Vec<String>) -> String {
    let mut res = String::with_capacity(nums.len());
    for (idx, s) in nums.iter().enumerate() {
        let c = s.as_bytes()[idx] as char;
        let append = match c {
            '0' => '1',
            _ => '0',
        };
        res.push(append);
    }

    res
}

// https://leetcode.com/problems/subarray-sum-equals-k/
fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    // curr_sum(i) = sum till index i
    // curr_sum(i) = prefix_sum(j) + k, where prefix_sum(j) is curr_sum(j), j < i
    //  => prefix_sum = curr_sum(i) - k
    //  => lookup if any prefix sum matches `curr_sum -k`

    type PrefixSum = i32;
    let mut prefix_sums: HashMap<PrefixSum, Vec<i32>> = HashMap::new();

    prefix_sums.insert(0, vec![-1]);

    let mut curr_sum: i32 = 0;
    let mut res = 0;

    for (idx, n) in nums.iter().enumerate() {
        curr_sum += n;

        let reqd_prefix_sum = curr_sum - k;
        if let Some(indices) = prefix_sums.get(&reqd_prefix_sum) {
            res += indices.len()
        };
        prefix_sums
            .entry(curr_sum)
            .and_modify(|indices| indices.push(idx as i32))
            .or_insert(vec![idx as i32]);
    }

    res as i32
}

struct RandomizedSet {
    map: HashMap<i32, usize>,
    arr: Vec<i32>,
}
impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            arr: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        match self.map.get(&val) {
            None => {
                self.map.insert(val, self.arr.len());
                self.arr.push(val);
                true
            }
            Some(_) => false,
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            Some(index) => {
                // remove from array
                self.arr.swap_remove(index);
                // update index in map IF item was NOT the last item in array
                if index < self.arr.len() {
                    self.map.insert(self.arr[index], index);
                }
                true
            }
            None => false,
        }
    }

    fn get_random(&self) -> i32 {
        *self.arr.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_container_most_water() {
        assert_eq!(container_most_water(vec![3, 4, 1, 2, 2, 4, 1, 3, 2]), 21);
        assert_eq!(container_most_water(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_find_substring_in_wraparound_string() {
        assert_eq!(find_substring_in_wraparound_string(String::from("abc")), 6);
        assert_eq!(find_substring_in_wraparound_string(String::from("cac")), 2);
        assert_eq!(find_substring_in_wraparound_string(String::from("zab")), 6);
        assert_eq!(find_substring_in_wraparound_string(String::from("baz")), 3);
        assert_eq!(
            find_substring_in_wraparound_string(String::from("yzabbazy")),
            10
        );
        assert_eq!(
            find_substring_in_wraparound_string(String::from("xyzab")),
            15
        );
    }

    #[test]
    fn test_diff_binary_string() {
        assert_eq!(
            find_different_binary_string(vec!["01".to_string(), "10".to_string()]),
            "11"
        );
    }

    #[test]
    fn test_subarray_sum_k() {
        assert_eq!(subarray_sum(vec![3, 4, 7, 2, -3, 1, 4, 2], 7), 4);
        assert_eq!(subarray_sum(vec![1, -1, 0], 0), 3);
        assert_eq!(subarray_sum(vec![3, 2, 7, 2, -3, 1], 7), 2);
    }
}
