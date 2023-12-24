#[derive(Debug, Clone)]
struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Coords {
    fn from(value: &str) -> Self {
        let mut splitted = value.split(", ");
        Self {
            x: splitted.next().unwrap().trim().parse::<i64>().unwrap(),
            y: splitted.next().unwrap().trim().parse::<i64>().unwrap(),
            z: splitted.next().unwrap().trim().parse::<i64>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Hailstone {
    pos: Coords,
    velocity: Coords,
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let splitted = value.split_once(" @ ").unwrap();
        Self {
            pos: Coords::from(splitted.0),
            velocity: Coords::from(splitted.1),
        }
    }
}

fn get_linear_equation_params(h: &Hailstone) -> (f64, f64) {
    let m = h.velocity.y as f64 / h.velocity.x as f64;
    let b = h.pos.y as f64 - m * (h.pos.x as f64);
    (m, b)
}

fn compare_with_time(f1: f64, f2: f64, velocity: i64) -> bool {
    (f2 < f1 && velocity < 0) || (f2 > f1 && velocity > 0)
}

fn main() {
    let hailstones = include_str!("../../input/day24")
        .lines()
        .map(Hailstone::from)
        .collect::<Vec<_>>();

    let min_range = 200000000000000f64;
    let max_range = 400000000000000f64;
    let intersections = (0..hailstones.len())
        .map(|i| {
            (i + 1..hailstones.len())
                .filter(|j| {
                    let h1 = &hailstones[i];
                    let h2 = &hailstones[*j];
                    let (m1, b1) = get_linear_equation_params(h1);
                    let (m2, b2) = get_linear_equation_params(h2);
                    let x = (-b2 + b1) / (-m1 + m2);
                    let y = (b1 * m2 - b2 * m1) / (-m1 + m2);
                    m1 != m2
                        && x > min_range
                        && x < max_range
                        && y > min_range
                        && y < max_range
                        && compare_with_time(h1.pos.x as f64, x, h1.velocity.x)
                        && compare_with_time(h2.pos.x as f64, x, h2.velocity.x)
                        && compare_with_time(h1.pos.y as f64, y, h1.velocity.y)
                        && compare_with_time(h2.pos.y as f64, y, h2.velocity.y)
                })
                .count()
        })
        .sum::<usize>();
    println!("p1: {intersections}");
}
