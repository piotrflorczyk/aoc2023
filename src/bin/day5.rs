use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
struct Range {
    dst: u64,
    src: u64,
    len: u64,
}

impl Range {
    fn dst_end(&self) -> u64 {
        self.dst + self.len
    }
    fn src_end(&self) -> u64 {
        self.src + self.len
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut tokens = s.split(' ');
        Self {
            dst: tokens.next().unwrap().parse::<u64>().unwrap(),
            src: tokens.next().unwrap().parse::<u64>().unwrap(),
            len: tokens.next().unwrap().parse::<u64>().unwrap(),
        }
    }
}

fn range_compare(val: u64, range: &Range) -> Ordering {
    if val >= range.src && val < range.src_end() {
        Ordering::Equal
    } else if val >= range.src_end() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn load_input() -> (Vec<u64>, Vec<Vec<Range>>) {
    let mut in_data = include_str!("../../input/day5").split("\r\n\r\n");
    let seeds = in_data
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|seed_no| seed_no.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = in_data
        .map(|section| {
            let mut map = section
                .split("\r\n")
                .skip(1)
                .map(Range::from)
                .collect::<Vec<_>>();
            map.sort_by_key(|range| range.src);
            map
        })
        .collect::<Vec<_>>();
    (seeds, maps)
}

fn p1() {
    let (seeds, maps) = load_input();
    let location = seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |acc, map| {
                match map.binary_search_by(|range| range_compare(acc, range)) {
                    Ok(idx) => map[idx].dst + (acc - map[idx].src),
                    Err(_) => acc,
                }
            })
        })
        .min()
        .unwrap();

    println!("location: {location}");
}

fn reduce_single_map(first_map: &[Range], second_map: &Vec<Range>) -> Vec<Range> {
    let mut res = Vec::new();
    for range in first_map.iter() {
        let mut curr = *range;
        while curr.len != 0 {
            let new_range = match second_map.binary_search_by(|r| range_compare(curr.dst, r)) {
                Ok(idx) => Range {
                    src: curr.src,
                    dst: second_map[idx].dst + (curr.dst - second_map[idx].src),
                    len: std::cmp::min(curr.len, second_map[idx].src_end() - curr.dst),
                },
                Err(idx) => {
                    if idx >= second_map.len() || curr.dst_end() < second_map[idx].src {
                        curr
                    } else {
                        Range {
                            src: curr.src,
                            dst: curr.dst,
                            len: second_map[idx].src - curr.dst,
                        }
                    }
                }
            };
            res.push(new_range);
            curr = Range {
                src: new_range.src_end(),
                dst: curr.dst + new_range.len,
                len: curr.len - new_range.len,
            };
        }
    }
    res
}

fn p2() {
    let (seeds, maps) = load_input();
    let mut seed_ranges = seeds
        .chunks(2)
        .map(|chunk| Range {
            dst: chunk[0],
            src: chunk[0],
            len: chunk[1],
        })
        .collect::<Vec<_>>();
    seed_ranges.sort_by_key(|range| range.src);

    let final_map = maps
        .iter()
        .fold(seed_ranges, |acc, map| reduce_single_map(&acc, map));

    let res = final_map.iter().min_by_key(|r| r.dst).unwrap().dst;
    println!("location: {res}");
}

fn main() {
    p1();
    p2();
}
