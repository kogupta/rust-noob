use std::collections::{HashSet, VecDeque};

// https://leetcode.com/problems/minimum-knight-moves/
fn knight_min_moves(x: i32, y: i32) -> i32 {
    // starting from origin, min steps to reach (x, y)
    // x and y in range: [-200, 200]

    // treat board as graph, min steps => bfs!

    type Point = (i32, i32);

    // is this a valid co-ordinate
    fn is_valid(x: i32, y: i32) -> bool {
        -200 < x && x < 200 && -200 < y && y < 200
    }

    enum Direction {
        RightUp,
        RightUp2,
        LeftUp,
        LeftUp2,
        RightDown,
        RightDown2,
        LeftDown,
        LeftDown2,
    }

    let directions: [Direction; 8] = [
        Direction::RightUp,
        Direction::RightUp2,
        Direction::LeftUp,
        Direction::LeftUp2,
        Direction::RightDown,
        Direction::RightDown2,
        Direction::LeftDown,
        Direction::LeftDown2,
    ];

    fn next(x: i32, y: i32, dir: &Direction) -> Option<Point> {
        let (r, s) = match dir {
            Direction::RightUp => (x + 2, y + 1),
            Direction::RightUp2 => (x + 1, y + 2),
            Direction::LeftUp => (x - 2, y + 1),
            Direction::LeftUp2 => (x - 1, y + 2),
            Direction::RightDown => (x + 2, y - 1),
            Direction::RightDown2 => (x + 1, y - 2),
            Direction::LeftDown => (x - 2, y - 1),
            Direction::LeftDown2 => (x - 1, y - 2),
        };

        if is_valid(r, s) {
            Some((r, s))
        } else {
            None
        }
    }

    let mut visited: HashSet<Point> = HashSet::new();
    let mut q: VecDeque<Point> = VecDeque::new();
    q.push_back((0, 0));

    let mut level = 0;
    while !q.is_empty() {
        let n = q.len();
        level += 1;

        for _ in 0..n {
            let p @ (r, s) = q.pop_front().unwrap();
            visited.insert(p);

            // next locations
            for direction in &directions {
                if let Some(next_point @ (a, b)) = next(r, s, direction) {
                    // next location is target?
                    if x == a && y == b {
                        return level;
                    }
                    // not target - add to q if not visited
                    if !visited.contains(&next_point) {
                        q.push_back(next_point);
                    }
                }
            }
        }
    }

    panic!("Expected solution before!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_min_moves() {
        assert_eq!(knight_min_moves(1, 2), 1);
        assert_eq!(knight_min_moves(4, 4), 4);
        assert_eq!(knight_min_moves(5, 5), 4);
    }
}
