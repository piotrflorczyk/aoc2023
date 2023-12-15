fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0u8, |hash, ch| hash.wrapping_add(ch).wrapping_mul(17))
}

fn p1() {
    let strings = include_str!("../../input/day15").split(',');
    let hash = strings.map(|s| hash(s) as u32).sum::<u32>();
    println!("{hash:?}");
}

fn p2() {
    let strings = include_str!("../../input/day15").split(',');
    let mut boxes = vec![Vec::<(&str, usize)>::new(); 256];
    strings.for_each(|s| {
        let split_idx: usize = s.find(|ch: char| !ch.is_alphabetic()).unwrap();
        let label = &s[0..split_idx];
        let hash_label = hash(label);
        let maybe_idx = boxes[hash_label as usize]
            .iter()
            .position(|(lbl, _)| label.eq(*lbl));
        let op = s.chars().nth(split_idx).unwrap();
        match op {
            '-' => {
                if let Some(idx) = maybe_idx {
                    boxes[hash_label as usize].remove(idx);
                }
            }
            '=' => {
                let num = s[split_idx + 1..].parse::<usize>().unwrap();
                if let Some(idx) = maybe_idx {
                    boxes[hash_label as usize][idx].1 = num;
                } else {
                    boxes[hash_label as usize].push((label, num));
                }
            }
            _ => unreachable!(),
        }
    });

    let focusing_power = boxes
        .iter()
        .enumerate()
        .map(|(i, boxx)| {
            boxx.iter()
                .enumerate()
                .fold(0, |acc, (j, lense)| acc + (i + 1) * (j + 1) * lense.1)
        })
        .sum::<usize>();

    println!("{focusing_power:?}");
}

fn main() {
    p1();
    p2();
}
