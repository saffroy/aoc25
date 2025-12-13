use regex::Regex;

struct Goal {
    x: usize,
    y: usize,
    reqs: Vec<usize>,
}

fn parse_text(text: &str) -> (Vec<Vec<String>>, Vec<Goal>) {
    let re_shape_num = Regex::new(r"^(\d+):$").unwrap();
    let re_shape_line = Regex::new(r"(^[\.\#]+)").unwrap();
    let re_space_goals = Regex::new(r"(\d+)x(\d+): (.*)").unwrap();

    let mut shapes: Vec<Vec<String>> = vec![];
    let mut goals: Vec<Goal> = vec![];

    text.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if let Some(_) = re_shape_num.captures(line) {
                shapes.push(vec![]);
            } else if let Some(caps) = re_shape_line.captures(line) {
                let last_idx = shapes.len()-1;
                let last_shape = &mut shapes[last_idx];
                last_shape.push(caps[1].to_string());
            } else if let Some(caps) = re_space_goals.captures(line) {
                let x: usize = caps[1].parse().unwrap();
                let y: usize = caps[2].parse().unwrap();
                let reqs: Vec<usize> =
                    caps[3]
                    .split(" ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                goals.push(Goal{ x, y, reqs })
            }
        });

    (shapes, goals)
}

pub fn parse_1(text: &str) -> i64 {
    let (shapes, goals) = parse_text(text);

    // reddit tells us that simply checking for total available space
    // is enough... on real input only, not test input #facepalm

    let shape_space: Vec<usize> =
        shapes
        .iter()
        .map(|shape| {
            shape
                .iter()
                .map(|line| line
                     .as_bytes()
                     .iter()
                     .filter(|&c| *c == b'#')
                     .count())
                .sum()
        })
        .collect();

    goals.iter().map(|goal| {
        let avail = goal.x * goal.y;
        let needed =
            goal
            .reqs
            .iter()
            .enumerate()
            .map(|(i, count)| shape_space[i]*count)
            .sum();
        (avail, needed)
    })
        .inspect(|(avail, needed)|
                 println!("{} avail {} needed {}",
                          if avail >= needed {"V"} else {"."},
                          avail, needed))
        .filter(|(avail, needed)| avail >= needed)
        .count() as i64
}
