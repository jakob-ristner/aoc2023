use nom::branch::alt;
use nom::character::complete::u32;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space0},
    multi::{many1, separated_list1},
    sequence, IResult,
};

use State::*;

fn main() {
    println!("Hello, world!");
    let (_, data) = parse(include_str!("input.txt")).unwrap();
    let ans: usize = data.iter().map(|(states, nums)| solve(states, nums)).sum();
    println!("{}", ans);
}

fn parse(content: &str) -> IResult<&str, Vec<(Vec<State>, Vec<usize>)>> {
    separated_list1(line_ending, parse_line)(content)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<State>, Vec<usize>)> {
    let (input, sequence) = many1(alt((operational, damaged, unknown)))(input)?;
    let (input, _) = space0(input)?;
    let (input, numbers) = separated_list1(tag(","), usize)(input)?;
    Ok((input, (sequence, numbers)))
}

fn usize(input: &str) -> IResult<&str, usize> {
    let (input, num) = u32(input)?;
    Ok((input, num as usize))
}

fn operational(input: &str) -> IResult<&str, State> {
    let (input, _) = many1(tag("."))(input)?;
    Ok((input, State::Operational))
}

fn damaged(input: &str) -> IResult<&str, State> {
    let (input, _) = tag("#")(input)?;
    Ok((input, State::Damaged))
}

fn unknown(input: &str) -> IResult<&str, State> {
    let (input, _) = tag("?")(input)?;
    Ok((input, State::Unknown))
}

fn new_dp_array(nums_l: usize, seq_l: usize) -> Vec<Vec<usize>> {
    let inner = vec![0; seq_l + 1];
    let mut outer = vec![inner; nums_l + 1];
    outer[0][seq_l] = 1;
    outer
}

fn pp_mat(mat: &Vec<Vec<usize>>) {
    for row in mat {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

fn can_match(states: &[State], num: usize) -> bool {
    if (states.len()) < num
        || states[..num].contains(&Operational)
        || states.get(num) == Some(&Damaged)
    {
        return false;
    }
    return true;
}

fn solve(states: &Vec<State>, nums: &Vec<usize>) -> usize {
    let mut states = states.clone();
    states = vec![states.clone(), states.clone(), states.clone(), states.clone(), states.clone()].join(&Unknown);
    let mut nums = nums.repeat(5);
    let mut dp = new_dp_array(nums.len(), states.len());
    for col in (0..states.len()).rev() {
        if states[col] == Damaged {
            break;
        } else {
            dp[0][col] = 1;
        }
    }
    let mut row = 1;
    for num in nums.iter().rev() {
        for col in (0..states.len()).rev() {
            match states[col] {
                Operational => {
                    dp[row][col] = dp[row][col + 1];
                }
                Damaged => {
                    if can_match(&states[col..], *num) {
                        let hash = dp[row - 1].get(col + num + 1).cloned();
                        dp[row][col] = dp[row - 1].get(col + num + 1).unwrap_or(&0).clone();
                        if row == 1 && hash.is_none() {
                            dp[row][col] = 1;
                        }
                    }
                }
                Unknown => {
                    if can_match(&states[col..], *num) {
                        let hash = dp[row - 1].get(col + num + 1).cloned();
                        dp[row][col] = dp[row - 1].get(col + num + 1).unwrap_or(&0).clone();
                        if row == 1 && hash.is_none() {
                            dp[row][col] = 1;
                        }
                    }
                    dp[row][col] += dp[row][col + 1];
                }
            }
        }
        row += 1;
    }
    return dp[nums.len()][0];
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Operational,
    Damaged,
    Unknown,
}
