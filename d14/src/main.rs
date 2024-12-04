use std::{collections::HashMap, hash::Hash, time::Instant};

use bimap::{BiHashMap, BiMap};

type Mat = Vec<Vec<char>>;

fn main() {
    let cont = include_str!("input.txt");
    let data = parse(cont);
    let p1 = p1(&data);
    let p2 = p2(&data);
    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn p2(data: &Mat) -> usize {
    let corr = cycles(data, 1000000000);
    load_p2(&corr)
}

fn cycles(data: &Mat, goal: usize) -> Mat {
    let mut map: BiHashMap<Mat, usize> = BiHashMap::new();

    let mut data = data.clone();
    for cycle in 0..goal {
        data = fall_rocks(&data);
        data = rotate(&data);
        data = fall_rocks(&data);
        data = rotate(&data);
        data = fall_rocks(&data);
        data = rotate(&data);
        data = fall_rocks(&data);
        data = rotate(&data);

        if let Some(cycle_start) = map.get_by_left(&data) {
            let cycle_len = cycle - cycle_start;
            let remaining = (goal - cycle) % cycle_len;
            let corr_cycle = cycle_start + remaining - 1;
            let corr_data = map.get_by_right(&corr_cycle).unwrap();
            return corr_data.clone();
        }
        map.insert(data.clone(), cycle);
    }
    data
}

fn load_p2(data: &Mat) -> usize {
    let mut load = 0;
    let size = data.len();

    for y in 0..size {
        for x in 0..size {
            if data[y][x] != 'O' {
                continue;
            }
            load += size - y;
        }
    }

    load
}

fn p1(data: &Mat) -> usize {
    let data = fall_rocks(data);
    load_p2(&data)
}

fn rotate<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut rotated = vec![vec![v[0][0].clone(); v.len()]; v[0].len()];
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            rotated[j][v.len() - 1 - i] = v[i][j].clone();
        }
    }
    rotated
}

fn fall_rocks(data: &Mat) -> Mat {
    let mut data = data.clone();
    let size = data.len();
    for y in 0..size {
        for x in 0..size {
            if data[y][x] != 'O' {
                continue;
            }

            let mut y_next = y;
            while y_next > 0 && data[y_next - 1][x] == '.' {
                y_next -= 1;
            }
            data[y][x] = '.';
            data[y_next][x] = 'O';
        }
    }
    data
}

fn parse(content: &str) -> Mat {
    content.lines().map(|line| line.chars().collect()).collect()
}
