fn p1() {
    let res = include_str!("../../input/day1")
        .lines()
        .map(|line| {
            let first_idx = line.find(|c: char| c.is_ascii_digit()).unwrap();
            let second_idx = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
            line.chars().nth(first_idx).unwrap().to_digit(10).unwrap() * 10
                + line.chars().nth(second_idx).unwrap().to_digit(10).unwrap()
        })
        .sum::<u32>();
    println!("Sum: {:?}", res);
}

fn p2() {
    let word_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let res = include_str!("../../input/day1")
        .lines()
        .map(|line| {
            let first_digit = word_digits
                .iter()
                .filter_map(|word| line.find(word).map(|idx| (idx, word)))
                .min_by_key(|(idx, _)| *idx)
                .map(|(_, word)| ((word_digits.iter().position(|x| x == word).unwrap()) % 9) + 1)
                .unwrap();
            let last_digit = word_digits
                .iter()
                .filter_map(|word| line.rfind(word).map(|idx| (idx, word)))
                .max_by_key(|(idx, _)| *idx)
                .map(|(_, word)| ((word_digits.iter().position(|x| x == word).unwrap()) % 9) + 1)
                .unwrap();

            // println!("{:?} {:?}", first_digit, last_digit);
            first_digit * 10 + last_digit
        })
        .sum::<usize>();
    println!("Sum: {:?}", res);
}

fn main() {
    p1();
    p2();
}
