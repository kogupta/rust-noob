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
