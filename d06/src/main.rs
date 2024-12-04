use std::iter::zip;

use regex::Regex;

fn main() {
    let x = parse();
    dbg!(part_1(x));
}

fn part_1(races: Vec<(usize, usize)>) -> usize {
    races
        .iter()
        .map(|&(dur, rec)| {
            (0..=dur)
                .into_iter()
                .map(|num| num * (dur - num))
                .filter(|num| num > &rec)
                .count()
        })
        .product()
}

fn parse() -> Vec<(usize, usize)> {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    let re = Regex::new(r"\d+").unwrap();

    let first: Vec<usize> = re
        .find_iter(lines[0])
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    let snd: Vec<usize> = re
        .find_iter(lines[1])
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    zip(first, snd).collect()
}
