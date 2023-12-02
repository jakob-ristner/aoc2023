use anyhow::{Context, Result};
use regex::{Captures, Regex};

#[derive(Debug)]
struct Game {
    id: u32,
    green_max: u32,
    red_max: u32,
    blue_max: u32,
}

fn part_1(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter(|game| game.green_max <= 13 && game.blue_max <= 14 && game.red_max <= 12)
        .fold(0, |acc, g| acc + g.id)
}

fn part_2(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|g| g.red_max * g.blue_max * g.green_max)
        .sum()
}

fn main() {
    if let Ok(games) = parse() {
        let part_1 = part_1(&games);
        let part_2 = part_2(&games);

        println!("part 1: {:?}", part_1);
        println!("part 2: {}", part_2);
    }
}

fn parse() -> Result<Vec<Game>> {
    let content = include_str!("../input.txt");
    let re = Regex::new(r"Game (?<id>\d+)|(?<num>\d+) (?<color>\w+)").unwrap();

    content
        .lines()
        .map(|line| {
            let caps: Vec<Captures> = re.captures_iter(line).collect();
            let id: u32 = caps[0].name("id").context("no id")?.as_str().parse()?;
            let set = caps[1..]
                .iter()
                .map(|cap| {
                    let color = cap.name("color").context("no color")?.as_str();
                    let num = cap.name("num").context("no num")?.as_str().parse()?;
                    Ok((color, num))
                })
                .collect::<Result<Vec<(&str, u32)>>>()?;

            let red_max = col_max(&set, "red")?;
            let blue_max = col_max(&set, "blue")?;
            let green_max = col_max(&set, "green")?;

            Ok(Game {
                id,
                red_max,
                blue_max,
                green_max,
            })
        })
        .collect()
}

fn col_max(set: &Vec<(&str, u32)>, col: &str) -> Result<u32> {
    let (_, n) = set
        .iter()
        .filter(|(b, _)| b == &col)
        .max_by(|a, b| a.1.cmp(&b.1))
        .context("No max number")?;
    Ok(*n)
}
