use std::ops::Range;

use regex::Regex;

type Maps = Vec<Map>;
type Seeds = Vec<i64>;

type Map = Vec<(Range<i64>, i64)>;

fn find_source(dest: i64, maps: &Vec<Map>) {
    let mut current = dest.clone();
}

fn find_destination(seed: i64, maps: &Vec<Map>) -> i64 {
    let mut current = seed.clone();
    for map in maps {
        for (source_range, offset) in map {
            if source_range.contains(&current) {
                current += offset;
                break;
            }
        }
    }
    current
}

fn main() {
    let (seeds, maps) = parse();
    println!("parsing done");

    let sol = seeds.iter().map(|s| find_destination(*s, &maps)).min();
    dbg!(sol);
}

fn parse() -> (Seeds, Maps) {
    let parts: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let seeds = seeds(parts[0]);
    let maps = maps(&parts[1..]);
    (seeds, maps)
}

fn maps(lines: &[&str]) -> Maps {
    let re = Regex::new(r"\d+").unwrap();
    let mut maps: Maps = Vec::new();
    for line in lines {
        let mut ranges: Map = Vec::new();
        let tail: String = line
            .chars()
            .skip_while(|c| !c.is_digit(10))
            .collect::<String>();
        for range in tail.lines() {
            let nums: Vec<i64> = re
                .find_iter(range)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect();

            let dest = nums[0];
            let src = nums[1];
            let dist = nums[2];

            let offset = dest - src;
            ranges.push(((src..src + dist), offset));
        }
        maps.push(ranges);
    }
    maps
}

fn seeds(line: &str) -> Seeds {
    let re = Regex::new(r"\d+").unwrap();
    let mut seeds = Vec::new();
    let matches: Vec<i64> = re
        .find_iter(line)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect();

    for i in 1..matches.len() {
        if i % 2 == 1 {
            let mut k: Vec<i64> = (matches[i - 1]..(matches[i - 1] + matches[i])).collect();
            seeds.append(&mut k);
        }
    }

    seeds
}

// fn seeds(line: &str) -> Seeds {
//     let re = Regex::new(r"\d+").unwrap();
//     re.find_iter(line)
//         .map(|m| m.as_str().parse().unwrap())
//         .collect()
// }
