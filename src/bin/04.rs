use rayon::prelude::*;
advent_of_code::solution!(4);

use hashbrown::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([a-z-]+)-(\d+)\[([a-z]+)\]$").unwrap();
}

fn parse_data(input: &str) -> Option<(String, u32, &str, &str)> {
    if let Some(captures) = RE.captures(input) {
        let letters = captures.get(1)?.as_str();
        let replaced = letters.replace("-", "");
        let id = captures.get(2)?.as_str().parse::<u32>().unwrap();
        let checksum = captures.get(3)?.as_str();

        Some((replaced, id, checksum, letters))
    } else {
        None
    }
}
fn parse_data_nom(input: &str) -> IResult<&str, (String, u32, &str)> {
    let (input, (parts, id, checksum)) = tuple((
        separated_list1(tag("-"), alpha1),
        preceded(tag("-"), map_res(digit1, str::parse)),
        delimited(tag("["), alpha1, tag("]")),
    ))(input)?;

    Ok((input, (parts.join("-"), id, checksum)))
}

fn top_five_letters(text: &str) -> String {
    let mut letter_counts = HashMap::with_capacity(30);

    for c in text.chars() {
        *letter_counts.entry(c).or_insert(0) += 1;
    }

    let mut sorted_letters: Vec<_> = letter_counts.into_iter().collect();
    sorted_letters.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    sorted_letters.iter().take(5).map(|&(c, _)| c).collect()
}

fn shift_letters(input: &str, shift: u32) -> String {
    let shift = shift % 26;
    input
        .chars()
        .map(|c| match c {
            '-' => ' ',
            _ => {
                let mut char_code = c as u32;
                char_code += shift;
                while char_code > 'z' as u32 {
                    char_code -= 26;
                }
                char_code as u8 as char
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    Some(
        lines
            .par_iter()
            .filter_map(|l| match parse_data_nom(l) {
                Ok((_, (letters, id, checksum))) => Some((letters, id, checksum)),
                Err(e) => {
                    println!("Error: {:?}", e);

                    None
                }
            })
            .filter_map(|(letters, id, checksum)| {
                let chars = letters.replace("-", "");
                let top_five = top_five_letters(&chars);

                if top_five.eq(checksum) {
                    return Some(id);
                }
                None
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .par_iter()
        .filter_map(|l| match parse_data_nom(l) {
            Ok((_, (letters, id, checksum))) => Some((letters, id, checksum)),
            Err(e) => {
                println!("Error: {:?}", e);

                None
            }
        })
        .filter_map(|(letters, id, checksum)| {
            let chars = letters.replace("-", "");
            let top_five = top_five_letters(&chars);

            if top_five.eq(checksum) {
                let result = shift_letters(&letters, id);
                if result.contains("northpole") {
                    return Some(id);
                }
            }
            None
        })
        .find_any(|_| true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_data_1() {
        let result = parse_data("aaaaa-bbb-z-y-x-123[abxyz]");

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.0, "aaaaabbbzyx".to_string());
        assert_eq!(result.1, 123);
        assert_eq!(result.2, "abxyz");
    }

    #[test]
    fn shift_letters_test() {
        let result = shift_letters("qzmtzixmtkozyivhz", 343);

        assert_eq!(result, "veryencryptedname");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1838));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(324));
    }
}
