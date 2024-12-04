use regex::Regex;

fn main() {
    let cards = parse();
    let sol1 = part1(&cards);
    let sol2 = part2(&cards);
    println!("part 1: {}", sol1);
    println!("part 2: {}", sol2);
}

fn eval(pos: usize, cards: &Vec<usize>) -> usize {
    cards[pos]
        + (pos + 1..=pos + cards[pos])
            .map(|other_pos| eval(other_pos, cards))
            .sum::<usize>()
}

fn part2(cards: &[(Vec<usize>, Vec<usize>)]) -> usize {
    let wins: Vec<usize> = cards.iter().map(winners).collect();
    (0..wins.len()).fold(0, |accum, pos| accum + eval(pos, &wins)) + wins.len()
}

fn part1(cards: &[(Vec<usize>, Vec<usize>)]) -> usize {
    cards.iter().fold(0, |accum, card| accum + points(card))
}

fn points(card: &(Vec<usize>, Vec<usize>)) -> usize {
    let wins = winners(card);
    match wins {
        0 => 0,
        num => 2_usize.pow((num - 1).try_into().unwrap()),
    }
}

fn winners((winning, have): &(Vec<usize>, Vec<usize>)) -> usize {
    have.iter()
        .filter(|x| winning.contains(x))
        .fold(0, |acc, _| acc + 1)
}

fn parse() -> Vec<(Vec<usize>, Vec<usize>)> {
    let lines = include_str!("../input.txt").lines();
    let re = Regex::new(r"\d+").unwrap();
    lines
        .map(|line| {
            let (winning, have) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            (
                re.find_iter(winning)
                    .map(|num| num.as_str().parse::<usize>().unwrap())
                    .collect(),
                re.find_iter(have)
                    .map(|num| num.as_str().parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect()
}
