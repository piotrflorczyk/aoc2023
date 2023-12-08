use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn parse_input() -> (String, HashMap<String, (String, String)>) {
    let mut in_data = include_str!("../../input/day8").lines();
    let dir = in_data.next().unwrap().to_string();
    let map = in_data
        .skip(1)
        .map(|line| {
            let node = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            (node, (left, right))
        })
        .collect::<HashMap<_, _>>();
    (dir, map)
}

fn find_path(
    start: &String,
    dest: &HashSet<&String>,
    map: &HashMap<String, (String, String)>,
    dir: &String,
) -> usize {
    let mut curr = start;
    let mut iter = 0;
    while !dest.contains(curr) {
        match dir.chars().nth(iter % dir.len()).unwrap() {
            'L' => curr = &map[curr].0,
            'R' => curr = &map[curr].1,
            _ => unreachable!(),
        };
        iter += 1;
    }
    iter
}

fn p1() {
    let (dir, map) = parse_input();
    let steps = find_path(
        &"AAA".to_string(),
        &HashSet::from([&"ZZZ".to_string()]),
        &map,
        &dir,
    );
    println!("steps: {steps}");
}

fn p2() {
    let (dir, map) = parse_input();
    let starts = map.keys().filter(|node| node.ends_with('A'));
    let ends = map
        .keys()
        .filter(|node| node.ends_with('Z'))
        .collect::<HashSet<_>>();

    let steps = starts
        .map(|node| find_path(node, &ends, &map, &dir))
        .fold(1, |acc, distance| lcm(acc, distance));

    println!("{steps:?}");
}

fn main() {
    p1();
    p2();
}
