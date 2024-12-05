// https://leetcode.com/problems/maximum-profit-in-job-scheduling/
fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    use std::collections::BTreeMap;

    type StartTime = i32;
    type EndTime = i32;
    type Profit = i32;
    type Job = (StartTime, EndTime, Profit);

    let mut jobs: Vec<Job> = {
        let n = start_time.len();
        let mut xs = Vec::with_capacity(n);
        for i in 0..n {
            xs.push((start_time[i], end_time[i], profit[i]));
        }

        xs
    };

    // sort by end time
    jobs.sort_unstable_by_key(|x| x.1);

    let mut profits: BTreeMap<EndTime, Profit> = BTreeMap::new();
    profits.insert(0, 0); // 0 job scheduled, 0 profit

    for (start_time, end_time, profit) in jobs {
        // find the job which ends *BEFORE* and closest to current job
        // aka "floor" key-value for the current `start_time`
        let (_, pft) = profits.range(..=start_time).next_back().unwrap();

        let (_, curr_max_profit) = profits.last_key_value().unwrap();
        if pft + profit > *curr_max_profit {
            profits.insert(end_time, pft + profit);
        }
    }

    *profits.last_key_value().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_schedule() {
        assert_eq!(
            job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
        assert_eq!(
            job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );
        assert_eq!(
            job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6
        );
    }
}
