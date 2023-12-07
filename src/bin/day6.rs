fn p1() {
    let in_data = include_str!("../../input/day6")
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let times = &in_data[0];
    let distances = &in_data[1];

    let result = times
        .iter()
        .zip(distances)
        .map(|(&time, &dist)| {
            let first_passing =
                ((time as f64 - ((time * time - 4 * dist) as f64).sqrt()) / 2f64).ceil();
            time - 2 * (first_passing as u64) + 1
        })
        .product::<u64>();

    println!("{result}");
}

fn p2() {
    let in_data = include_str!("../../input/day6")
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let time = in_data[0];
    let dist = in_data[1];

    let first_passing = ((time as f64 - ((time * time - 4 * dist) as f64).sqrt()) / 2f64).ceil();
    let result = time - 2 * (first_passing as u64) + 1;

    println!("{result:?}");
}

fn main() {
    p1();
    p2();
}
