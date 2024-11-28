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
}
