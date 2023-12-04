use regex::Regex;
use std::collections::HashMap;

fn main() {
    let matrix = parse();

    let sol1 = part_1(&matrix);
    let sol2 = part_2(&matrix);

    println!("part 1: {}", sol1);
    println!("part 2: {}", sol2);
}

type NumberPos = (usize, (usize, usize), u32);

fn part_2(matrix: &Vec<Vec<char>>) -> u32 {
    let mut gear_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let row_start_end = find_numbers(&matrix);

    for number_pos in &row_start_end {
        if let Some((pos, num)) = gear(matrix, number_pos) {
            let arr = gear_numbers.entry(pos).or_insert(vec![]);
            arr.push(num);
        }
    }

    gear_numbers.values().fold(0, |accum, list| {
        if list.len() == 2 {
            accum + list.iter().product::<u32>()
        } else {
            accum
        }
    })
}

fn part_1(matrix: &Vec<Vec<char>>) -> u32 {
    let row_start_end = find_numbers(&matrix);
    let parts: Vec<&NumberPos> = row_start_end
        .iter()
        .filter(|x| is_part(&matrix, x))
        .collect();

    parts.iter().fold(0, |accum, (_, (_, _), num)| accum + num)
}

fn parse() -> Vec<Vec<char>> {
    let lines = include_str!("../input.txt").lines();
    let matrix: Vec<Vec<char>> = lines.map(|x| x.chars().collect::<Vec<char>>()).collect();
    matrix
}

fn adjacent(row: usize, col: usize) -> Vec<(Option<usize>, Option<usize>)> {
    vec![
        (row.checked_sub(1), Some(col)),          // left
        (row.checked_sub(1), col.checked_sub(1)), // top left
        (Some(row), col.checked_sub(1)),          // top
        (Some(row + 1), col.checked_sub(1)),      // top right
        (Some(row + 1), Some(col)),               // right
        (Some(row + 1), Some(col + 1)),           // bottom right
        (Some(row), Some(col + 1)),               // bottom
        (row.checked_sub(1), Some(col + 1)),      // bottom left
    ]
}

fn gear(
    matrix: &Vec<Vec<char>>,
    (row, (start, end), number): &NumberPos,
) -> Option<((usize, usize), u32)> {
    for col in *start..*end {
        for (orx, ocx) in adjacent(*row, col) {
            if let Some(ch) = try_get_char(matrix, (orx, ocx)) {
                if ch == '*' {
                    return Some(((orx.unwrap(), ocx.unwrap()), *number));
                }
            }
        }
    }
    None
}

fn try_get_char(
    mat: &Vec<Vec<char>>,
    (ocol, orow): (Option<usize>, Option<usize>),
) -> Option<char> {
    let (cx, rx) = (orow?, ocol?);
    let adj_row = mat.get(rx)?;
    let ch = adj_row.get(cx)?;
    return Some(*ch);
}

fn is_part(matrix: &Vec<Vec<char>>, (row, (start, end), _): &NumberPos) -> bool {
    for col in *start..*end {
        for pos in adjacent(*row, col) {
            if let Some(ch) = try_get_char(matrix, pos) {
                if !ch.is_digit(10) && ch != '.' {
                    return true;
                }
            }
        }
    }
    false
}

fn find_numbers(matrix: &Vec<Vec<char>>) -> Vec<NumberPos> {
    let re = Regex::new(r"\d+").expect("faulty regex");
    let mut row_start_end: Vec<NumberPos> = Vec::new();
    for (i, line) in matrix.iter().enumerate() {
        let text: String = line.iter().collect();
        for m in re.find_iter(&text) {
            row_start_end.push((
                i,
                (m.start(), m.end()),
                m.as_str().parse().expect("could not parse number"),
            ));
        }
    }
    row_start_end
}
