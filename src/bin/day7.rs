use std::cmp::Reverse;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Card {
    Spot(u8),
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

impl Card {
    fn get_rank(&self) -> u8 {
        match self {
            Card::Spot(x) => *x,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
            Card::Joker => 1,
        }
    }
}

impl From<char> for Card {
    fn from(card: char) -> Self {
        card.to_digit(10)
            .map(|v| Card::Spot(v as u8))
            .unwrap_or_else(|| match card {
                'T' => Card::Spot(10),
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => unreachable!(),
            })
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_rank().cmp(&other.get_rank())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        let splitted = input.split_once(' ').unwrap();
        Self {
            cards: splitted.0.chars().map(Card::from).collect::<Vec<_>>(),
            bid: splitted.1.parse::<usize>().unwrap(),
        }
    }
}

impl Hand {
    fn get_rank(&self) -> u8 {
        let mut letter_counts = HashMap::new();
        let mut joker_count = 0;
        self.cards.iter().for_each(|c| {
            if *c == Card::Joker {
                joker_count += 1;
            } else {
                *letter_counts.entry(c).or_insert(0) += 1;
            }
        });
        let mut occurances = letter_counts.values().copied().collect::<Vec<_>>();
        occurances.sort_by_key(|v| Reverse(*v));

        if occurances.is_empty() {
            occurances.push(5);
        } else {
            occurances[0] += joker_count
        }

        match occurances[..] {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            _ => 0,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let left_rank = self.get_rank();
        let right_rank = other.get_rank();
        match left_rank.cmp(&right_rank) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calc_score(hands: &[Hand]) -> usize {
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (idx + 1) * hand.bid)
}

fn p1() {
    let mut hands = include_str!("../../input/day7")
        .lines()
        .map(Hand::from)
        .collect::<Vec<_>>();
    hands.sort();
    let score = calc_score(&hands);
    println!("score: {score:?}");
}

fn p2() {
    let mut hands = include_str!("../../input/day7")
        .lines()
        .map(Hand::from)
        .map(|hand| Hand {
            cards: hand
                .cards
                .iter()
                .map(|card| match card {
                    Card::Jack => Card::Joker,
                    x => *x,
                })
                .collect::<Vec<_>>(),
            bid: hand.bid,
        })
        .collect::<Vec<_>>();
    hands.sort();
    let score = calc_score(&hands);
    println!("score: {score:?}");
}

fn main() {
    p1();
    p2();
}
