use std::cmp;
use std::collections::HashSet;

fn p1() {
    let result = include_str!("../../input/day4")
        .lines()
        .map(|line| {
            let data = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let winning: HashSet<u32> = HashSet::from_iter(
                data.0
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap()),
            );
            let picked: HashSet<u32> = HashSet::from_iter(
                data.1
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap()),
            );
            picked.intersection(&winning).count()
        })
        .map(|cnt| if cnt != 0 { 1 << (cnt - 1) } else { 0 })
        .sum::<usize>();
    println!("p1: {result:?}");
}

fn p2() {
    let num_scorecards = include_str!("../../input/day4").lines().count();
    let mut scorecards_weight = vec![1; num_scorecards];
    include_str!("../../input/day4")
        .lines()
        .map(|line| {
            let data = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let winning: HashSet<u32> = HashSet::from_iter(
                data.0
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap()),
            );
            let picked: HashSet<u32> = HashSet::from_iter(
                data.1
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap()),
            );
            picked.intersection(&winning).count()
        })
        .enumerate()
        .for_each(|(idx, count)| {
            for i in idx + 1..cmp::min(idx + 1 + count, scorecards_weight.len()) {
                scorecards_weight[i] += scorecards_weight[idx];
            }
        });

    let result = scorecards_weight.iter().sum::<usize>();
    println!("p2: {result:?}");
}

fn main() {
    p1();
    p2();
}
