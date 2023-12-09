fn parse_input() -> Vec<Vec<i64>> {
    include_str!("../../input/day9")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn extrapolate((sum_end, sum_start): (i64, i64), v: &Vec<i64>) -> (i64, i64) {
    let mut curr = v.clone();
    let mut running_sum_end = 0;
    let mut running_sum_start = 0;
    let mut sign = 1;
    while !curr.iter().all(|&v| v == 0) {
        running_sum_end += curr.last().unwrap();
        running_sum_start += sign * curr.first().unwrap();
        sign *= -1;
        curr = curr
            .iter()
            .enumerate()
            .skip(1)
            .map(|(idx, num)| num - curr[idx - 1])
            .collect::<Vec<_>>();
    }
    (sum_end + running_sum_end, sum_start + running_sum_start)
}

fn main() {
    let (p1, p2) = parse_input().iter().fold((0, 0), extrapolate);
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
