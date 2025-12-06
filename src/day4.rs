use itertools::iproduct;

fn parse_text(text: &str) -> Vec<Vec<bool>> {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars()
             .map(|c| c == '@')
             .collect())
        .collect()
}

fn neighbours_1d_unchecked(n: usize) -> Vec<usize> {
    vec![n.overflowing_sub(1).0, n, n+1]
}

fn neighbours<T>(grid: &[Vec<T>], i: usize, j: usize) -> Vec<(usize, usize)> {
    iproduct!(neighbours_1d_unchecked(i), neighbours_1d_unchecked(j))
        .filter(|&(x, y)| x < grid.len() && y < grid[0].len())
        .collect::<Vec<(usize, usize)>>()
}

fn count_adjacent(grid: &[Vec<bool>], x: usize, y: usize) -> usize {
    neighbours(grid, x, y)
        .iter()
        .filter(|(i, j)| (*i, *j) != (x, y) && grid[*i][*j])
        .count()
}

fn removables(grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(x, y)| grid[x][y] && count_adjacent(grid, x, y) < 4)
        .collect()
}

pub fn parse_1(text: &str) -> i64 {
    let grid = parse_text(text);
    removables(&grid).len() as i64
}

pub fn parse_2(text: &str) -> i64 {
    let mut grid = parse_text(text);
    let mut removed = 0;

    loop {
        let r = removables(&grid);
        r.iter().for_each(|&(x, y)| grid[x][y] = false);
        removed += r.len();
        if r.is_empty() {
            return removed as i64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    #[test]
    fn test4_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 13);
    }
    #[test]
    fn test4_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 43);
    }
}
