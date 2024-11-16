// https://leetcode.com/problems/fruit-into-baskets/
fn total_fruit(fruits: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;

    let mut max_len = 0;
    // window
    let mut start: usize = 0;
    // state
    let mut fruit_count: HashMap<i32, u32> = HashMap::with_capacity(3);

    for (idx, fruit) in fruits.iter().enumerate() {
        *fruit_count.entry(*fruit).or_insert(0) += 1;

        if fruit_count.len() <= 2 {
            max_len = max(max_len, idx - start + 1);
        } else {
            while fruit_count.len() > 2 {
                let f = fruits[start];
                start += 1;

                if let Some(count) = fruit_count.get_mut(&f) {
                    if *count == 1 {
                        fruit_count.remove(&f);
                    } else {
                        *count -= 1
                    }
                }
            }
        }
    }

    max_len as i32
}

// https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    use std::cmp::max;

    let mut curr_sum: i32 = card_points.iter().take(k as usize).sum();

    if card_points.len() == k as usize {
        return curr_sum;
    }

    let mut max_sum = curr_sum;

    let mut right = card_points.len() - 1;
    for idx in (0..k).rev() {
        curr_sum = curr_sum - card_points[idx as usize] + card_points[right];
        max_sum = max(max_sum, curr_sum);
        right -= 1;
    }

    max_sum
}

// https://leetcode.com/problems/longest-repeating-character-replacement
fn character_replacement(s: String, k: i32) -> i32 {
    use std::cmp::max;

    // sliding window
    // window length - most frequent char in window -> # of replacements
    // => invariant: window length - most frequent char in window <= k
    // => update left pointer when invariant is false

    let mut start: usize = 0;
    let mut max_freq = 0;
    let char_array = s.as_bytes();
    let mut char_counts = [0; 26];

    for (end, c) in char_array.iter().enumerate() {
        let idx = (c - b'A') as usize;
        char_counts[idx] += 1;

        max_freq = max(max_freq, char_counts[idx]);

        // invariant: window-length - most-freq-char-count <= k
        // window length = idx - start + 1
        while (end - start + 1) - max_freq > k as usize {
            let char_index = (char_array[start] - b'A') as usize;
            char_counts[char_index] -= 1;
            start += 1;
        }
    }

    (s.len() - start) as i32
}

// https://leetcode.com/problems/max-consecutive-ones-iii/description/
fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::max;

    let mut zeroes = 0;
    let mut max_len = 0;
    let mut start = 0;
    for (end, one_or_zero) in nums.iter().enumerate() {
        if *one_or_zero == 0 {
            zeroes += 1;

            // # of zeroes should be <= k
            while zeroes > k {
                let n = nums[start];
                start += 1;
                if n == 0 {
                    zeroes -= 1;
                }
            }
        }

        max_len = max(max_len, end - start + 1);
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fruitiness() {
        assert_eq!(total_fruit(vec![3, 3, 2, 1, 2, 1, 0]), 4);
        assert_eq!(total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]), 5);
    }

    #[test]
    fn test_max_points() {
        assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
        assert_eq!(max_score(vec![2, 2, 2], 2), 4);
        assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
        assert_eq!(max_score(vec![2, 11, 4, 5, 3, 9, 2], 3), 17);
        assert_eq!(max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3), 202);
    }

    #[test]
    fn test_char_replacement() {
        assert_eq!(character_replacement(String::from("ABAB"), 2), 4);
        assert_eq!(character_replacement(String::from("AABAC"), 2), 5);
        assert_eq!(character_replacement(String::from("AABABBA"), 1), 4);
        assert_eq!(character_replacement(String::from("ABABBA"), 2), 5);
        assert_eq!(character_replacement(String::from("BBABCCDD"), 2), 5);
        assert_eq!(character_replacement(String::from("BCBABCCCCA"), 2), 6);
    }

    #[test]
    fn test_longest_ones() {
        assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
        assert_eq!(
            longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3,
            ),
            10
        );
    }
}
