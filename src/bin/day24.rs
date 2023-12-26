use std::collections::HashSet;
use std::ops::Index;

#[derive(Debug, Clone)]
struct Coords {
    x: i64,
    y: i64,
    z: i64,
}

impl Index<usize> for Coords {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("wrong index"),
        }
    }
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

fn p1() {
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

fn p2() {
    let hailstones = include_str!("../../input/day24")
        .lines()
        .map(Hailstone::from)
        .collect::<Vec<_>>();

    let mut sets = vec![HashSet::from_iter(-1000..=1000); 3];
    (0..hailstones.len()).for_each(|i| {
        (i + 1..hailstones.len()).for_each(|j| {
            let h1 = &hailstones[i];
            let h2 = &hailstones[j];
            (0..3).for_each(|i| {
                if h1.velocity[i] == h2.velocity[i] {
                    let mut set = HashSet::from([h1.velocity[i]]);
                    let diff = h1.pos[i].abs_diff(h2.pos[i]);
                    for v in -1000..=1000 {
                        if v != h1.velocity[i] && diff % (v.abs_diff(h1.velocity[i])) == 0 {
                            set.insert(v);
                        }
                    }
                    sets[i] = sets[i].intersection(&set).copied().collect::<HashSet<_>>();
                }
            });
        })
    });
    let h1 = &hailstones[0];
    let h2 = &hailstones[1];
    let rock_velocity = Coords {
        x: *sets[0].iter().next().unwrap(),
        y: *sets[1].iter().next().unwrap(),
        z: *sets[2].iter().next().unwrap(),
    };

    //two lines one with rock velocity cutting through h1 and second through h2.
    //Point of their crossing is the initial rock position
    let m_h1 = (h1.velocity.y - rock_velocity.y) as f64 / (h1.velocity.x - rock_velocity.x) as f64;
    let m_h2 = (h2.velocity.y - rock_velocity.y) as f64 / (h2.velocity.x - rock_velocity.x) as f64;
    let c_h1 = h1.pos.y as f64 - m_h1 * (h1.pos.x as f64);
    let c_h2 = h2.pos.y as f64 - m_h2 * (h2.pos.x as f64);
    let x_pos = (c_h2 - c_h1) / (m_h1 - m_h2);
    let y_pos = m_h1 * x_pos + c_h1;
    let time = (x_pos - h1.pos.x as f64) / (h1.velocity.x - rock_velocity.x) as f64;
    let z_pos = h1.pos.z as f64 + (h1.velocity.z - rock_velocity.z) as f64 * time;

    let res = [x_pos as i64, y_pos as i64, z_pos as i64]
        .iter()
        .sum::<i64>();
    println!("p2: {res:?}");
}

fn main() {
    p1();
    p2();
}
