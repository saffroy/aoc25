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

pub fn parse_2(_text: &str) -> i64 {
    0
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
        assert_eq!(parse_2(&INPUT_TEXT_1), 6);
    }
}
