// https://leetcode.com/problems/insert-interval
fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::{max, min};

    // sorted, disjoint intervals
    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut index = 0;
    // let mut new_interval = new_interval;

    // less than pivot
    while index < intervals.len() && intervals[index][1] < new_interval[0] {
        result.push(intervals[index].clone());
        index += 1;
    }

    // = pivot/merged-entity
    // | ....... |
    //       | ... |  => intersection
    //            | ... |  => NO intersection, lo > merged.hi
    while index < intervals.len() && intervals[index][0] <= new_interval[1] {
        // merge
        new_interval[0] = min(new_interval[0], intervals[index][0]);
        new_interval[1] = max(new_interval[1], intervals[index][1]);
        index += 1;
    }

    result.push(new_interval);

    // items > merged-entity
    while index < intervals.len() {
        result.push(intervals[index].clone());
        index += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_insert() {
        assert_eq!(
            insert(vec![vec![1, 3], vec![4, 6], vec![6, 7], vec![8, 10], vec![11, 15]],
                   vec![5, 8]),
            vec![vec![1, 3], vec![4, 10], vec![11, 15]]);
        // assert_eq!(
        //     insert(vec![[1, 3], [6, 9]], vec![2, 5]),
        //     vec![[1, 5], [6, 9]]);
        // assert_eq!(
        //     insert(vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], vec![4, 8]),
        //     vec![[1, 2], [3, 10], [12, 16]]);
        // assert_eq!(
        //     insert(vec![[1, 2], [6, 7], [8, 10], [12, 16]], vec![3, 5]),
        //     vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]);
        // assert_eq!(
        //     insert(vec![[1, 2], [3, 5], [6, 7], [8, 10]], vec![12, 16]),
        //     vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]);
        // assert_eq!(
        //     insert(vec![[1, 5]], vec![6, 8]),
        //     vec![[1, 5], [6, 8]]);
        // assert_eq!(
        //     insert(vec![[6, 8]], vec![1, 5]),
        //     vec![[1, 5], [6, 8]]);
        // assert_eq!(
        //     insert(vec![], vec![5, 7]),
        //     vec![[5, 7]]);
    }
}