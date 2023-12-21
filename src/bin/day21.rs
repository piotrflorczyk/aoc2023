fn load_grid() -> Vec<Vec<char>> {
    let mut grid = include_str!("../../input/day21")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let middle = (grid.len() / 2, grid[0].len() / 2);
    assert_eq!(grid[middle.0][middle.1], 'S');
    grid[middle.0][middle.1] = '.';
    grid
}

fn simulate_grid(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    iterations: usize,
) -> Vec<Vec<char>> {
    let mut curr_grid = grid.clone();
    curr_grid[start.0][start.1] = 'O';
    for _ in 0..iterations {
        let mut new_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
        curr_grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &el)| match el {
                '#' => new_grid[i][j] = '#',
                'S' | 'O' => [(-1, 0), (1, 0), (0, 1), (0, -1)]
                    .iter()
                    .for_each(|(di, dj)| {
                        if (i as i32 + di) >= 0
                            && (j as i32 + dj) >= 0
                            && (i as i32 + di) < grid.len() as i32
                            && (j as i32 + dj) < grid[0].len() as i32
                            && curr_grid[(i as i32 + di) as usize][(j as i32 + dj) as usize] == '.'
                        {
                            new_grid[(i as i32 + di) as usize][(j as i32 + dj) as usize] = 'O'
                        }
                    }),
                _ => (),
            });
        });
        curr_grid = new_grid;
    }
    curr_grid
}

fn calculate_positions(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|&&el| el == 'O').count())
        .sum::<usize>()
}

fn get_postions(grid: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> usize {
    let sim_grid = simulate_grid(grid, start, steps);
    calculate_positions(&sim_grid)
}

fn p1() {
    let grid = load_grid();
    let simulated_grid = simulate_grid(&grid, (grid.len() / 2, grid[0].len() / 2), 64);
    let res = calculate_positions(&simulated_grid);
    println!("p1: {res}");
}

fn p2() {
    let grid = load_grid();
    let full_cycles = 26501365 / grid.len();
    let reminder = 26501365 % grid.len();
    let mid = grid.len() / 2;
    let grid_len = grid.len();

    let odd = get_postions(&grid, (mid, mid), grid_len);
    let even = get_postions(&grid, (mid, mid), grid_len + 1);

    let left = get_postions(&grid, (mid, grid_len - 1), grid_len - 1);
    let right = get_postions(&grid, (mid, 0), grid_len - 1);
    let bottom = get_postions(&grid, (0, mid), grid_len - 1);
    let top = get_postions(&grid, (grid_len - 1, mid), grid_len - 1);

    let left_top_corner_small = get_postions(&grid, (0, 0), reminder - 1);
    let right_top_corner_small = get_postions(&grid, (0, grid_len - 1), reminder - 1);
    let left_bottom_corner_small = get_postions(&grid, (grid_len - 1, 0), reminder - 1);
    let right_bottom_corner_small = get_postions(&grid, (grid_len - 1, grid_len - 1), reminder - 1);

    let left_top_corner_big = get_postions(&grid, (0, 0), reminder + grid_len - 1);
    let right_top_corner_big = get_postions(&grid, (0, grid_len - 1), reminder + grid_len - 1);
    let left_bottom_corner_big = get_postions(&grid, (grid_len - 1, 0), reminder + grid_len - 1);
    let right_bottom_corner_big =
        get_postions(&grid, (grid_len - 1, grid_len - 1), reminder + grid_len - 1);

    let total_area = odd * (full_cycles - 1) * (full_cycles - 1)
        + even * (full_cycles) * full_cycles
        + left
        + right
        + top
        + bottom
        + (full_cycles - 1)
            * (left_bottom_corner_big
                + right_bottom_corner_big
                + left_top_corner_big
                + right_top_corner_big)
        + full_cycles
            * (left_top_corner_small
                + left_bottom_corner_small
                + right_bottom_corner_small
                + right_top_corner_small);

    println!("p2: {total_area}");
}

fn main() {
    p1();
    p2();
}
