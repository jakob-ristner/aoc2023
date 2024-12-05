use core::panic;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let content = include_str!("input.txt");
    let (_, map) = map(content).unwrap();
    //let ans = sim_beam(&map);
    let mut max = 0;
    for (pos, dir) in all_starts(&map) {
        let mut visited = HashMap::new();
        beam(&map, &mut visited, pos, dir);
        max = max.max(visited.keys().len());
    }
    println!("{}", max);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}


type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Obj {
    VertSplitter,
    HorzSplitter,
    PosMirror,
    NegMirror,
    Empty,
    Padding,
}

fn object(input: &str) -> IResult<&str, Obj> {
    use Obj::*;
    alt((
        value(VertSplitter, tag("|")),
        value(HorzSplitter, tag("-")),
        value(PosMirror, tag("/")),
        value(NegMirror, tag("\\")),
        value(Empty, tag(".")),
    ))(input)
}

fn pad_map(map: Vec<Vec<Obj>>) -> Vec<Vec<Obj>> {
    let mut padded_map = vec![vec![Obj::Padding; map[0].len() + 2]; map.len() + 2];
    for (i, row) in map.iter().enumerate() {
        for (j, obj) in row.iter().enumerate() {
            padded_map[i + 1][j + 1] = *obj;
        }
    }
    padded_map
}

fn all_starts(map: &Vec<Vec<Obj>>) -> Vec<(Pos, Dir)> {

    let mut starts = vec![];
    for i in 1..map.len() - 1 {
        starts.push(((1, i), Dir::Right));
        starts.push(((1, i), Dir::Left));
    }
    for i in 1..map[0].len() - 1 {
        starts.push(((i, 1), Dir::Down));
        starts.push(((i, 1), Dir::Up));
    }
    starts
}

fn sim_beam(map: &Vec<Vec<Obj>>) -> usize {
    let mut visited = HashMap::new();
    beam(map, &mut visited, (1, 1), Dir::Right);
    visited.keys().len()
}

fn beam(map: &Vec<Vec<Obj>>, visited: &mut HashMap<Pos, Vec<Dir>>, pos: Pos, dir: Dir) {
    let (x, y) = pos;

    if visited.get(&pos).map(|d| d.contains(&dir)).unwrap_or(false) || map[y][x] == Obj::Padding {
        return;
    }
    visited.entry(pos).or_default().push(dir);
    match map[y][x] {
        Obj::VertSplitter => match dir {
            Dir::Left | Dir::Right => {
                beam(map, visited, (x, y - 1), Dir::Up);
                beam(map, visited, (x, y + 1), Dir::Down);
            }
            Dir::Up | Dir::Down => beam(map, visited, next_pos(pos, dir), dir),
        },
        Obj::HorzSplitter => match dir {
            Dir::Up | Dir::Down => {
                beam(map, visited, (x - 1, y), Dir::Left);
                beam(map, visited, (x + 1, y), Dir::Right);
            }
            Dir::Left | Dir::Right => beam(map, visited, next_pos(pos, dir), dir),
        },
        Obj::PosMirror => {
            let next_dir = flip_pos_mirror(dir);
            let next_pos = next_pos(pos, next_dir);
            beam(map, visited, next_pos, next_dir);
        }
        Obj::NegMirror => {
            let next_dir = flip_neg_mirror(dir);
            let next_pos = next_pos(pos, next_dir);
            beam(map, visited, next_pos, next_dir);
        }
        Obj::Empty => {
            beam(map, visited, next_pos(pos, dir), dir);
        }
        Obj::Padding => unreachable!(),
    }
}

fn flip_neg_mirror(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Left,
        Dir::Down => Dir::Right, // not in test input
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
    }
}

fn flip_pos_mirror(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left, // not in test input
        Dir::Left => Dir::Down, // not in test input
        Dir::Right => Dir::Up,
    }
}

fn next_pos((x, y): Pos, dir: Dir) -> Pos {
    match dir {
        Dir::Up => (x, y - 1),
        Dir::Down => (x, y + 1),
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
    }
}

fn map(input: &str) -> IResult<&str, Vec<Vec<Obj>>> {
    let (input, map) = separated_list1(line_ending, many1(object))(input)?;
    Ok((input, pad_map(map)))
}
