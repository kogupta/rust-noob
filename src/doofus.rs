fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    fn dfs(image: &mut Vec<Vec<i32>>, original_colour: i32, row: usize, col: usize, color: i32) {
        if image[row][col] == original_colour {
            image[row][col] = color;

            let m = image.len();
            let n = image[0].len();

            // up
            if row > 0 {
                dfs(image, original_colour, row - 1, col, color);
            }
            // down
            if row < m - 1 {
                dfs(image, original_colour, row + 1, col, color);
            }
            // left
            if col > 0 {
                dfs(image, original_colour, row, col - 1, color);
            }
            // right
            if col < n - 1 {
                dfs(image, original_colour, row, col + 1, color);
            }
        }
    }

    let sr = sr as usize;
    let sc = sc as usize;

    let original_colour = image[sr][sc];

    if original_colour == color {
        return image;
    }

    let mut image = image;
    dfs(&mut image, original_colour, sr, sc, color);

    image
}

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    fn dfs(grid: &Vec<Vec<char>>, row: usize, col: usize, visited: &mut [Vec<bool>]) {
        if !visited[row][col] && grid[row][col] == '1' {
            visited[row][col] = true;

            if row > 0 {
                dfs(grid, row - 1, col, visited);
            }
            if row < grid.len() - 1 {
                dfs(grid, row + 1, col, visited);
            }
            if col > 0 {
                dfs(grid, row, col - 1, visited);
            }
            if col < grid[0].len() - 1 {
                dfs(grid, row, col + 1, visited);
            }
        }
    }

    let mut id: i32 = 0;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    for (i, chars) in grid.iter().enumerate() {
        for (j, c) in chars.iter().enumerate() {
            if *c == '1' && !visited[i][j] {
                id += 1;
                dfs(&grid, i, j, &mut visited);
            }
        }
    }

    id
}

fn num_islands2(mut grid: Vec<Vec<char>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
        if grid[row][col] == '1' {
            grid[row][col] = '0';

            if row > 0 {
                dfs(grid, row - 1, col);
            }
            if row < grid.len() - 1 {
                dfs(grid, row + 1, col);
            }
            if col > 0 {
                dfs(grid, row, col - 1);
            }
            if col < grid[0].len() - 1 {
                dfs(grid, row, col + 1);
            }
        }
    }

    let mut id: i32 = 0;

    // use in place mutation to track whether node is visited
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                id += 1;
                dfs(&mut grid, i, j);
            }
        }
    }

    id
}

// https://leetcode.com/problems/surrounded-regions/
fn solve(board: &mut Vec<Vec<char>>) {
    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
        if board[r][c] == 'O' {
            board[r][c] = 'S';

            if r > 0 {
                dfs(board, r - 1, c);
            }
            if r < board.len() - 1 {
                dfs(board, r + 1, c);
            }
            if c > 0 {
                dfs(board, r, c - 1);
            }
            if c < board[0].len() - 1 {
                dfs(board, r, c + 1);
            }
        }
    }

    let m = board.len();
    let n = board[0].len();

    // search for O in boundary - mark them as S
    for r in 0..m {
        for c in [0, n - 1] {
            if board[r][c] == 'O' {
                dfs(board, r, c);
            }
        }
    }

    for r in [0, m - 1] {
        for c in 0..n {
            if board[r][c] == 'O' {
                dfs(board, r, c);
            }
        }
    }

    // re-mark S as O, other O as X
    for chars in board {
        for c in 0..n {
            match chars[c] {
                'S' => chars[c] = 'O',
                _ => chars[c] = 'X',
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill() {
        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }

    #[test]
    fn test_island_count() {
        assert_eq!(
            num_islands2(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
        assert_eq!(
            num_islands2(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }

    #[test]
    fn test_surrounded_regions() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        solve(&mut board);

        assert_eq!(board, expected);
    }
}
