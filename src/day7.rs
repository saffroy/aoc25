fn parse_text(text: &str) -> Vec<&[u8]> {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .collect()
}

pub fn parse_1(text: &str) -> i64 {
    let grid = parse_text(text);

    let width = grid[0].len();
    let mut beams: Vec<bool> = vec![false; width];

    let s_pos = grid[0].iter().position(|&c| c == b'S').unwrap();
    beams[s_pos] = true;

    let mut split_count = 0;
    for line in grid.iter() {
        let mut next_beams: Vec<bool> = vec![false; width];
        for col in 0..width {
            let mut b = beams[col];

            if line[col] == b'^' {
                if b {
                    split_count += 1;
                    if col > 0 {
                        next_beams[col-1] = true;
                    }
                    if col < width-1 {
                        next_beams[col+1] = true;
                    }
                }
                b = false;
            }
            next_beams[col] |= b;
        }
        beams = next_beams;
    }

    split_count
}

pub fn parse_2(text: &str) -> i64 {
    let grid = parse_text(text);

    let width = grid[0].len();
    let mut tls: Vec<i64> = vec![0; width];

    let s_pos = grid[0].iter().position(|&c| c == b'S').unwrap();
    tls[s_pos] = 1;

    for line in grid.iter() {
        let mut next_tls: Vec<i64> = vec![0; width];
        for col in 0..width {
            let mut tl = tls[col];

            if line[col] == b'^' {
                if tl > 0 {
                    if col > 0 {
                        next_tls[col-1] += tl;
                    }
                    if col < width-1 {
                        next_tls[col+1] += tl;
                    }
                }
                tl = 0;
            }
            next_tls[col] += tl;
        }
        tls = next_tls;
    }

    tls.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    #[test]
    fn test7_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 21);
    }
    #[test]
    fn test7_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 40);
    }
}
