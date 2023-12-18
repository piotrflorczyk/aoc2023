use std::cmp::{max, min};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn from(value: &str) -> Self {
        match value {
            "L" => Dir::Left,
            "R" => Dir::Right,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => unreachable!(),
        }
    }
    fn from_int(val: u8) -> Self {
        match val {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => unreachable!(),
        }
    }
}

impl Dir {
    fn dx(&self) -> i64 {
        match self {
            Dir::Left => -1,
            Dir::Right => 1,
            _ => 0,
        }
    }
    fn dy(&self) -> i64 {
        match self {
            Dir::Up => -1,
            Dir::Down => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Move {
    dir: Dir,
    steps: i64,
}

impl Move {
    fn from_p1(value: &str) -> Self {
        let mut splitted = value.split(' ');
        Self {
            dir: Dir::from(splitted.next().unwrap()),
            steps: splitted.next().unwrap().parse::<i64>().unwrap(),
        }
    }

    fn from_p2(value: &str) -> Self {
        let hex = value.split(' ').nth(2).unwrap();
        let val = i64::from_str_radix(&hex[2..hex.len() - 1], 16).unwrap();

        Self {
            dir: Dir::from_int((val & 0xF) as u8),
            steps: (val >> 4),
        }
    }
}

fn p1_slow() -> usize {
    let moves = include_str!("../../input/day18")
        .lines()
        .map(Move::from_p1)
        .collect::<Vec<_>>();

    let mut min_grid = (i64::MAX, i64::MAX);
    let mut max_grid = (i64::MIN, i64::MIN);
    let mut curr = (0, 0);
    moves.iter().for_each(|mov| {
        curr = (
            curr.0 + mov.steps * mov.dir.dy(),
            curr.1 + mov.steps * mov.dir.dx(),
        );
        min_grid.0 = min(min_grid.0, curr.0);
        min_grid.1 = min(min_grid.1, curr.1);
        max_grid.0 = max(max_grid.0, curr.0);
        max_grid.1 = max(max_grid.1, curr.1);
    });

    let mut grid = vec![
        vec![0; (max_grid.1 - min_grid.1 + 3) as usize];
        (max_grid.0 - min_grid.0 + 3) as usize
    ];
    let starting_point = (0 - min_grid.0 + 1, 0 - min_grid.1 + 1);

    let mut curr = starting_point;
    moves.iter().for_each(|mov| {
        let end = (
            curr.0 + mov.steps * mov.dir.dy(),
            curr.1 + mov.steps * mov.dir.dx(),
        );
        while curr != end {
            grid[curr.0 as usize][curr.1 as usize] = 1;
            curr = (curr.0 + mov.dir.dy(), curr.1 + mov.dir.dx());
        }
    });

    let mut queue = vec![(0i32, 0i32)];
    while let Some(point) = queue.pop() {
        if grid[point.0 as usize][point.1 as usize] != 2 {
            grid[point.0 as usize][point.1 as usize] = 2;
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .for_each(|(dy, dx)| {
                    let yy = point.0 + dy;
                    let xx = point.1 + dx;
                    if yy >= 0
                        && yy < grid.len() as i32
                        && xx >= 0
                        && xx < grid[0].len() as i32
                        && grid[yy as usize][xx as usize] == 0
                    {
                        queue.push((yy, xx));
                    }
                });
        }
    }
    grid.iter()
        .map(|row| row.iter().filter(|&&el| el < 2).count())
        .sum::<usize>()
}

fn shoelace_formula_solve(moves: &Vec<Move>) -> i64 {
    let mut curr = (0, 0);
    let mut area = 0;
    let mut perim = 0;
    for mov in moves {
        let next = (
            curr.0 + mov.dir.dx() * mov.steps,
            curr.1 + mov.dir.dy() * mov.steps,
        );
        area += curr.0 * next.1 - next.0 * curr.1;
        perim += (mov.dir.dx() + mov.dir.dy()).abs() * mov.steps;
        curr = next;
    }
    area += perim;
    area /= 2;
    area + 1
}

fn p1() -> i64 {
    let moves = include_str!("../../input/day18")
        .lines()
        .map(Move::from_p1)
        .collect::<Vec<_>>();
    shoelace_formula_solve(&moves)
}
fn p2() -> i64 {
    let moves = include_str!("../../input/day18")
        .lines()
        .map(Move::from_p2)
        .collect::<Vec<_>>();
    shoelace_formula_solve(&moves)
}

fn main() {
    let p1_slow = p1_slow();
    let p1 = p1();
    println!("p1: {p1} / {p1_slow}");
    let p2 = p2();
    println!("p2: {p2}");
}
