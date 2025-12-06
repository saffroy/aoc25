use std::cmp;
use std::collections::HashMap;

fn parse_text(text: &str) -> Vec<Vec<u8>> {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line
             .as_bytes()
             .iter()
             .map(|b| b - b'0')
            .collect())
        .collect()
}

fn joltage(bank: Vec<u8>) -> i64 {
    // make array of max subsequent digit
    let max_post: Vec<u8> = bank
        .iter()
        .rev()
        .fold((0u8, vec![]), |(max, mut v), e| {
            let m = cmp::max(max, *e);
            v.push(m);
            (m, v)
        })
        .1
        .into_iter()
        .rev()
        .collect();

    (0..bank.len()-1)
        .map(|k| bank[k] as i64*10i64+max_post[k+1] as i64)
        .max().unwrap()
}

pub fn parse_1(text: &str) -> i64 {
    parse_text(text)
        .into_iter()
        .map(joltage)
        .sum()
}

// joltage2 requires memoization, its cache is indexed by:
// (bank length, digits)
type Cache = HashMap<(usize, u32), i64>;

fn joltage2(bank: &[u8], digits: u32, cache: &mut Cache) -> i64 {
    let key = (bank.len(), digits);

    if let Some(v) = cache.get(&key) {
        *v
    } else {
        let v = if digits > bank.len() as u32 {
            0
        } else if digits == 1 {
            *bank[..]
                .iter()
                .max()
                .unwrap() as i64
        } else {
            cmp::max(bank[0] as i64 * 10i64.pow(digits-1)
                     + joltage2(&bank[1..], digits-1, cache),
                     joltage2(&bank[1..], digits, cache))
        };
        cache.insert(key, v);
        v
    }
}

pub fn parse_2(text: &str) -> i64 {
    parse_text(text)
        .into_iter()
        .map(|bank| joltage2(&bank, 12, &mut HashMap::new()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
987654321111111
811111111111119
234234234234278
818181911112111
";
    #[test]
    fn test3_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 357);
    }
    #[test]
    fn test3_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 3121910778619);
    }
}
