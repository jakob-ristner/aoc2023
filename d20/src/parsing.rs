
use nom::{branch::alt, bytes::complete::tag, character::complete::{alpha1, line_ending, space1}, combinator::eof, multi::{fold_many1, separated_list1}, sequence::tuple, IResult, Parser};

use crate::{init_conjunctions, Circuit, Module};

pub fn circuit(input: &str) -> IResult<&str, Circuit> {
    let (input, circ) = fold_many1(|input| {
        let (input, module) = alt((
            broadcaster,
            flipflop,
            conjunction,
        ))(input)?;
        let (input, _) = alt((
            line_ending,
            eof
        ))(input)?;
        Ok((input, module))
    }, Circuit::new, |mut acc, (name, module)| {
        acc.insert(name, module);
        acc
    })(input)?;
    Ok((input, init_conjunctions(circ)))
}

fn outputs(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tuple((tag(","), space1)), alpha1.map(|s: &str| s.to_string()))(input)
}

fn broadcaster(input: &str) -> IResult<&str, (String, Module)> {
    let (input, _) = tag("broadcaster -> ")(input)?;
    let (input, outs) = outputs(input)?;
    Ok((input, ("broadcaster".into(), Module::Broadcaster(outs))))
}

fn flipflop(input: &str) -> IResult<&str, (String, Module)> {
    let (input, _) = tag("%")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outs) = outputs(input)?;
    let flipflop = Module::new_flipflop(outs);
    Ok((input, (name.into(), flipflop)))
}

fn conjunction(input: &str) -> IResult<&str, (String, Module)> {
    let (input, _) = tag("&")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outs) = outputs(input)?;
    let conjunction = Module::new_conjunction(outs);
    Ok((input, (name.into(), conjunction)))
}
