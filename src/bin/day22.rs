use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let splitted = value.split_once('~').unwrap();
        let p1_arr = splitted
            .0
            .split(',')
            .map(|coord| coord.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let p2_arr = splitted
            .1
            .split(',')
            .map(|coord| coord.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self {
            x: (min(p1_arr[0], p2_arr[0]), max(p1_arr[0], p2_arr[0])),
            y: (min(p1_arr[1], p2_arr[1]), max(p1_arr[1], p2_arr[1])),
            z: (min(p1_arr[2], p2_arr[2]), max(p1_arr[2], p2_arr[2])),
        }
    }
}

fn append_brick(space: &mut [Vec<Vec<usize>>], brick: &Brick, brick_no: usize) {
    (brick.z.0..=brick.z.1).for_each(|z| {
        (brick.y.0..=brick.y.1)
            .for_each(|y| (brick.x.0..=brick.x.1).for_each(|x| space[z][y][x] = brick_no))
    });
}

fn main() {
    let mut bricks = include_str!("../../input/day22")
        .lines()
        .map(Brick::from)
        .collect::<Vec<_>>();
    bricks.sort_by_key(|b| b.z.0);

    let max_x = bricks.iter().max_by_key(|b| b.x.1).unwrap().x.1;
    let max_y = bricks.iter().max_by_key(|b| b.y.1).unwrap().y.1;
    let max_z = bricks.iter().max_by_key(|b| b.z.1).unwrap().z.1;
    let mut space = vec![vec![vec![0usize; max_x + 1]; max_y + 1]; max_z + 1];
    (0..=max_y).for_each(|y| {
        (0..=max_x).for_each(|x| {
            space[0][y][x] = 1;
        })
    });

    let stable_bricks = bricks
        .iter()
        .enumerate()
        .map(|(idx, b)| {
            let diff = (1..max_z)
                .find(|i| {
                    !(b.y.0..=b.y.1).all(|y| (b.x.0..=b.x.1).all(|x| space[b.z.0 - i][y][x] == 0))
                })
                .unwrap();
            let mut new_brick = b.clone();
            new_brick.z.0 -= diff - 1;
            new_brick.z.1 -= diff - 1;
            append_brick(&mut space, &new_brick, idx + 2);
            new_brick
        })
        .collect::<Vec<_>>();

    let supported_by = stable_bricks
        .iter()
        .enumerate()
        .map(|(idx, b)| {
            let set = (b.y.0..=b.y.1)
                .flat_map(|y| {
                    (b.x.0..=b.x.1)
                        .filter(|&x| space[b.z.0 - 1][y][x] != 0)
                        .map(|x| space[b.z.0 - 1][y][x])
                        .collect::<HashSet<_>>()
                })
                .collect::<HashSet<_>>();
            (idx + 2, set)
        })
        .collect::<HashMap<_, _>>();

    // this will include ground
    let critical_bricks = supported_by
        .values()
        .filter(|v| v.len() == 1)
        .flatten()
        .collect::<HashSet<_>>();

    let redundand_bricks = bricks.len() - (critical_bricks.len() - 1);
    println!("p1: {redundand_bricks}");

    let mut supporting = HashMap::new();
    supported_by.iter().for_each(|(k, set)| {
        set.iter().for_each(|v| {
            supporting.entry(v).or_insert(HashSet::new()).insert(*k);
        })
    });

    let empty_set = HashSet::new();
    let total_fallen = supported_by
        .keys()
        .map(|&brick| {
            let mut fallen_bricks = HashSet::from([brick]);
            let mut queue = Vec::from_iter(supporting.get(&brick).unwrap_or(&empty_set).iter());
            while let Some(&b) = queue.pop() {
                if supported_by[&b].is_subset(&fallen_bricks) {
                    fallen_bricks.insert(b);
                    queue.extend(supporting.get(&b).unwrap_or(&empty_set).iter());
                }
            }
            fallen_bricks.len() - 1
        })
        .sum::<usize>();
    println!("p2: {total_fallen}");
}
