fn p1() {
    let mut grid = include_str!("../../input/day21")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for _ in 0..64 {
        let mut new_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
        grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &el)| match el {
                '#' => new_grid[i][j] = '#',
                'S' | 'O' => [(-1, 0), (1, 0), (0, 1), (0, -1)]
                    .iter()
                    .for_each(|(di, dj)| {
                        if (i as i32 + di) >= 0
                            && (j as i32 + dj) >= 0
                            && (i as i32 + di) < grid.len() as i32
                            && (j as i32 + dj) < grid[0].len() as i32
                            && grid[(i as i32 + di) as usize][(j as i32 + dj) as usize] == '.'
                        {
                            new_grid[(i as i32 + di) as usize][(j as i32 + dj) as usize] = 'O'
                        }
                    }),
                _ => (),
            });
        });
        grid = new_grid;
    }
    let res = grid
        .iter()
        .map(|row| row.iter().filter(|&&el| el == 'O').count())
        .sum::<usize>();
    println!("p1: {res}");
}

fn main() {
    p1();
}
