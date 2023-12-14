use std::cmp::Reverse;
use std::collections::HashMap;

fn get_rocks(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &ch)| ch == 'O')
                .map(|(j, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn simulate_north(grid: &mut [Vec<char>]) {
    let rocks = get_rocks(grid);

    rocks.iter().for_each(|(i, j)| {
        grid[*i][*j] = '.';
        let new_i = (0..i + 1)
            .rev()
            .find(|c| grid[*c][*j] != '.')
            .map(|x| x + 1)
            .unwrap_or(0);
        grid[new_i][*j] = 'O';
    });
}

fn simulate_south(grid: &mut Vec<Vec<char>>) {
    let mut rocks = get_rocks(grid);
    rocks.reverse();

    rocks.iter().for_each(|(i, j)| {
        grid[*i][*j] = '.';
        let new_i = (*i..grid.len())
            .find(|c| grid[*c][*j] != '.')
            .map(|x| x - 1)
            .unwrap_or(grid.len() - 1);
        grid[new_i][*j] = 'O';
    });
}

fn simulate_west(grid: &mut [Vec<char>]) {
    let mut rocks = get_rocks(grid);
    rocks.sort_by_key(|(_, j)| *j);

    rocks.iter().for_each(|(i, j)| {
        grid[*i][*j] = '.';
        let new_j = (0..j + 1)
            .rev()
            .find(|c| grid[*i][*c] != '.')
            .map(|x| x + 1)
            .unwrap_or(0);
        grid[*i][new_j] = 'O';
    });
}

fn simulate_east(grid: &mut [Vec<char>]) {
    let mut rocks = get_rocks(grid);
    rocks.sort_by_key(|(_, j)| Reverse(*j));

    rocks.iter().for_each(|(i, j)| {
        grid[*i][*j] = '.';
        let new_j = (*j..grid[0].len())
            .find(|c| grid[*i][*c] != '.')
            .map(|x| x - 1)
            .unwrap_or(grid[0].len() - 1);
        grid[*i][new_j] = 'O';
    });
}

fn calc_weight(rocks: &[(usize, usize)], grid_len: usize) -> usize {
    rocks.iter().map(|(i, _)| grid_len - i).sum::<usize>()
}

fn load_grid() -> Vec<Vec<char>> {
    include_str!("../../input/day14")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn p1() {
    let mut grid = load_grid();
    simulate_north(&mut grid);
    let rocks = get_rocks(&grid);
    let weight = calc_weight(&rocks, grid.len());
    println!("p1 weight {weight}");
}

fn p2() {
    let mut grid = load_grid();
    let mut map: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    let start_cycle = (0..)
        .find_map(|x| {
            simulate_north(&mut grid);
            simulate_west(&mut grid);
            simulate_south(&mut grid);
            simulate_east(&mut grid);
            let rocks = get_rocks(&grid);
            map.insert(rocks, x)
        })
        .unwrap();
    let end_cycle = map.values().max().unwrap();
    let idx = start_cycle + ((1000000000 - (start_cycle + 1)) % (end_cycle - start_cycle));
    let rocks = map.iter().find(|(_, &v)| v == idx).unwrap().0;
    let weight = calc_weight(rocks, grid.len());
    println!("p2 weight: {weight}");
}

fn main() {
    p1();
    p2();
}
