    use std::collections::HashMap;

    use nom::bytes::complete::{tag, take_until};
    use nom::character::complete::{line_ending, u64};
    use nom::multi::{fold_many1, separated_list1};
    use nom::sequence::{preceded, tuple};
    use nom::Parser;
    use nom::{
        branch::alt, combinator::value, IResult,
    };

    use crate::defs::*;

    pub fn parse(input: &str) -> IResult<&str, (HashMap<String, Rule>, Vec<Obj>)> {
        let (input, rules) = fold_many1(
            |input| {
                let (input, rule) = rule(input)?;
                let (input, _) = line_ending(input)?;
                Ok((input, rule))
            },
            HashMap::new,
            |mut acc, item| {
                acc.insert(item.name.clone(), item);
                acc
            },
        )(input)?;
        let (input, _) = line_ending(input)?;
        let (input, objs) = separated_list1(line_ending, obj)(input)?;
        Ok((input, (rules, objs)))
    }

    //{x=1515,m=927,a=2182,s=1121}
    fn obj(input: &str) -> IResult<&str, Obj> {
        let (input, _) = tag("{")(input)?;
        let (input, x) = preceded(tag("x="), u64)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, m) = preceded(tag("m="), u64)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, a) = preceded(tag("a="), u64)(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, s) = preceded(tag("s="), u64)(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((input, Obj { x, m, a, s }))
    }

    //zrz{x>3467:tjz,a<3096:hmq,bnj}
    fn rule(input: &str) -> IResult<&str, Rule> {
        let (input, name) = take_until("{")(input)?;
        let (input, _) = tag("{")(input)?;
        let (input, ops) = separated_list1(
            tag(","),
            tuple((
                alt((
                    value(Val::X, tag("x")),
                    value(Val::M, tag("m")),
                    value(Val::A, tag("a")),
                    value(Val::S, tag("s")),
                )),
                alt((value(Op::Lt, tag("<")), value(Op::Gt, tag(">")))),
                u64,
                preceded(tag(":"), take_until(",").map(|s: &str| s.to_string())),
            )),
        )(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, ow) = take_until("}")(input)?;
        let (input, _) = tag("}")(input)?;
        Ok((
            input,
            Rule {
                name: name.to_string(),
                ops,
                ow: ow.to_string(),
            },
        ))
    }
