use std::cmp::max;
use std::collections::{HashMap, HashSet};

type Coords = (usize, usize);
type Node = (Coords, usize);

fn load_grid() -> Vec<Vec<char>> {
    include_str!("../../input/day23")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
fn p1() {
    let grid = load_grid();
    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);
    let mut max_len = 0;

    let visited = vec![vec![0; grid[0].len()]; grid.len()];
    let mut queue = vec![(start, visited)];

    while let Some(((x, y), mut visited)) = queue.pop() {
        if (x, y) == end {
            let path_len = visited
                .iter()
                .map(|row| row.iter().sum::<i32>())
                .sum::<i32>();
            max_len = max(max_len, path_len);
        }
        visited[x][y] = 1;

        let valid_dirs = [(-1, 0), (0, 1), (0, -1), (1, 0)]
            .iter()
            .filter(|(dx, dy)| {
                (x as i32 + dx) >= 0
                    && (x as i32 + dx) < grid.len() as i32
                    && (y as i32 + dy) >= 0
                    && (y as i32 + dy) < grid[0].len() as i32
                    && visited[(x as i32 + dx) as usize][(y as i32 + dy) as usize] == 0
                    && match grid[(x as i32 + dx) as usize][(y as i32 + dy) as usize] {
                        '#' => false,
                        '>' => *dy == 1,
                        '<' => *dy == -1,
                        'v' => *dx == 1,
                        '^' => *dx == -1,
                        _ => true,
                    }
            })
            .collect::<Vec<_>>();
        valid_dirs.iter().skip(1).for_each(|(dx, dy)| {
            queue.push((
                ((x as i32 + dx) as usize, (y as i32 + dy) as usize),
                visited.clone(),
            ));
        });
        if !valid_dirs.is_empty() {
            let (dx, dy) = valid_dirs[0];
            queue.push((
                ((x as i32 + dx) as usize, (y as i32 + dy) as usize),
                visited,
            ));
        }
    }
    println!("p1: {max_len}");
}

fn compress_graph(grid: &Vec<Vec<char>>, start: Coords, end: Coords) -> HashMap<Coords, Vec<Node>> {
    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
    let mut queue = vec![(start, start, 0)];

    // compress graph
    let mut graph: HashMap<Coords, Vec<Node>> = HashMap::new();
    while let Some(((x, y), start, size)) = queue.pop() {
        if (x, y) == end {
            graph.entry(start).or_default().push(((x, y), size));
            graph.entry((x, y)).or_default().push((start, size));
        }
        let valid_dirs = [(-1, 0), (0, 1), (0, -1), (1, 0)]
            .iter()
            .filter(|(dx, dy)| {
                (x as i32 + dx) >= 0
                    && (x as i32 + dx) < grid.len() as i32
                    && (y as i32 + dy) >= 0
                    && (y as i32 + dy) < grid[0].len() as i32
                    && grid[(x as i32 + dx) as usize][(y as i32 + dy) as usize] != '#'
            })
            .collect::<Vec<_>>();
        if valid_dirs.len() <= 2 {
            visited[x][y] = 1;
            valid_dirs.iter().for_each(|(dx, dy)| {
                let vstd = visited[(x as i32 + dx) as usize][(y as i32 + dy) as usize];
                if vstd != 1 && !(size == 0 && vstd == 2) {
                    queue.push((
                        ((x as i32 + dx) as usize, (y as i32 + dy) as usize),
                        start,
                        size + 1,
                    ));
                }
            });
        } else {
            visited[x][y] = 2;
            let neighbours = valid_dirs
                .iter()
                .map(|(dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize))
                .filter(|(x, y)| visited[*x][*y] == 0)
                .collect::<Vec<_>>();
            neighbours.iter().for_each(|n| queue.push((*n, (x, y), 0)));

            graph.entry(start).or_default().push(((x, y), size + 1));
            graph.entry((x, y)).or_default().push((start, size + 1));
        }
    }
    graph
}

fn dfs(
    graph: &HashMap<Coords, Vec<Node>>,
    visited: &mut HashSet<Coords>,
    v: Coords,
    end: Coords,
) -> Option<usize> {
    if v == end {
        return Some(0);
    }
    let mut max_path = None;
    graph[&v].iter().for_each(|&(n, weight)| {
        if !visited.contains(&n) {
            visited.insert(n);
            if let Some(dist) = dfs(graph, visited, n, end) {
                max_path = Some(max_path.unwrap_or(0).max(weight + dist));
            }
            visited.remove(&n);
        }
    });
    max_path
}

fn p2() {
    let grid = load_grid();
    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);

    let graph = compress_graph(&grid, start, end);
    let mut visited = HashSet::new();
    visited.insert(start);
    let max_path = dfs(&graph, &mut visited, start, end).unwrap();
    println!("p2: {max_path}");

    // dfs2

    let mut max_path = 0;
    let mut queue = vec![(start, HashSet::new(), 0)];
    while let Some((v, mut visited, path_len)) = queue.pop() {
        if visited.contains(&v) {
            continue;
        }
        visited.insert(v);
        if v == end {
            max_path = max(max_path, path_len);
        }
        graph[&v].iter().for_each(|(neighbour, weight)| {
            if !visited.contains(neighbour) {
                queue.push((*neighbour, visited.clone(), path_len + weight));
            }
        });
    }
    println!("p2_alt: {max_path}");
}

fn main() {
    p1();
    p2();
}
