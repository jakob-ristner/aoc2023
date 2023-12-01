static NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let part_1 = solve_part_1();
    let part_2 = solve_part_2();
    println!("part 1: {:?}", part_1);
    println!("part 2: {:?}", part_2);
}

fn solve_part_2() -> Option<u32> {
    let content = include_str!("../input.txt");
    content
        .lines()
        .map(|line| {
            let mut curr: String = line.to_owned();
            for (i, t) in NUMS.iter().enumerate() {
                curr = curr.replace(t, format!("{}{}{}", t, (i + 1).to_string(), t).as_str())
            }
            calc_line(curr)
        })
        .sum()
}

fn solve_part_1() -> Option<u32> {
    let content = include_str!("../input.txt");
    content.lines().map(|x| calc_line(x.to_owned())).sum()
}

fn calc_line(line: String) -> Option<u32> {
    let numbers: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
    let first = numbers[0].to_digit(10)?;
    let last = numbers.last()?.to_digit(10)?;
    Some(first * 10 + last)
}
