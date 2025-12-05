use regex::Regex;

pub fn parse_1(text: &str) -> i64 {
    let re = Regex::new(r"([LR])(\d+)").unwrap();
    let mut pos: i32 = 50;
    let mut count_zero: i32 = 0;

    for caps in re.captures_iter(text) {
        let right: bool = &caps[1] == "R";
        let count: i32 = caps[2].parse().unwrap();

        pos += if right { count } else { 100-count };
        pos %= 100;

        if pos == 0 {
            count_zero += 1;
        }
    }

    count_zero as i64
}

pub fn parse_2(text: &str) -> i64 {
    let re = Regex::new(r"([LR])(\d+)").unwrap();
    let mut pos: i32 = 50;
    let mut count_zero: i32 = 0;

    for caps in re.captures_iter(text) {
        let right: bool = &caps[1] == "R";
        let mut count: i32 = caps[2].parse().unwrap();

        // full dial rotations
        count_zero += count / 100;
        count %= 100;

        if count == 0 {
            continue
        }

        if !right {
            if pos > 0 && count >= pos {
                count_zero += 1
            }
        } else if count + pos >= 100 {
            count_zero += 1
        }

        pos += if right { count } else { 100-count };
        pos %= 100;
    }

    count_zero as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    #[test]
    fn test1_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 3);
    }
    #[test]
    fn test1_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 6);
    }
}
