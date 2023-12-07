use std::collections::HashMap;

fn load_map() -> Vec<Vec<char>> {
    let line_len = include_str!("../../input/day3")
        .lines()
        .next()
        .unwrap()
        .len();

    let middle_of_grid = include_str!("../../input/day3").lines().map(|line| {
        ['.']
            .iter()
            .cloned()
            .chain(line.chars())
            .chain(vec!['.'])
            .collect::<Vec<char>>()
    });
    let grid = [vec!['.'; line_len + 2]]
        .iter()
        .cloned()
        .chain(middle_of_grid)
        .chain(vec![vec!['.'; line_len + 2]])
        .collect::<Vec<Vec<_>>>();
    grid
}

fn p1() {
    let grid = load_map();
    let dirs = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut acc = 0;
    let mut res = 0;
    let mut is_adjacent = false;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            match grid[i][j] {
                c if c.is_ascii_digit() => {
                    acc = acc * 10 + c.to_digit(10).unwrap();
                    is_adjacent = is_adjacent
                        || dirs.iter().any(|(dx, dy)| {
                            let c = grid[(i as i32 + dx) as usize][(j as i32 + dy) as usize];
                            c != '.' && !c.is_ascii_digit()
                        });
                }
                _ => {
                    if is_adjacent {
                        res += acc;
                    }
                    acc = 0;
                    is_adjacent = false;
                }
            };
        }
    }
    println!("{res:?}");
}

fn p2() {
    let grid = load_map();
    let dirs = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut gears = HashMap::new();
    let mut acc = 0;
    let mut adjacent = None;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            match grid[i][j] {
                c if c.is_ascii_digit() => {
                    acc = acc * 10 + c.to_digit(10).unwrap();
                    if adjacent.is_none() {
                        adjacent = dirs
                            .iter()
                            .filter_map(|(dx, dy)| {
                                match grid[(i as i32 + dx) as usize][(j as i32 + dy) as usize] {
                                    '*' => Some((i as i32 + dx, j as i32 + dy)),
                                    _ => None,
                                }
                            })
                            .next();
                    }
                }
                _ => {
                    if let Some(key) = adjacent {
                        gears.entry(key).or_insert(vec![]).push(acc);
                    }
                    acc = 0;
                    adjacent = None;
                }
            };
        }
    }
    let res = gears
        .iter()
        .filter(|(_, val)| val.len() == 2)
        .fold(0, |acc, (_, value)| acc + value[0] * value[1]);
    println!("Result: {res}");
}

fn main() {
    p1();
    p2();
}
