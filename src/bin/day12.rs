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

fn calc_ways_dp(puzzle: &PuzzleInput) -> usize {
    // format (input_idx, group_idx, group_cnt)
    let max_grp_cnt = puzzle.groups.iter().max().unwrap();
    let mut dp =
        vec![vec![vec![0; max_grp_cnt + 2]; puzzle.groups.len() + 2]; puzzle.data.len() + 2];

    dp[puzzle.data.len()][puzzle.groups.len()][0] = 1;
    dp[puzzle.data.len()][puzzle.groups.len() - 1][puzzle.groups[puzzle.groups.len() - 1]] = 1;

    for data_idx in (0..puzzle.data.len()).rev() {
        for group_idx in (0..puzzle.groups.len() + 1).rev() {
            let range = if group_idx < puzzle.groups.len() {
                puzzle.groups[group_idx]
            } else {
                puzzle.groups[puzzle.groups.len() - 1]
            };
            for group_cnt in (0..range + 1).rev() {
                match puzzle.data.as_bytes()[data_idx] {
                    b'.' if group_cnt == range => {
                        dp[data_idx][group_idx][group_cnt] = dp[data_idx + 1][group_idx + 1][0];
                    }
                    b'.' if group_cnt == 0 => {
                        dp[data_idx][group_idx][group_cnt] = dp[data_idx + 1][group_idx][0];
                    }
                    b'#' => {
                        dp[data_idx][group_idx][group_cnt] =
                            dp[data_idx + 1][group_idx][group_cnt + 1];
                    }
                    b'?' => {
                        let mut val = 0;
                        if group_cnt == range {
                            val += dp[data_idx + 1][group_idx + 1][0]
                        } else if group_cnt == 0 {
                            val += dp[data_idx + 1][group_idx][0];
                        }
                        val += dp[data_idx + 1][group_idx][group_cnt + 1];
                        dp[data_idx][group_idx][group_cnt] = val;
                    }
                    _ => (),
                }
            }
        }
    }
    dp[0][0][0]
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

fn p1_p2_dp() {
    let puzzles = include_str!("../../input/day12")
        .lines()
        .map(PuzzleInput::from_p2)
        .collect::<Vec<_>>();
    let ways = puzzles.iter().map(calc_ways_dp).sum::<usize>();
    println!("dp: {ways:?}");
}

fn main() {
    p1();
    p2();
    p1_p2_dp();
}
