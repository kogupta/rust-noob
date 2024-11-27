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
    fn dfs(grid: &Vec<Vec<char>>, row: i32, col: i32, visited: &mut [Vec<bool>]) {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        if row < 0 || row >= m || col < 0 || col >= n {
            return;
        }

        let row = row as usize;
        let col = col as usize;
        if visited[row][col] || grid[row][col] == '0' {
            return;
        }

        visited[row][col] = true;

        let row = row as i32;
        let col = col as i32;
        dfs(grid, row - 1, col, visited);
        dfs(grid, row + 1, col, visited);
        dfs(grid, row, col - 1, visited);
        dfs(grid, row, col + 1, visited);
    }

    let mut id: i32 = 0;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    for (i, chars) in grid.iter().enumerate() {
        for (j, c) in chars.iter().enumerate() {
            if *c == '1' && !visited[i][j] {
                id += 1;
                dfs(&grid, i as i32, j as i32, &mut visited);
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
}
