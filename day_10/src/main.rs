use std::collections::HashMap;

use part1::*;

mod part1;

fn main() {
    let (map, start) = parse();
    let path = find_loop(&map, &start);
    let sol1 = part_1(&path);
    println!("{}", sol1);
}

fn find_loop(map: &Map, start: &Pos) -> Vec<Pos> {
    let mut prev = *start;
    let mut curr = get_next_initial(map, start);
    let mut path: Vec<Pos> = vec![*start, curr];

    while &curr != start {
        let next = get_next(map, &curr, &prev);
        path.push(next);
        prev = curr;
        curr = next;
    }

    path
}

fn parse() -> (Map, (i32, i32)) {
    let lines = include_str!("../input.txt").lines();

    let mut chars = HashMap::new();
    let mut start = (0, 0);

    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            chars.insert((row as i32, col as i32), ch);
            if ch == 'S' {
                start = (row as i32, col as i32);
            }
        }
    }

    (chars, start)
}
