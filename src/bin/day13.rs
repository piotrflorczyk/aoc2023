use std::cmp;

fn compare_rows(idxa: usize, idxb: usize, grid: &[Vec<char>]) -> i32 {
    (0..grid[0].len()).fold(0, |acc, i| {
        acc + (grid[idxa][i] as i32 - grid[idxb][i] as i32).abs()
    })
}

fn compare_cols(idxa: usize, idxb: usize, grid: &[Vec<char>]) -> i32 {
    (0..grid.len()).fold(0, |acc, i| {
        acc + (grid[i][idxa] as i32 - grid[i][idxb] as i32).abs()
    })
}

fn load_input() -> Vec<Vec<Vec<char>>> {
    include_str!("../../input/day13")
        .split("\r\n\r\n")
        .map(|grid| {
            grid.lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn calculate_reflections(grids: &[Vec<Vec<char>>], difference: i32) -> usize {
    grids.iter().fold(0, |acc, grid| {
        let row_reflection = (1..grid.len()).find(|&i| {
            let reflection_size = cmp::min(i, grid.len() - i);
            let diff = (1..reflection_size + 1)
                .fold(0, |acc, j| acc + compare_rows(i - j, i + j - 1, grid));
            diff == difference
        });
        let col_reflection = (1..grid[0].len()).find(|&i| {
            let reflection_size = cmp::min(i, grid[0].len() - i);
            let diff = (1..reflection_size + 1)
                .fold(0, |acc, j| acc + compare_cols(i - j, i + j - 1, grid));
            diff == difference
        });
        acc + col_reflection.unwrap_or_else(|| row_reflection.unwrap() * 100)
    })
}

fn main() {
    let grids = load_input();
    let p1 = calculate_reflections(&grids, 0);
    let p2 = calculate_reflections(&grids, ('#' as i32 - '.' as i32).abs());
    println!("p1: {p1}");
    println!("p2: {p2}");
}
