fn load_grid() -> Vec<Vec<char>> {
    let line_len = include_str!("../../input/day10")
        .lines()
        .next()
        .unwrap()
        .len();
    let middle_of_grid = include_str!("../../input/day10").lines().map(|line| {
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn negate(&self) -> Self {
        match self {
            Dir::Down => Dir::Up,
            Dir::Up => Dir::Down,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

fn transform_dir(symbol: char, dir: &Dir) -> Option<Dir> {
    match (dir, symbol) {
        (&x, '|') if x == Dir::Up || x == Dir::Down => Some(x),
        (&x, '-') if x == Dir::Left || x == Dir::Right => Some(x),
        (Dir::Down, 'L') => Some(Dir::Right),
        (Dir::Left, 'L') => Some(Dir::Up),
        (Dir::Down, 'J') => Some(Dir::Left),
        (Dir::Right, 'J') => Some(Dir::Up),
        (Dir::Up, '7') => Some(Dir::Left),
        (Dir::Right, '7') => Some(Dir::Down),
        (Dir::Up, 'F') => Some(Dir::Right),
        (Dir::Left, 'F') => Some(Dir::Down),
        _ => None,
    }
}

fn map_to_char(dira: &Dir, dirb: &Dir) -> char {
    match (dira, dirb) {
        (Dir::Down, Dir::Left) | (Dir::Right, Dir::Up) => 'J',
        (Dir::Down, Dir::Right) | (Dir::Left, Dir::Up) => 'L',
        (Dir::Up, Dir::Left) | (Dir::Right, Dir::Down) => '7',
        (Dir::Up, Dir::Right) | (Dir::Left, Dir::Down) => 'F',
        _ => unreachable!(),
    }
}

fn next_coordinate((i, j): (usize, usize), dir: &Dir) -> (usize, usize) {
    match dir {
        Dir::Down => (i + 1, j),
        Dir::Up => (i - 1, j),
        Dir::Right => (i, j + 1),
        Dir::Left => (i, j - 1),
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn main() {
    let grid = load_grid();
    let (startx, starty) = find_start(&grid);
    let start_dirs = [Dir::Up, Dir::Down, Dir::Right, Dir::Left]
        .iter()
        .filter(|dir| {
            let (i, j) = next_coordinate((startx, starty), dir);
            transform_dir(grid[i][j], dir).is_some()
        })
        .collect::<Vec<_>>();
    assert_eq!(start_dirs.len(), 2);

    let (mut patha, mut pathb) = ((startx, starty), (startx, starty));
    let (mut dira, mut dirb) = (*start_dirs[0], *start_dirs[1]);

    let mut new_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
    new_grid[startx][starty] = map_to_char(&dira.negate(), &dirb);
    let mut steps = 0;
    while patha != pathb || patha == (startx, starty) {
        patha = next_coordinate(patha, &dira);
        dira = transform_dir(grid[patha.0][patha.1], &dira).unwrap();

        pathb = next_coordinate(pathb, &dirb);
        dirb = transform_dir(grid[pathb.0][pathb.1], &dirb).unwrap();

        new_grid[patha.0][patha.1] = grid[patha.0][patha.1];
        new_grid[pathb.0][pathb.1] = grid[pathb.0][pathb.1];
        steps += 1;
    }

    let mut tmp = vec![vec![0; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        let mut inside = false;
        for j in 0..grid[i].len() {
            match new_grid[i][j] {
                '.' => tmp[i][j] += if inside { 1 } else { 0 },
                '|' | 'F' | '7' => inside = !inside,
                _ => (),
            }
        }
    }

    let count = tmp.iter().fold(0, |acc, row| acc + row.iter().sum::<i32>());
    println!("steps (p1): {steps}");
    println!("count (p2): {count:?}");
    /*new_grid.iter().enumerate().for_each(|(i, l)| {
        l.iter().enumerate().for_each(|(j, ch)| match tmp[i][j] {
            1 => print!("I"),
            _ => print!("{ch}"),
        });
        print!("\n");
    });*/
}
