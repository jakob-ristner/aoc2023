use itertools::Itertools;

type Pos = (i128, i128);

fn main() {
    let matrix = parse();
    let galxs = galaxies(&matrix);
    let pairs = galxs.into_iter().combinations(2);
    let exp = expanded(&matrix);
    let sol1: i128 = pairs
        .clone()
        .fold(0, |acc, pair| acc + dist(pair[0], pair[1], &exp, 2));
    let sol2: i128 = pairs
        .clone()
        .fold(0, |acc, pair| acc + dist(pair[0], pair[1], &exp, 1000000));
    println!("part 1: {}", sol1);
    println!("part 2: {}", sol2);
}

fn between(num: i128, n1: i128, n2: i128) -> bool {
    (num > n1 && num < n2) || (num < n1 && num > n2)
}

fn dist(
    (frow, fcol): Pos,
    (trow, tcol): Pos,
    (r_exp, c_exp): &(Vec<i128>, Vec<i128>),
    factor: i128,
) -> i128 {
    r_exp
        .iter()
        .filter(|&&row| between(row, trow, frow))
        .count() as i128
        * (factor - 1)
        + c_exp
            .iter()
            .filter(|&&col| between(col, tcol, fcol))
            .count() as i128
            * (factor - 1)
        + (frow - trow).abs()
        + (fcol - tcol).abs()
}

fn expanded(mat: &Vec<Vec<char>>) -> (Vec<i128>, Vec<i128>) {
    let mut rows = Vec::new();
    let mut cols = Vec::new();

    for (row, line) in mat.iter().enumerate() {
        if line.iter().all(|&c| c == '.') {
            rows.push(row as i128)
        }
    }

    for (col, line) in transpose(mat.clone()).iter().enumerate() {
        if line.iter().all(|&c| c == '.') {
            cols.push(col as i128)
        }
    }

    (rows, cols)
}

fn parse() -> Vec<Vec<char>> {
    let matrix: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    matrix
}

fn galaxies(mat: &Vec<Vec<char>>) -> Vec<Pos> {
    let mut out = Vec::new();
    for r in 0..mat.len() {
        for c in 0..mat[0].len() {
            if mat[r][c] == '#' {
                out.push((r as i128, c as i128));
            }
        }
    }
    out
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
