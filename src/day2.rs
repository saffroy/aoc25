use regex::Regex;

fn parse_text(text: &str) -> Vec<(i64, i64)> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();

    text.lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split(","))
        .map(|seqstr| {
            let caps = re.captures(seqstr).unwrap();
            let lo = caps[1].parse().unwrap();
            let hi = caps[2].parse().unwrap();
            (lo, hi)
        })
        .collect()
}

fn count_digits(mut n: i64) -> u32 {
    let mut d = 1;
    while n > 9 {
        d += 1;
        n /= 10;
    }
    d
}

fn sum_inval(lo: i64, hi: i64) -> i64 {
    assert!(lo < hi);

    let lo_digits: u32 = count_digits(lo);
    let hi_digits: u32 = count_digits(hi);

    // don't handle this extra case unless required
    assert!(lo_digits + 1 >= hi_digits);

    if lo_digits < hi_digits {
        // split interval into ranges with same # of digits
        sum_inval(lo, 10i64.pow(lo_digits)-1)
            + sum_inval(10i64.pow(lo_digits), hi)
    } else if !lo_digits.is_multiple_of(2) {
        // odd number of digits can't be invalid
        0
    } else {
        // split lo and hi into top/bottom pair of numbers
        // with equal number of digits
        let mid_pow = 10i64.pow(lo_digits / 2);
        let top_lo = lo / mid_pow;
        let top_hi = hi / mid_pow;
        let bot_lo = lo % mid_pow;
        let bot_hi = hi % mid_pow;

        let mut invals = 0;
        if top_lo == top_hi {
            // same top digits: only inval id has the same as its
            // bottom digits
            if bot_lo <= top_lo
                && top_lo <= bot_hi {
                    invals += top_lo * mid_pow + top_lo
                }
        } else {
            if bot_lo <= top_lo {
                invals += top_lo * mid_pow + top_lo
            }
            invals += (top_lo+1 .. top_hi)
                .map(|top| top * mid_pow + top)
                .sum::<i64>();
            if top_hi <= bot_hi {
                invals += top_hi * mid_pow + top_hi
            }
        }
        invals
    }
}

pub fn parse_1(text: &str) -> i64 {
    parse_text(text)
        .into_iter()
        .map(|(lo, hi)| sum_inval(lo, hi))
        .sum()
}

fn is_inval2(n: &i64) -> bool {
    let s = n.to_string();
    let s_len = s.len();
    for prefix_len in 1..s_len/2+1 {
        if s_len.is_multiple_of(prefix_len) {
            let prefix = &s[0..prefix_len];
            let reps = s_len / prefix_len;
            let is_repeat = (0..reps)
                .all(|k| prefix == &s[k*prefix_len..(k+1)*prefix_len]);
            if is_repeat {
                return true
            }
        }
    }
    false
}

fn sum_inval2(lo: i64, hi: i64) -> i64 {
    (lo..hi+1)
        .filter(is_inval2)
        .sum()
}

pub fn parse_2(text: &str) -> i64 {
    parse_text(text)
        .into_iter()
        .map(|(lo, hi)| sum_inval2(lo, hi))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 1227775554);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 4174379265);
    }

    #[test]
    fn test_parse_text() {
        assert_eq!(parse_text("\n123-456,7890-1234\n"),
                              [(123, 456), (7890, 1234)]);
    }
    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(9), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(100), 3);
    }
    #[test]
    fn test_count_inval() {
        assert_eq!(sum_inval(11, 22), 11+22);
    }
    #[test]
    fn test_count_inval2() {
        assert_eq!(sum_inval2(11, 22), 11+22);
        assert_eq!(sum_inval2(95, 115), 99+111);
    }
}
