advent_of_code::solution!(7);

use std::str::from_utf8;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::map,
    sequence::delimited,
};

fn outside_brackets(input: &str) -> IResult<&str, &str> {
    is_not("[]")(input)
}

fn inside_brackets(input: &str) -> IResult<&str, &str> {
    delimited(tag("["), is_not("]"), tag("]"))(input)
}

fn parse_sections(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let mut outside = Vec::new();
    let mut inside = Vec::new();

    let mut remaining = input;
    loop {
        if remaining.is_empty() {
            break;
        }
        match alt((
            map(outside_brackets, |s| {
                outside.push(s);
                ""
            }),
            map(inside_brackets, |s| {
                inside.push(s);
                ""
            }),
        ))(remaining)
        {
            Ok((next_input, _)) => {
                remaining = next_input;
            }
            Err(_) => break,
        }
    }

    Ok((remaining, (outside, inside)))
}

#[inline(always)]
fn abba_check(input: &&&str) -> bool {
    input
        .as_bytes()
        .array_windows()
        .any(|[a1, b1, b2, a2]| (a1 != b1) && (a1 == a2) && (b1 == b2))
}

#[inline(always)]
fn aba_check(input: &str) -> Vec<&[u8; 3]> {
    input
        .as_bytes()
        .array_windows()
        .filter(|[a1, b, a2]| (a1 != b) && (a1 == a2))
        .collect()
}

fn has_abba(matches: &[&str]) -> bool {
    matches.iter().filter(abba_check).count() > 0
}

pub fn get_abas<'a>(matches: &'a [&str]) -> Vec<&'a [u8; 3]> {
    matches.iter().flat_map(|m| aba_check(m)).collect()
}

fn transform_to_bab(input: &[u8; 3]) -> String {
    let remapped = [input[1], input[0], input[1]];
    from_utf8(&remapped).unwrap().to_owned()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    Some(
        lines
            .iter()
            .filter(|l| {
                let (_, (outside, inside)) = parse_sections(l).unwrap();
                has_abba(&outside) && !has_abba(&inside)
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    Some(
        lines
            .iter()
            .filter(|l| {
                let (_, (outside, inside)) = parse_sections(l).unwrap();
                let outside = get_abas(&outside);
                let inside: Vec<&str> = get_abas(&inside)
                    .iter()
                    .map(|&a| from_utf8(a).unwrap())
                    .collect();

                outside
                    .iter()
                    .map(|o| transform_to_bab(o))
                    .any(|o| inside.contains(&o.as_str()))
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
