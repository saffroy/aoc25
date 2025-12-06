use regex::Regex;
use std::cmp::Ordering;

struct Edge {
    pos: usize,
    up: bool,   // true if beginning of an interval (count goes up)
    count: u32, // count of intervals covering this position
}

fn cmp_edge(a: &Edge, b: &Edge) -> std::cmp::Ordering {
    if a.pos < b.pos {
        Ordering::Less
    } else if a.pos > b.pos {
        Ordering::Greater
    } else if a.up {
        Ordering::Less
    } else if b.up {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn parse_text(text: &str) -> (Vec<Edge>, Vec<usize>) {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut intervals: Vec<Edge> = vec![];
    let mut items: Vec<usize> = vec![];

    text.lines().for_each(|line|{
        if line.is_empty() {
            return;
        }
        if let Some(caps) = re.captures(line) {
            let a: usize = caps[1].parse().unwrap();
            let b: usize = caps[2].parse().unwrap();
            intervals.push(Edge{pos: a, up: true, count: 0});
            intervals.push(Edge{pos: b, up: false, count: 0});
        } else {
            let p: usize = line.parse().unwrap();
            items.push(p);
        }
    });

    intervals.sort_by(cmp_edge);

    let mut cur_count: u32 = 0;
    intervals.iter_mut().for_each(|e| {
        if e.up {
            cur_count += 1;
            e.count = cur_count;
        } else {
            e.count = cur_count;
            cur_count -= 1;
        }
    });

    (intervals, items)
}

fn is_fresh(intervals: &[Edge], item: usize) -> bool {
    if item < intervals[0].pos
        || item > intervals.last().unwrap().pos {
            return false;
        }

    // find first edge on or after item
    let p = intervals.partition_point(|e| e.pos < item);
    let e = &intervals[p];

    // either item is on the edge of an interval (they are inclusive),
    // or the edge after item isn't bringing count up from zero
    e.pos == item || !(e.up && e.count == 1)
}

pub fn parse_1(text: &str) -> i64 {
    let (intervals, items) = parse_text(text);
    items.iter()
        .filter(|&&item| is_fresh(&intervals, item))
        .count() as i64
}

pub fn parse_2(text: &str) -> i64 {
    let (intervals, _) = parse_text(text);

    let mut total = 0;
    let mut range_start = 0;
    intervals.iter().for_each(|e| {
        if e.count == 1 {
            if e.up {
                range_start = e.pos
            } else {
                total += e.pos - range_start + 1
            }
        }
    });
    total as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
    #[test]
    fn test5_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 3);
    }
    #[test]
    fn test5_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 14);
    }
}
