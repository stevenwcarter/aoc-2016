use rayon::prelude::*;
advent_of_code::solution!(4);

use hashbrown::HashMap;

use nom::{
    bytes::complete::{is_not, tag, take_while},
    character::complete::digit1,
    combinator::map_res,
    sequence::delimited,
    IResult,
};

fn parse_u32(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn parse_data_nom(input: &str) -> IResult<&str, (&str, u32, &str)> {
    let (input, part1) = take_while(|c: char| !c.is_ascii_digit() || c == '-')(input)?;
    let (input, number) = map_res(digit1, parse_u32)(input)?;
    let (input, part2) = delimited(tag("["), is_not("]"), tag("]"))(input)?;

    Ok((input, (&part1[..part1.len() - 1], number, part2)))
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
                let result = shift_letters(letters, id);
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
        let result = parse_data_nom("aaaaa-bbb-z-y-x-123[abxyz]");

        assert!(result.is_ok());
        let result = result.unwrap().1;
        assert_eq!(result.0, "aaaaa-bbb-z-y-x");
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
