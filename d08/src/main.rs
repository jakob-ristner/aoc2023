use num::Integer;
use std::collections::{HashMap, VecDeque};

use regex::Regex;
#[derive(Debug, Hash, Clone, Copy)]
enum Dir {
    Left,
    Right,
}
type NodeMap = HashMap<String, (String, String)>;

fn main() {
    let (map, dirs) = parse();
    let starts = part_2_starting(&map);
    let sol1 = part_1(&map, &dirs);
    let periods: Vec<u128> = starts
        .iter()
        .map(|s| period(&map, &dirs, s.to_string()))
        .collect();
    let sol2: u128 = periods.iter().fold(1, |acc, num| acc.lcm(num));
    println!("part 1: {}", sol1);
    println!("part 2: {}", sol2);
}

fn period(map: &NodeMap, dirs: &VecDeque<Dir>, start: String) -> u128 {
    let mut curr = start;
    let mut points: Vec<u128> = Vec::new();
    let mut dirs = dirs.clone();
    for i in 0.. {
        if curr.chars().last().unwrap() == 'Z' {
            points.push(i);
        }
        if points.len() == 2 {
            return points[1] - points[0];
        }
        let dir = dirs.pop_front().unwrap();
        let (l, r) = map.get(&curr).unwrap();
        curr = match dir {
            Dir::Left => l.clone(),
            Dir::Right => r.clone(),
        };
        dirs.push_back(dir);
    }
    unreachable!();
}

fn part_1(map: &NodeMap, dirs: &VecDeque<Dir>) -> u128 {
    let mut curr = "AAA".to_string();
    let mut dirs = dirs.clone();
    for i in 0.. {
        if curr == "ZZZ" {
            return i;
        }
        let dir = dirs.pop_front().unwrap();
        let (l, r) = map.get(&curr).unwrap();
        curr = match dir {
            Dir::Left => l.clone(),
            Dir::Right => r.clone(),
        };
        dirs.push_back(dir);
    }
    unreachable!();
}

fn part_2_starting(map: &NodeMap) -> Vec<String> {
    map.clone()
        .into_keys()
        .filter(|val| val.chars().last().unwrap() == 'A')
        .collect()
}

fn parse() -> (NodeMap, VecDeque<Dir>) {
    let re = Regex::new(r"(?<from>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
    let (dirs_raw, rest) = include_str!("../input.txt").split_once("\n\n").unwrap();
    (
        rest.lines()
            .map(|line| {
                let cap = re.captures(line).unwrap();
                let from = cap.name("from").unwrap().as_str().to_string();
                let left = cap.name("left").unwrap().as_str().to_string();
                let right = cap.name("right").unwrap().as_str().to_string();
                (from, (left, right))
            })
            .collect(),
        (dirs_raw
            .chars()
            .map(|c| {
                use self::Dir::*;
                match c {
                    'L' => Left,
                    'R' => Right,
                    _ => panic!("invalid direction"),
                }
            })
            .collect()),
    )
}
