fn main() {
    let nums = parse();
    let sol1 = part_1(&nums);
    let sol2 = part_2(&nums);
    println!("part 1: {}", sol1);
    println!("part 2: {}", sol2);
}

fn parse() -> Vec<Vec<i64>> {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_1(nums: &Vec<Vec<i64>>) -> i64 {
    nums.iter().fold(0, |acc, curr| acc + extrapolate(curr))
}
fn part_2(nums: &Vec<Vec<i64>>) -> i64 {
    nums.iter()
        .fold(0, |acc, curr| acc + extrapolate_backwards(curr))
}

fn extrapolate_backwards(nums: &Vec<i64>) -> i64 {
    let mut curr = nums.clone();
    let mut accum = 0;
    let mut i = 0;

    while !curr.iter().all(|c| c == &0) {
        accum += curr[0] * if i % 2 == 0 { 1 } else { -1 };
        i += 1;
        let next: Vec<i64> = curr.windows(2).map(|slc| slc[1] - slc[0]).collect();
        curr = next.clone();
    }
    accum
}

fn extrapolate(nums: &Vec<i64>) -> i64 {
    let mut curr = nums.clone();
    let mut accum = 0;

    while !curr.iter().all(|c| c == &0) {
        accum += curr.last().unwrap();
        let next: Vec<i64> = curr.windows(2).map(|slc| slc[1] - slc[0]).collect();
        curr = next.clone();
    }
    accum
}
