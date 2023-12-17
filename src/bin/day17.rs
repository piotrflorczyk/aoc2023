use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Dir {
    None = 0,
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}
impl Dir {
    fn row_inc(&self) -> i32 {
        match &self {
            Dir::Up => -1,
            Dir::Down => 1,
            _ => 0,
        }
    }
    fn col_inc(&self) -> i32 {
        match &self {
            Dir::Left => -1,
            Dir::Right => 1,
            _ => 0,
        }
    }
    fn oposite(&self) -> Self {
        match &self {
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Right => Dir::Left,
            _ => Dir::None,
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct Path {
    cost: u32,
    pos: (i32, i32),
    dir: Dir,
    dir_cnt: u8,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Path {}

fn is_valid(pos: (i32, i32), grid: &Vec<Vec<u32>>) -> bool {
    pos.0 >= 0 && pos.0 < grid.len() as i32 && pos.1 >= 0 && pos.1 < grid[0].len() as i32
}

fn find_path(grid: &Vec<Vec<u32>>, min_moves: u8, max_moves: u8) -> Path {
    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![0u64; grid[0].len()]; grid.len()];
    heap.push(Path {
        cost: 0,
        pos: (0, 0),
        dir: Dir::None,
        dir_cnt: min_moves,
    });
    while let Some(path) = heap.pop() {
        if path.pos.0 == grid.len() as i32 - 1 && path.pos.1 == grid[0].len() as i32 - 1 {
            return path;
        }

        let compressed_dir = (path.dir as u64) << (4 * (path.dir_cnt - min_moves));
        if visited[path.pos.0 as usize][path.pos.1 as usize] & compressed_dir == 0 {
            visited[path.pos.0 as usize][path.pos.1 as usize] |= compressed_dir;
            [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
                .iter()
                .for_each(|&next_dir| {
                    if path.dir == next_dir && path.dir_cnt != max_moves {
                        let new_pos = (
                            path.pos.0 + next_dir.row_inc(),
                            path.pos.1 + next_dir.col_inc(),
                        );
                        if is_valid(new_pos, grid) {
                            heap.push(Path {
                                cost: path.cost + grid[new_pos.0 as usize][new_pos.1 as usize],
                                pos: new_pos,
                                dir: next_dir,
                                dir_cnt: path.dir_cnt + 1,
                            })
                        }
                    }
                    if next_dir != path.dir.oposite() && next_dir != path.dir {
                        let new_pos = (
                            path.pos.0 + next_dir.row_inc() * (min_moves as i32),
                            path.pos.1 + next_dir.col_inc() * (min_moves as i32),
                        );
                        if is_valid(new_pos, grid) {
                            heap.push(Path {
                                cost: path.cost
                                    + (1..min_moves + 1)
                                        .map(|i| {
                                            grid[(path.pos.0 + next_dir.row_inc() * i as i32)
                                                as usize]
                                                [(path.pos.1 + next_dir.col_inc() * i as i32)
                                                    as usize]
                                        })
                                        .sum::<u32>(),
                                pos: new_pos,
                                dir: next_dir,
                                dir_cnt: min_moves,
                            })
                        }
                    }
                });
        }
    }
    unreachable!()
}

fn main() {
    let grid = include_str!("../../input/day17")
        .lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as u32).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let p1 = find_path(&grid, 1, 3).cost;
    println!("p1: {p1:?}");

    let p2 = find_path(&grid, 4, 10).cost;
    println!("p2: {p2}");
}
