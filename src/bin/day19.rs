use std::cmp;
use std::collections::HashMap;

type Part = HashMap<String, u64>;
fn parse_part(value: &str) -> Part {
    value[1..value.len() - 1]
        .split(',')
        .map(|el| {
            let splitted = el.split_once('=').unwrap();
            (splitted.0.to_string(), splitted.1.parse::<u64>().unwrap())
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Rule {
    value: String,
    op: String,
    number: u64,
    destination: String,
}

impl Rule {
    fn from(value: &str) -> Self {
        match value.split_once(':') {
            Some((rule_str, destination)) => Self {
                value: rule_str[0..1].to_string(),
                op: rule_str[1..2].to_string(),
                number: rule_str[2..].parse::<u64>().unwrap(),
                destination: destination.to_string(),
            },
            None => Self {
                value: "x".to_string(),
                op: "".to_string(),
                number: 0,
                destination: value.to_string(),
            },
        }
    }
    fn matches_agains_part(&self, part: &Part) -> bool {
        match self.op.as_str() {
            "<" => part[&self.value] < self.number,
            ">" => part[&self.value] > self.number,
            _ => true,
        }
    }

    fn split_ranges(&self, ranges: &Ranges) -> (Ranges, Ranges) {
        let mut passing = ranges.clone();
        let mut rest = ranges.clone();
        let range = ranges[&self.value];
        let new_range = if self.op == "<" {
            (
                (range.0, cmp::min(range.1, self.number - 1)),
                (cmp::max(range.0, self.number), range.1),
            )
        } else if self.op == ">" {
            (
                (cmp::max(range.0, self.number + 1), range.1),
                (range.0, cmp::min(range.1, self.number)),
            )
        } else {
            ((range.0, range.1), (1, 0))
        };
        passing.insert(self.value.clone(), new_range.0);
        rest.insert(self.value.clone(), new_range.1);
        (passing, rest)
    }
}

type RuleList = Vec<Rule>;
type Workflows = HashMap<String, RuleList>;

fn parse_rule_list(value: &str) -> (String, RuleList) {
    let (name, rules_str) = value.split_once('{').unwrap();
    let rules = rules_str[0..rules_str.len() - 1]
        .split(',')
        .map(Rule::from)
        .collect::<Vec<_>>();
    (name.to_string(), rules)
}

fn apply_rule_list(rule_list: &RuleList, part: &Part) -> String {
    rule_list
        .iter()
        .find(|&r| r.matches_agains_part(part))
        .unwrap()
        .destination
        .clone()
}

fn resolve_part(workflows: &Workflows, part: &Part) -> String {
    let mut curr = "in".to_string();
    while curr != "A" && curr != "R" {
        let rule = &workflows[&curr];
        curr = apply_rule_list(rule, part);
    }
    curr
}

fn p1() {
    let (wokrflows_str, parts_str) = include_str!("../../input/day19")
        .split_once("\r\n\r\n")
        .unwrap();
    let workflows = wokrflows_str.lines().map(parse_rule_list).collect();
    let parts = parts_str.lines().map(parse_part).collect::<Vec<_>>();

    let match_parts_sum = parts
        .iter()
        .filter(|part| resolve_part(&workflows, part) == "A")
        .map(|part| part.values().sum::<u64>())
        .sum::<u64>();
    println!("p1: {match_parts_sum:?}");
}

type Ranges = HashMap<String, (u64, u64)>;

fn calc_combinations(ranges: &Ranges) -> u64 {
    ranges
        .values()
        .map(|(min, max)| if max >= min { max - min + 1 } else { 0 })
        .product::<u64>()
}

fn unwrap_ruleset(name: String, ranges: &Ranges, workflows: &Workflows) -> u64 {
    let rule_list = &workflows[&name];
    let mut curr_ranges = ranges.clone();
    rule_list
        .iter()
        .map(|rule| {
            let (passing, rest) = rule.split_ranges(&curr_ranges);
            curr_ranges = rest;
            match rule.destination.as_str() {
                "A" => calc_combinations(&passing),
                "R" => 0,
                x => unwrap_ruleset(x.to_string(), &passing, workflows),
            }
        })
        .sum::<u64>()
}

fn p2() {
    let (wokrflows_str, _) = include_str!("../../input/day19")
        .split_once("\r\n\r\n")
        .unwrap();
    let workflows = wokrflows_str.lines().map(parse_rule_list).collect();
    let ranges = Ranges::from([
        ("x".to_string(), (1, 4000)),
        ("m".to_string(), (1, 4000)),
        ("a".to_string(), (1, 4000)),
        ("s".to_string(), (1, 4000)),
    ]);
    let combinations = unwrap_ruleset("in".to_string(), &ranges, &workflows);
    println!("p2: {combinations}");
}

fn main() {
    p1();
    p2();
}
