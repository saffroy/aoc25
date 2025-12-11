use std::collections::HashSet;

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

pub fn parse_2(_text: &str) -> i64 {
    0
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
        assert_eq!(parse_2(&INPUT_TEXT_1), 6);
    }
}
