mod defs;
mod parser;
use std::collections::HashMap;

use crate::defs::*;
use crate::parser::*;

fn main() {
    let content = include_str!("input.txt");
    let (_, (rules, objs)) = parse(content).unwrap();
    let acc = objs
        .iter()
        .filter(|obj| apply_seq(obj, &rules))
        .map(|obj| obj.sum())
        .sum::<u64>();

    let combs = combinations_accepted("in", &rules, 1, 4000, 1, 4000, 1, 4000, 1, 4000);

    println!("Part 1: {}", acc);
    println!("Part 2: {}", combs);
}

pub fn apply_seq(obj: &Obj, rules: &HashMap<String, Rule>) -> bool {
    let name = "in";
    let mut rule = rules.get(name).unwrap();
    loop {
        let next = rule.apply(obj);
        if next == "A" {
            return true;
        } else if next == "R" {
            return false;
        }
        rule = rules.get(&next).unwrap();
    }
}

fn combinations_accepted(
    name: &str,
    all_rules: &HashMap<String, Rule>,
    min_x: u64,
    max_x: u64,
    min_m: u64,
    max_m: u64,
    min_a: u64,
    max_a: u64,
    min_s: u64,
    max_s: u64,
) -> u64 {
    if name == "R" {
        return 0;
    }
    if name == "A" {
        return (max_x - min_x + 1)
            * (max_m - min_m + 1)
            * (max_a - min_a + 1)
            * (max_s - min_s + 1);
    }

    let rule = all_rules.get(name).unwrap();
    let mut acc = 0;
    let mut min_x = min_x;
    let mut max_x = max_x;
    let mut min_m = min_m;
    let mut max_m = max_m;
    let mut min_a = min_a;
    let mut max_a = max_a;
    let mut min_s = min_s;
    let mut max_s = max_s;

    for (val, op, num, dest) in &rule.ops {
        match op {
            Op::Lt => match val {
                Val::X => {
                    if min_x >= *num {
                        continue;
                    } else {
                        acc += combinations_accepted(
                            dest,
                            all_rules,
                            min_x,
                            *num - 1,
                            min_m,
                            max_m,
                            min_a,
                            max_a,
                            min_s,
                            max_s,
                        );
                        min_x = *num;
                    }
                }
                Val::M => {
                    if min_m >= *num {
                        continue;
                    } else {
                        acc += combinations_accepted(
                            dest,
                            all_rules,
                            min_x,
                            max_x,
                            min_m,
                            *num - 1,
                            min_a,
                            max_a,
                            min_s,
                            max_s,
                        );
                        min_m = *num;
                    }
                }
                Val::A => {
                    if min_a >= *num {
                        continue;
                    } else {
                        acc += combinations_accepted(
                            dest,
                            all_rules,
                            min_x,
                            max_x,
                            min_m,
                            max_m,
                            min_a,
                            *num - 1,
                            min_s,
                            max_s,
                        );
                        min_a = *num;
                    }
                }
                Val::S => {
                    if min_s >= *num {
                        continue;
                    } else {
                        acc += combinations_accepted(
                            dest,
                            all_rules,
                            min_x,
                            max_x,
                            min_m,
                            max_m,
                            min_a,
                            max_a,
                            min_s,
                            *num - 1,
                        );
                        min_s = *num;
                    }
                }
            },
            Op::Gt => match val {
                Val::X => {
                    if max_x <= *num {
                        continue;
                    } else {
                        acc += combinations_accepted(
                            dest,
                            all_rules,
                            *num + 1,
                            max_x,
                            min_m,
                            max_m,
                            min_a,
                            max_a,
                            min_s,
                            max_s,
                        );
                        max_x = *num;
                    }
                }
                Val::M => {
                    if max_m <= *num {
                        continue;
                    } else {
                        acc += combinations_accepted(
                            dest,
                            all_rules,
                            min_x,
                            max_x,
                            *num + 1,
                            max_m,
                            min_a,
                            max_a,
                            min_s,
                            max_s,
                        );
                        max_m = *num;
                    }
                }
                Val::A => {
                    if max_a <= *num {
                        continue;
                    } else {
                        acc += combinations_accepted(
                            dest,
                            all_rules,
                            min_x,
                            max_x,
                            min_m,
                            max_m,
                            *num + 1,
                            max_a,
                            min_s,
                            max_s,
                        );
                        max_a = *num;
                    }
                }
                Val::S => {
                    if max_s <= *num {
                        continue;
                    } else {
                        acc += combinations_accepted(
                            dest,
                            all_rules,
                            min_x,
                            max_x,
                            min_m,
                            max_m,
                            min_a,
                            max_a,
                            *num + 1,
                            max_s,
                        );
                        max_s = *num;
                    }
                }
            },
        }
    }

    acc + combinations_accepted(
        &rule.ow, all_rules, min_x, max_x, min_m, max_m, min_a, max_a, min_s, max_s,
    )
}
