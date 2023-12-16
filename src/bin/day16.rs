#[repr(u8)]
#[derive(Clone, Copy)]
enum Dir {
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}

impl Dir {
    fn x_increment(&self) -> i32 {
        match self {
            Dir::Up => -1,
            Dir::Down => 1,
            _ => 0,
        }
    }
    fn y_increment(&self) -> i32 {
        match self {
            Dir::Left => -1,
            Dir::Right => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy)]
struct Beam {
    x: i32,
    y: i32,
    dir: Dir,
}

fn is_valid_beam(curr: &Beam, grid: &Vec<Vec<char>>) -> bool {
    curr.x >= 0 && curr.x < grid.len() as i32 && curr.y >= 0 && curr.y < grid[0].len() as i32
}

fn get_next_beams(curr: &Beam, grid: &[Vec<char>]) -> Vec<Beam> {
    let element = grid[curr.x as usize][curr.y as usize];
    let new_dirs = match (element, curr.dir) {
        ('.', _) | ('-', Dir::Left | Dir::Right) | ('|', Dir::Down | Dir::Up) => vec![curr.dir],
        ('/', Dir::Right) | ('\\', Dir::Left) => vec![Dir::Up],
        ('/', Dir::Down) | ('\\', Dir::Up) => vec![Dir::Left],
        ('/', Dir::Up) | ('\\', Dir::Down) => vec![Dir::Right],
        ('/', Dir::Left) | ('\\', Dir::Right) => vec![Dir::Down],
        ('|', Dir::Left | Dir::Right) => vec![Dir::Up, Dir::Down],
        ('-', Dir::Up | Dir::Down) => vec![Dir::Left, Dir::Right],
        _ => unreachable!(),
    };
    new_dirs
        .iter()
        .map(|new_dir| Beam {
            x: curr.x + new_dir.x_increment(),
            y: curr.y + new_dir.y_increment(),
            dir: *new_dir,
        })
        .collect::<Vec<_>>()
}

fn simulate_beam(grid: &Vec<Vec<char>>, starting_beam: &Beam) -> usize {
    let mut energized_grid = vec![vec![0; grid[0].len()]; grid.len()];
    let mut queue = vec![*starting_beam];
    while let Some(beam) = queue.pop() {
        if is_valid_beam(&beam, grid)
            && energized_grid[beam.x as usize][beam.y as usize] & beam.dir as u8 == 0
        {
            queue.append(&mut get_next_beams(&beam, grid));
            energized_grid[beam.x as usize][beam.y as usize] |= beam.dir as u8;
        }
    }

    energized_grid.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&&cell| cell != 0).count()
    })
}

fn main() {
    let grid = include_str!("../../input/day16")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let p1 = simulate_beam(
        &grid,
        &Beam {
            x: 0,
            y: 0,
            dir: Dir::Right,
        },
    );
    println!("p1: {p1:?}");
    // grid.len() == grid[0].len()
    let starting_beams = (0..grid.len())
        .flat_map(|i| {
            [
                Beam {
                    x: i as i32,
                    y: 0,
                    dir: Dir::Right,
                },
                Beam {
                    x: i as i32,
                    y: grid[0].len() as i32 - 1,
                    dir: Dir::Left,
                },
                Beam {
                    x: 0,
                    y: i as i32,
                    dir: Dir::Down,
                },
                Beam {
                    x: grid.len() as i32 - 1,
                    y: i as i32,
                    dir: Dir::Up,
                },
            ]
        })
        .collect::<Vec<_>>();
    let max_energized = starting_beams
        .iter()
        .map(|s| simulate_beam(&grid, s))
        .max()
        .unwrap();
    println!("p2: {max_energized:?}");
}
