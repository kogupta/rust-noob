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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fruitiness() {
        assert_eq!(total_fruit(vec![3, 3, 2, 1, 2, 1, 0]), 4);
        assert_eq!(total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]), 5);
    }
}