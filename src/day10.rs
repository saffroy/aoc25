use std::collections::HashSet;
use lru::LruCache;

struct Machine {
    key: usize,
    buttons: Vec<usize>,
    joltage: Vec<usize>,
}

fn parse_key(s: &[u8]) -> usize {
    let mut key = 0;
    for &c in s.iter().rev() {
        key *= 2;
        key += if c == b'#' { 1 } else { 0 };
    }
    key
}

fn parse_nums(s: &str) -> Vec<usize> {
    s.split(",")
        .map(|numstr| numstr.parse::<usize>().unwrap())
        .collect()
}

fn parse_button(s: &str) -> usize {
    parse_nums(s)
        .iter()
        .map(|&n| 2usize.pow(n as u32))
        .reduce(|acc, n| acc|n).unwrap()
}

fn parse_text(text: &str) -> Vec<Machine> {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut machine = Machine{key: 0, buttons: vec![], joltage: vec![]};
            line.split(" ")
                .for_each(|word| {
                    let w = word.as_bytes();
                    let s = &w[1..w.len()-1];
                    match w[0] {
                        b'[' => machine.key = parse_key(s),
                        b'(' => machine.buttons.push(
                            parse_button(str::from_utf8(s).unwrap())),
                        b'{' => machine.joltage =
                            parse_nums(str::from_utf8(s).unwrap()),
                        _ => (),
                    }
                });
            machine
        })
        .collect()
}

fn solve_machine(machine: &Machine) -> usize {
    // breadth-first search over reachable states
    let mut states: HashSet<usize> = HashSet::new();
    let mut depth: usize = 0;
    let mut cur_states: Vec<usize> = vec![0; 1]; // start state

    while !states.contains(&machine.key) {
        let next_states: Vec<usize> = cur_states
            .iter()
            .flat_map(|&s| machine.buttons
                      .iter()
                      .map(move |b| s^b))
            .filter(|&s| !states.contains(&s))
            .collect();
        assert!(!next_states.is_empty());

        next_states.iter().for_each(|&s|{
            states.insert(s);
        });
        depth += 1;
        cur_states = next_states;
    }
    depth
}

pub fn parse_1(text: &str) -> i64 {
    let machines = parse_text(text);
    machines
        .iter()
        .map(solve_machine)
        .sum::<usize>() as i64
}

fn min_sol(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (Some(va), Some(vb)) => Some(std::cmp::min(va, vb)),
        (Some(_), None) => a,
        (None, Some(_)) => b,
        (None, None) => None,
    }
}

fn add_sol(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (Some(va), Some(vb)) => Some(va+vb),
        _ => None,
    }
}

type Cache = LruCache<(usize, Vec<usize>), Option<usize>>;

fn subsolve(buttons: &[Vec<usize>], target: &[usize],
            max_presses: Option<usize>,
            cache: &mut Cache) -> Option<usize> {
    let n_buttons = buttons.len();
    let n_counters = target.len();

    let key = (n_buttons, target.to_vec());
    if let Some(r) = cache.get(&key) {
        return *r;
    }

    let r =
        if target.iter().all(|&n| n == 0) {
            println!("#");
            Some(0)
        } else if n_buttons == 0 {
            None
        } else {
            let b = &buttons[0];
            let max_multiplier = (0..n_counters)
                .map(|k| if b[k] > target[k] { 0 }
                     else if b[k] > 0 { target[k] }
                     else { usize::MAX })
                .min().unwrap();
            let max_multiplier =
                min_sol(Some(max_multiplier), max_presses).unwrap();
            assert!(max_multiplier < usize::MAX);

            let mut sub_target: Vec<usize> = vec![0; n_counters];
            let mut sub_best = None;
            for m in (0..=max_multiplier).rev() {
                for k in 0..n_counters {
                    sub_target[k] = target[k] - m*b[k];
                }
                let sub_r = subsolve(&buttons[1..], &sub_target,
                                     min_sol(max_presses, sub_best),
                                     cache);
                sub_best = min_sol(sub_best,
                                   add_sol(Some(m), sub_r));
            }

            sub_best
        };

    // println!("{:?} <-- {:?} {:?}",
    //          r, n_buttons, target);
    cache.put(key, r);
    r
}

fn solve_joltage(machine: &Machine) -> usize {
    // strategy: dynamic programming, like with the coin change
    // problem, but in multiple dimensions (one per joltage level)
    let mut cache = LruCache::new(
        std::num::NonZeroUsize::new(10_000_000).unwrap());
    let n_counters = machine.joltage.len();
    let mut buttons_joltage: Vec<Vec<usize>> = machine.buttons
        .iter()
        .map(|b| (0..n_counters)
             .map(|k| (b >> k) & 1)
            .collect())
        .collect();
    //WIP: sort buttons by rarity, i.e. count how many buttons update
    // each level -- didn't help much :-/
    let _counter_pop: Vec<usize> =
        (0..n_counters)
        .map(|k| buttons_joltage
             .iter()
             .map(|b| b[k])
             .sum())
        .collect();
    buttons_joltage.sort_by_key(|b| {
        // let v: Vec<usize> = (0..n_counters)
        //     .map(|k| b[k]*(n_counters - counter_pop[k]))
        //     .sorted()
        //     .rev()
        //     .collect();
        let v: usize = b.iter().sum();
        std::cmp::Reverse(v)
    });
    println!("{:?}", buttons_joltage);
    subsolve(&buttons_joltage,
             &machine.joltage,
             Some(machine.joltage.iter().sum()), //helpful?
             &mut cache)
        .unwrap()
}

pub fn parse_2(text: &str) -> i64 {
    let machines = parse_text(text);
    machines
        .iter()
        .map(solve_joltage)
        .inspect(|r| { println!("{}", r)})
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    #[test]
    fn test10_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 7);
    }
    #[test]
    fn test10_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 10+12+11);
    }
}
