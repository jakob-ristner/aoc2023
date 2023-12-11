use std::collections::{HashMap, HashSet};
mod part1;
use part1::*;

static TO_WEST: [char; 3] = ['-', 'F', 'L'];
static TO_EAST: [char; 3] = ['-', '7', 'J'];
static TO_NORTH: [char; 3] = ['|', 'F', '7'];
static TO_SOUTH: [char; 3] = ['|', 'J', 'L'];
type Pos = (i32, i32);
type Map = HashMap<Pos, char>;

#[derive(Debug, PartialEq, Eq)]
enum State {
    Inside,
    Loop,
    Outside,
}

fn pos_state(&(row, col): &Pos, map: &Map, path: &Vec<Pos>) -> State {
    use State::*;
    let mut accum = 0;
    let path: HashSet<Pos> = HashSet::from_iter(path.clone().into_iter());
    let mut prev = map.get(&(row, col)).unwrap();
    let (mut cr, cc) = (row + 1, col);
    if path.contains(&(row, col)) {
        return Loop;
    }

    while let Some(ch) = map.get(&(cr, cc)) {
        if path.contains(&(cr, cc)) && ch != &'|' {
            if (ch == &'L' && prev == &'F') || (ch == &'J' && prev == &'7') {
                accum -= 1;
            } else if !((ch == &'L' && prev == &'7') || (ch == &'J' && prev == &'F')) {
                accum += 1;
            }
            prev = ch;
        }
        cr = cr + 1;
    }

    return if accum % 2 == 0 { Outside } else { Inside };
}

fn main() {
    let (map, start) = parse();
    let path = find_loop(&map, &start);

    println!("part 1: {}", part_1(&path));

    let inside: Vec<_> = map
        .keys()
        .filter(|pos| pos_state(pos, &map, &path) == State::Inside)
        .collect();

    println!("part 2: {}", inside.len());
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
