fn parse_input() -> Vec<Vec<char>> {
    include_str!("../../input/day11")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_idx_pos(idx: usize, vector: &[usize]) -> usize {
    vector.binary_search(&idx).unwrap_or_else(|x| x)
}

fn get_galaxies(
    grid: &[Vec<char>],
    empty_rows: &[usize],
    empty_cols: &[usize],
    multiplier: usize,
) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &ch)| ch == '#')
                .map(|(j, _)| {
                    (
                        i + get_idx_pos(i, empty_rows) * multiplier,
                        j + get_idx_pos(j, empty_cols) * multiplier,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn calc_distances(galaxies: &[(usize, usize)]) -> i64 {
    galaxies.iter().enumerate().fold(0, |acc, (i, g)| {
        acc + galaxies.iter().skip(i + 1).fold(0, |acc2, g2| {
            acc2 + (g.0 as i64 - g2.0 as i64).abs() + (g.1 as i64 - g2.1 as i64).abs()
        })
    })
}

fn main() {
    let grid = parse_input();
    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|&x| x == '.'))
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();
    let empty_cols = (0..grid[0].len())
        .filter(|&j| (0..grid.len()).all(|i| grid[i][j] == '.'))
        .collect::<Vec<_>>();

    let galaxies_p1 = get_galaxies(&grid, &empty_rows, &empty_cols, 1);
    let distance_p1 = calc_distances(&galaxies_p1);
    let galaxies_p2 = get_galaxies(&grid, &empty_rows, &empty_cols, 1000000 - 1);
    let distance_p2 = calc_distances(&galaxies_p2);

    println!("p1: {distance_p1}");
    println!("p2: {distance_p2}");
}
