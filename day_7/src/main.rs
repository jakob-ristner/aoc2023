use std::{cmp::Ordering, collections::HashMap, iter::zip};

type Hand = (Vec<u32>, u32, u32);

fn main() {
    let mut hands1 = parse_1();
    let sol1 = part_1(&mut hands1);
    let mut hands2 = parse_2();
    let sol2 = part_1(&mut hands2);
    println!("{}", sol1);
    println!("{}", sol2);
}

fn part_1(hands: &mut Vec<Hand>) -> u32 {
    hands.sort_by(|a, b| cmp_hand(a, b));
    let sol1 = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, (_, _, bid))| {
            acc + (index as u32 + 1) * bid
        });

    sol1
}

fn cmp_hand((cards_1, score_1, _): &Hand, (cards_2, score_2, _): &Hand) -> Ordering {
    match score_1.cmp(&score_2) {
        Ordering::Equal => {
            for (c1, c2) in zip(cards_1, cards_2) {
                match c1.cmp(&c2) {
                    Ordering::Equal => continue,
                    ord => return ord,
                };
            }
            return Ordering::Equal;
        }
        ord => ord,
    }
}

fn get_card_score_part2(ch: char) -> u32 {
    match ch {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 0,
        'T' => 10,
        c => c.to_digit(10).expect("not a card"),
    }
}

fn get_card_score(ch: char) -> u32 {
    match ch {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        c => c.to_digit(10).expect("not a card"),
    }
}

fn get_occurences(cards: &Vec<u32>) -> Vec<u32> {
    let mut visited: HashMap<u32, u32> = HashMap::new();
    for &card in cards {
        *visited.entry(card).or_insert(0) += 1;
    }
    let mut occ: Vec<u32> = visited.into_values().collect();
    occ.resize(5, 0);
    occ.sort();
    occ
}

fn get_hand_score_part_2(cards: &Vec<u32>) -> u32 {
    let mut out: Vec<u32> = Vec::new();
    for i in 1..=14 {
        let mut test = cards.clone();
        test.iter_mut().for_each(|x| {
            if *x == 0 {
                *x = i
            }
        });
        out.push(get_hand_score(&test));
    }
    *out.iter().max().unwrap()
}

fn get_hand_score(cards: &Vec<u32>) -> u32 {
    let occ = get_occurences(cards);
    match &occ[0..5] {
        [_, _, _, _, 5] => 6, // Five of a kind
        [_, _, _, _, 4] => 5, // Four of a kind
        [_, _, _, 2, 3] => 4, // Full house (kåk)
        [_, _, _, _, 3] => 3, // Three of a kind
        [_, _, _, 2, 2] => 2, // Two pair
        [_, _, _, _, 2] => 1, // Pair
        _ => 0,               // High card
    }
}

fn parse_2() -> Vec<Hand> {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (cards_raw, bid_raw) = line.split_once(" ").unwrap();
            let cards: Vec<u32> = cards_raw.chars().map(get_card_score_part2).collect();
            let bid: u32 = bid_raw.parse().unwrap();
            let score = get_hand_score_part_2(&cards);

            (cards, score, bid)
        })
        .collect()
}

fn parse_1() -> Vec<Hand> {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (cards_raw, bid_raw) = line.split_once(" ").unwrap();
            let cards: Vec<u32> = cards_raw.chars().map(get_card_score).collect();
            let bid: u32 = bid_raw.parse().unwrap();
            let score = get_hand_score(&cards);

            (cards, score, bid)
        })
        .collect()
}
