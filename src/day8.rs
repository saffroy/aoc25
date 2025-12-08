use itertools::Itertools;
use std::collections::BinaryHeap;
use std::cmp::{min, Reverse};

type Point = (i64, i64, i64);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    dist: i64,
    i: usize,
    j: usize,
}

fn parse_text(text: &str) -> Vec<Point> {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(",")
             .map(|s| s.parse::<i64>().unwrap())
             .collect_tuple() // from Itertools
             .unwrap())
        .collect()
}

fn distance(a: Point, b: Point) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn dist_heap(points: &[Point]) -> BinaryHeap<Reverse<Pair>> {
    let n = points.len();

    // collect distances between every pair into min-heap
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        for j in i+1..n {
            let dist = distance(points[i], points[j]);
            let p = Pair{dist, i, j};
            heap.push(Reverse(p));
        }
    }

    heap
}

pub fn solve_1(text: &str, connections: u32) -> i64 {
    let points = parse_text(text);
    let n = points.len();

    // collect distances between every pair into min-heap
    let mut heap = dist_heap(&points);

    // initially, each point is its own circuit
    let mut circuit: Vec<u32> = (0..n as u32).collect();

    // connect points for K closest pairs
    for _ in 0..connections {
        let Reverse(p) = heap.pop().unwrap();
        let (ci, cj) = (circuit[p.i], circuit[p.j]);
        if ci != cj {
            let ck = min(ci, cj);
            for c in circuit.iter_mut() {
                if *c == ci || *c == cj {
                    *c = ck
                }
            }
        }
    }

    let mut circuit_sizes: Vec<u32> = vec![0; n];
    circuit.iter().for_each(|&ck| {
        circuit_sizes[ck as usize] += 1;
    });

    circuit_sizes.sort_by_key(|&s| Reverse(s));

    circuit_sizes.into_iter().take(3).product::<u32>() as i64
}

pub fn parse_1(text: &str) -> i64 {
    solve_1(text, 1000)
}

pub fn parse_2(text: &str) -> i64 {
    let points = parse_text(text);
    let n = points.len();

    // collect distances between every pair into min-heap
    let mut heap = dist_heap(&points);

    // initially, each point is its own circuit
    let mut circuit: Vec<u32> = (0..n as u32).collect();

    // connect points until there is only one circuit
    loop {
        let Reverse(p) = heap.pop().unwrap();
        let (ci, cj) = (circuit[p.i], circuit[p.j]);
        if ci != cj {
            let ck = min(ci, cj);
            for c in circuit.iter_mut() {
                if *c == ci || *c == cj {
                    *c = ck
                }
            }
            if circuit.iter().all(|&c| c == circuit[0]) {
                return points[p.i].0 * points[p.j].0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
    #[test]
    fn test8_parse1() {
        assert_eq!(solve_1(&INPUT_TEXT_1, 10), 40);
    }
    #[test]
    fn test8_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 25272);
    }
}
