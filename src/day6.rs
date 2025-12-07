use regex::Regex;

fn parse_text(text: &str) -> (Vec<Vec<i64>>, Vec<&str>) {
    let re = Regex::new(r"\S+").unwrap();
    let mut nums: Vec<Vec<i64>> = vec![];
    let mut ops: Vec<&str> = vec![];

    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| re.find_iter(line)
             .map(|m| m.as_str())
             .collect())
        .for_each(|v: Vec<&str>| {
            if v[0] == "+" || v[0] == "*" {
                ops = v
            } else {
                nums.push(v.iter()
                          .map(|s| s.parse::<i64>().unwrap())
                          .collect())
            }
        });

    (nums, ops)
}

pub fn parse_1(text: &str) -> i64 {
    let (nums, ops) = parse_text(text);
    (0..ops.len())
        .map(|col| (0..nums.len())
             .map(|row| nums[row][col])
             .reduce(|acc, e| if ops[col] == "*" { acc * e} else { acc + e })
             .unwrap())
        .sum()
}

pub fn parse_2(text: &str) -> i64 {
    let re = Regex::new(r"(\d+)\s*([*+]?)").unwrap();
    let grid: Vec<_> = text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .collect();

    let mut total = 0;
    let mut args: Vec<i64> = vec![];

    (0..grid[0].len())
        .rev()
        .for_each(|col| {
            let v = (0..grid.len())
                .map(|row| grid[row][col])
                .collect::<Vec<u8>>();
            let s = str::from_utf8(&v).unwrap();

            if let Some(caps) = re.captures(s) {
                args.push(caps[1].parse().unwrap());

                if !caps[2].is_empty() {
                    let inc = if &caps[2] == "*" {
                        args.iter().product::<i64>()
                    } else {
                        args.iter().sum::<i64>()
                    };
                    total += inc;
                    args.clear();
                }
            }
        });

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    #[test]
    fn test6_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 4277556);
    }
    #[test]
    fn test6_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 3263827);
    }
}
