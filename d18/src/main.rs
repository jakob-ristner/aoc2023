use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::i128;
use nom::character::complete::u32;
use nom::character::complete::{anychar, char, space1};
use nom::combinator::value;
use nom::sequence::{delimited, tuple};
use nom::{character::complete::line_ending, multi::separated_list1, IResult};

//  r d l u
const DIRS: [(i128, i128); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type Op = (usize, i128);
type Pos = (i128, i128);

fn parse_p2(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(line_ending, |input| {
        let (input, _) = tuple((anychar, space1, u32, space1, tag("(#")))(input)?;
        let (input, hex_dist) = take(5_usize)(input)?;
        let (input, dir) = u32(input)?;
        let steps = i128::from_str_radix(hex_dist, 16).unwrap();
        let (input, _) = char(')')(input)?;
        Ok((input, (dir as usize, steps)))
    })(input)
}

fn parse_p1(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(line_ending, |input| {
        let (input, dir) = alt((
            value(0, char('R')),
            value(1, char('D')),
            value(2, char('L')),
            value(3, char('U')),
        ))(input)?;
        let (input, _) = space1(input)?;
        let (input, steps) = i128(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = delimited(tag("("), take(7_usize), tag(")"))(input)?;
        Ok((input, (dir, steps)))
    })(input)
}

fn main() {
    let content = include_str!("input.txt");
    let (_, ops2) = parse_p2(content).unwrap();
    let (_, ops1) = parse_p1(content).unwrap();
    let vert1 = build_points(&ops1);
    let vert2 = build_points(&ops2);
    let a1 = area(&vert1);
    let a2 = area(&vert2);
    println!("Part 1: {}", a1);
    println!("Part 2: {}", a2);
}

fn area(vertices: &[(i128, i128)]) -> usize {
    let area = all_inside(vertices);
    let boundary = boundary_points(vertices);
    (area - boundary / 2 + 1) + boundary
}

fn all_inside(vertices: &[(i128, i128)]) -> usize {
    let mut sum = 0;
    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % vertices.len()];
        sum += (x1 * y2) - (x2 * y1);
    }
    sum.unsigned_abs() as usize / 2
}

fn boundary_points(vertices: &[Pos]) -> usize {
    let mut sum = 0;
    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % vertices.len()];
        let dx = x2 - x1;
        let dy = y2 - y1;
        sum += dx.abs() + dy.abs();
    }
    sum as usize
}

fn build_points(ops: &Vec<Op>) -> Vec<Pos> {
    let mut vertices = Vec::new();
    let mut pos = (0, 0);
    for (dir, steps) in ops {
        let (dx, dy) = DIRS[*dir];
        pos = (pos.0 + dx * { *steps }, pos.1 + dy * { *steps });
        vertices.push(pos);
    }
    vertices
}