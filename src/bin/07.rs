advent_of_code::solution!(7);

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take},
    combinator::{map, verify},
    sequence::{delimited, tuple},
    IResult,
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

fn abba_pattern(input: &str) -> IResult<&str, &str> {
    verify(
        tuple((
            take(1usize), // First character
            take(1usize), // Second character
            take(1usize), // Third character
            take(1usize), // Fourth character
        )),
        |(a, b, c, d)| a == d && b == c && a != b, // Ensure the "abba" pattern
    )(input)
    .map(|(remaining, _)| (remaining, input)) // Return input for traversal
}

/// Parser to check if any "abba" pattern exists in the input
fn contains_abba(input: &str) -> bool {
    // Slide through the string looking for the pattern
    let mut slice = input;

    while slice.len() >= 4 {
        if let Ok((_, _)) = abba_pattern(slice) {
            return true;
        }
        slice = &slice[1..]; // Move forward by one character
    }

    false
}

fn has_abba(matches: &[&str]) -> bool {
    matches.iter().filter(|m| contains_abba(m)).count() > 0
}

fn aba_pattern(input: &str) -> IResult<&str, &str> {
    verify(
        tuple((
            take(1usize), // First character
            take(1usize), // Second character
            take(1usize), // Third character
        )),
        |(a, b, c)| a == c && a != b, // Check the "aba" condition
    )(input)
    .map(|(remaining, (_, _, _))| (remaining, &input[0..3])) // Return the matched pattern
}

fn collect_aba_patterns(input: &str) -> Vec<&str> {
    let mut slice = input;
    let mut matches = Vec::new();

    while slice.len() >= 3 {
        if let Ok((_, matched)) = aba_pattern(slice) {
            matches.push(matched);
            slice = &slice[1..]; // Move the window by one character
        } else {
            slice = &slice[1..]; // Move the window by one character
        }
    }

    matches
}

pub fn get_abas<'a>(matches: &'a [&str]) -> Vec<&'a str> {
    matches
        .iter()
        .flat_map(|m| collect_aba_patterns(m))
        .collect()
}

fn transform_to_bab(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    if chars.len() == 3 {
        chars[2] = chars[1];
        chars[1] = chars[0];
        chars[0] = chars[2];
    }
    chars.into_iter().collect() // Convert back to a String
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
                let inside = get_abas(&inside);

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
