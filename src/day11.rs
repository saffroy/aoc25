use std::collections::HashMap;

use itertools::Itertools;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_text(text: &str) -> Graph<'_> {
    let mut graph = HashMap::new();
    text.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (from, words) = line.split_once(": ").unwrap();
            let to = words.split(" ").collect_vec();
            graph.insert(from, to);
        });
    graph
}

fn count_paths(graph: &Graph,
               label_source: &str, label_sink: &str) -> i64 {
    let mut counts: HashMap<&str, i64> = HashMap::new();
    let mut nodes_from: Vec<&str> = vec![label_source];

    while !nodes_from.is_empty() {
        let mut nodes_to: Vec<&str> = vec![];

        nodes_from.iter().for_each(|label_from| {
            graph.get(label_from).unwrap().iter()
                .for_each(|label_to| {
                    if let Some(c) = counts.get_mut(label_to) {
                        *c += 1;
                    } else {
                        counts.insert(label_to, 1);
                    }
                    if *label_to != label_sink {
                        nodes_to.push(label_to);
                    }
                })
        });
        nodes_from = nodes_to;
    }

    *counts.get(label_sink).unwrap()
}

pub fn parse_1(text: &str) -> i64 {
    let graph = parse_text(text);
    count_paths(&graph, "you", "out")
}

// same base logic as part 1, different map-based implementation to
// accomodate nastier graph in part 2

struct CountMap {
    h: HashMap<String, i64>,
}

impl CountMap {
    fn new() -> CountMap {
        CountMap{h: Default::default()}
    }
    fn inc(&mut self, label: &str, n: i64) {
        if let Some(c) = self.h.get_mut(label) {
            *c += n;
        } else {
            self.h.insert(label.to_string(), n);
        }
    }
}

fn count_paths2(graph: &Graph,
               label_source: &str, label_sink: &str) -> i64 {
    let mut counts = CountMap::new();
    let mut nodes_from = CountMap::new();

    nodes_from.inc(label_source, 1);

    while !nodes_from.h.is_empty() {
        let mut nodes_to = CountMap::new();

        nodes_from.h.iter().for_each(|(label_from, &n)| {
            if let Some(out) = graph.get(&label_from[..]) {
                out.iter().for_each(|&label_to| {
                    counts.inc(label_to, n);
                    if label_to != label_sink {
                        nodes_to.inc(label_to, n);
                    }
                })
            }
        });
        nodes_from = nodes_to;
    }

    *counts.h.get(label_sink).unwrap()
}

pub fn parse_2(text: &str) -> i64 {
    let graph = parse_text(text);
    count_paths2(&graph, "svr", "fft")
        * count_paths2(&graph, "fft", "dac")
        * count_paths2(&graph, "dac", "out")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
    #[test]
    fn test11_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 5);
    }

    #[test]
    fn test11_count2() {
        let graph = parse_text(INPUT_TEXT_1);
        assert_eq!(count_paths2(&graph, "you", "out"), 5);
    }

    const INPUT_TEXT_2: &str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
    #[test]
    fn test11_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_2), 2);
    }
}
