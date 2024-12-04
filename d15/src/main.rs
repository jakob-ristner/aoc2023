use std::time::Instant;

use nom::character::complete::u32;
use nom::{
    branch::alt,
    character::complete::{alpha1, char},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let p1 = p1(input);
    let time = Instant::now();
    let p2 = p2(input);
    println!("Time: {}µs", time.elapsed().as_micros());
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn p2(input: &str) -> u32 {
    let (_, ops) = ops(input).unwrap();
    let mut map: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    for op in ops {
        perform_op(&mut map, &op);
    }
    focus(&map)
}

fn p1(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[derive(Debug)]
enum Op {
    Insert(String, u32),
    Remove(String),
}

fn perform_op(map: &mut [Vec<(String, u32)>], op: &Op) {
    match op {
        Op::Insert(key, value) => {
            let hash = hash(key);
            for i in 0..map[hash as usize].len() {
                if &map[hash as usize][i].0 == key {
                    map[hash as usize][i].1 = *value;
                    return;
                }
            }
            map[hash as usize].push((key.clone(), *value));
        }
        Op::Remove(key) => {
            let hash = hash(key);
            for i in 0..map[hash as usize].len() {
                if &map[hash as usize][i].0 == key {
                    map[hash as usize].remove(i);
                    break;
                }
            }
        }
    }
}

fn insert(input: &str) -> IResult<&str, Op> {
    let (input, key) = alpha1(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = u32(input)?;
    Ok((input, Op::Insert(key.into(), value)))
}

fn remove(input: &str) -> IResult<&str, Op> {
    let (input, key) = alpha1(input)?;
    let (input, _) = char('-')(input)?;
    Ok((input, Op::Remove(key.into())))
}

fn ops(input: &str) -> IResult<&str, Vec<Op>> {
    let (input, list) = separated_list1(char(','), alt((remove, insert)))(input)?;
    Ok((input, list))
}

fn focus(map: &[Vec<(String, u32)>]) -> u32 {
    let mut sum = 0;
    for (bx_num, bx) in map.iter().enumerate() {
        for (slot_num, (_, value)) in bx.iter().enumerate() {
            sum += (bx_num as u32 + 1) * (slot_num as u32 + 1) * value;
        }
    }
    sum
}
