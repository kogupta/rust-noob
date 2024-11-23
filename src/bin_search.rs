// https://leetcode.com/problems/koko-eating-bananas
fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    fn time_taken(piles: &[i32], rate: i32) -> i32 {
        // note: integer ceiling division: (n + k - 1)/k
        piles
            .iter()
            .fold(0, |acc, pile| acc + ((pile + rate - 1) / rate))
    }

    let max_pile = piles.iter().max().unwrap();
    if piles.len() == h as usize {
        *max_pile
    } else {
        let mut lo = 1;
        let mut hi = *max_pile;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let t = time_taken(&piles, mid);
            if t <= h {
                // t within expected bound - rate of t is acceptable
                // there maybe some acceptable rate lower than t
                hi = mid;
            } else {
                // not fast enough - look right
                lo = mid + 1;
            }
        }

        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_eating_speed() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
