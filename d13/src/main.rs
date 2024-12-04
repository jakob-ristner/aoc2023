use std::{cmp, collections::VecDeque, isize};

fn main() {
    let contents = include_str!("./input.txt");
    let split: Vec<&str> = contents.split("\n\n").collect();
    let ans: isize = split
        .iter()
        .enumerate()
        .map(|(i, content)| {
            let (o1, o2) = parse(content);
            let x = (solve(&o1).unwrap_or(0) * 100 + solve(&o2).unwrap_or(0));
            return x;
        })
        .sum();
    dbg!(ans);
}

fn parse(contents: &str) -> (Vec<Vec<isize>>, Vec<Vec<isize>>) {
    let lines = contents.lines();
    let mut out = Vec::new();
    for line in lines.clone() {
        let mut num = vec![0; line.len()];
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => num[i] = 1,
                '.' => (),
                _ => panic!("Invalid character at line {}:{}", line, i),
            }
        }
        out.push(num);
    }
    let t: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut out2 = Vec::new();
    for i in 0..t[0].len() {
        let mut num = vec![0; t.len()];
        for j in 0..t.len() {
            match t[j][i] {
                '#' => num[j] = 1,
                '.' => (),
                _ => panic!("Invalid character"),
            }
        }
        out2.push(num);
    }
    (out, out2)
}

fn solve(list: &Vec<Vec<isize>>) -> Option<isize> {
    for i in 0..list.len() {
        let x = &list[i..];
        let y = &list[..i];
        let min = cmp::min(x.len(), y.len());
        let y_p: Vec<&Vec<isize>> = y.iter().rev().take(min).collect();
        let x_p: Vec<&Vec<isize>> = x.iter().take(min).collect();
        if min > 0 && p2eq(&x_p, &y_p) {
            p2eq(&x_p, &y_p);
            return Some(i as isize);
        }
    }
    return None;
}

fn pp_mat(mat: &Vec<Vec<isize>>) {
    for i in mat {
        for x in i {
            print!("{} ", x)
        }
        println!();
    }
}

fn p1eq(x_p: &Vec<&Vec<isize>>, y_p: &Vec<&Vec<isize>>) -> bool {
    y_p == x_p
}

fn p2eq(x_p: &Vec<&Vec<isize>>, y_p: &Vec<&Vec<isize>>) -> bool {
    let mut out: Vec<Vec<isize>> = vec![vec![0; x_p[0].len()]; x_p.len()];
    let mut ctr = 0;
    for i in 0..y_p.len() {
        for j in 0..y_p[i].len() {
            let res = x_p[i][j] - y_p[i][j];
            out[i][j] = res;
            if res == 1 || res == -1 {
                ctr += 1;
            }
        }
    }
    if ctr == 1 {
        pp_mat(&out);
        return true;
    }
    false
}
