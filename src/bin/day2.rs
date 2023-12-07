use std::collections::HashMap;

fn p1() {
    let max_values = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let res = include_str!("../../input/day2")
        .lines()
        .filter(|line| {
            line.split(": ").nth(1).unwrap().split("; ").all(|game| {
                game.split(", ").all(|pair| {
                    let elements = pair.split(' ').collect::<Vec<_>>();
                    elements[0].parse::<u32>().unwrap() <= max_values[elements[1]]
                })
            })
        })
        .map(|line| {
            line.split(": ")
                .nth(0)
                .unwrap()
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();

    println!("Res: {:?}", res);
}

fn p2() {
    let res = include_str!("../../input/day2")
        .lines()
        .map(|line| {
            let mut values = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            line.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .for_each(|game| {
                    game.split(", ").for_each(|pair| {
                        let elements = pair.split(' ').collect::<Vec<_>>();
                        if elements[0].parse::<u32>().unwrap() > values[elements[1]] {
                            values.insert(elements[1], elements[0].parse::<u32>().unwrap());
                        }
                    })
                });
            values["red"] * values["green"] * values["blue"]
        })
        .sum::<u32>();

    println!("Res: {:?}", res);
}

fn main() {
    p1();
    p2();
}
