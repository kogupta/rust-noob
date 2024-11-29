// https://leetcode.com/problems/minimum-knight-moves/
fn knight_min_moves(x: i32, y: i32) -> i32 {
    use std::collections::{HashSet, VecDeque};
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

// https://leetcode.com/problems/bus-routes/
fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    use std::collections::{HashMap, VecDeque};

    if source == target {
        return 0;
    }

    // `routes` are "components"
    // consider it Map[RouteId, [BusRoute]]
    // build a reverse map: Map[BusRoute, [RouteId]]
    // lookup using these 2 maps

    type Route = usize;
    type BusStop = i32;

    fn build_reverse_map(routes: &[Vec<i32>]) -> HashMap<BusStop, Vec<Route>> {
        let mut bus_stops: HashMap<BusStop, Vec<Route>> = HashMap::new();
        for (id, stops) in routes.iter().enumerate() {
            for stop in stops {
                bus_stops.entry(*stop).or_default().push(id);
            }
        }

        bus_stops
    }

    let bus_stops = build_reverse_map(&routes);

    if !bus_stops.contains_key(&source) || !bus_stops.contains_key(&target) {
        return -1;
    }

    // start searching via routes
    // source -> one or more routes
    let mut visited = vec![false; routes.len()];
    let mut q: VecDeque<Route> = VecDeque::new();

    for route in bus_stops.get(&source).unwrap() {
        q.push_back(*route);
        visited[*route] = true;
    }

    let mut stops = 0;

    while !q.is_empty() {
        let n = q.len();
        for _ in 0..n {
            let route = q.pop_front().unwrap();
            for bus_stop in &routes[route] {
                if target == *bus_stop {
                    return stops + 1;
                }
                for route in bus_stops.get(bus_stop).unwrap() {
                    if !visited[*route] {
                        visited[*route] = true;
                        q.push_back(*route);
                    }
                }
            }
        }

        stops += 1;
    }

    -1
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

    #[test]
    fn test_bus_routes() {
        assert_eq!(
            num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6),
            2
        );
    }
}
