use std::collections::HashMap;

struct PuzzleInput {
    data: String,
    groups: Vec<usize>,
}

impl PuzzleInput {
    fn get_min_len(&self, group_idx: usize) -> usize {
        if group_idx >= self.groups.len() {
            0
        } else {
            self.groups
                .iter()
                .skip(group_idx)
                .fold(0, |acc, g| acc + g + 1)
                - 1
        }
    }

    fn from_p1(value: &str) -> Self {
        let splitted = value.split_once(' ').unwrap();
        Self {
            data: splitted.0.to_string(),
            groups: splitted
                .1
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn from_p2(value: &str) -> Self {
        let splitted = value.split_once(' ').unwrap();
        Self {
            data: (0..5).map(|_| splitted.0).collect::<Vec<_>>().join("?"),
            groups: splitted
                .1
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5),
        }
    }
}

fn calc_ways(
    cache: &mut HashMap<(usize, usize), usize>,
    puzzle: &PuzzleInput,
    data_idx: usize,
    group_idx: usize,
) -> usize {
    if cache.contains_key(&(data_idx, group_idx)) {
        return cache[&(data_idx, group_idx)];
    }
    if data_idx >= puzzle.data.len() {
        cache.insert(
            (data_idx, group_idx),
            (group_idx == puzzle.groups.len()) as usize,
        );
    } else if puzzle.data.len() - data_idx < puzzle.get_min_len(group_idx) {
        cache.insert((data_idx, group_idx), 0);
    } else {
        let res = match puzzle.data.as_bytes()[data_idx] {
            b'.' => calc_ways(cache, puzzle, data_idx + 1, group_idx),
            x if x == b'#' || x == b'?' => {
                let group_len = puzzle.data[data_idx..].split('.').next().unwrap().len();
                let mut res = if x == b'?' {
                    calc_ways(cache, puzzle, data_idx + 1, group_idx)
                } else {
                    0
                };
                if group_idx < puzzle.groups.len()
                    && group_len >= puzzle.groups[group_idx]
                    && (data_idx + puzzle.groups[group_idx] == puzzle.data.len()
                        || puzzle.data.as_bytes()[data_idx + puzzle.groups[group_idx]] != b'#')
                {
                    res += calc_ways(
                        cache,
                        puzzle,
                        data_idx + puzzle.groups[group_idx] + 1,
                        group_idx + 1,
                    )
                }
                res
            }
            _ => unreachable!(),
        };
        cache.insert((data_idx, group_idx), res);
    }
    *cache.get(&(data_idx, group_idx)).unwrap_or(&0)
}

fn p1() {
    let puzzles = include_str!("../../input/day12")
        .lines()
        .map(PuzzleInput::from_p1)
        .collect::<Vec<_>>();

    let ways = puzzles.iter().fold(0, |acc, puzzle| {
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        acc + calc_ways(&mut cache, puzzle, 0, 0)
    });

    println!("p1: {ways}");
}

fn p2() {
    let puzzles = include_str!("../../input/day12")
        .lines()
        .map(PuzzleInput::from_p2)
        .collect::<Vec<_>>();

    let ways = puzzles.iter().fold(0, |acc, puzzle| {
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        acc + calc_ways(&mut cache, puzzle, 0, 0)
    });

    println!("p2: {ways}");
}

fn main() {
    p1();
    p2();
}
