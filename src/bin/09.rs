advent_of_code::solution!(9);

use nom::{
    branch::alt,
    bytes::complete::{take, take_until},
    character::complete::char,
    sequence::delimited,
    IResult,
};

fn decompress(input: &str) -> String {
    let mut result = String::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        match parse_chunk(remaining) {
            Ok((next_input, chunk)) => {
                result.push_str(&chunk);
                remaining = next_input;
            }
            Err(_) => {
                // Append the rest if parsing fails
                result.push_str(remaining);
                break;
            }
        }
    }

    result
}

fn decompressed_length(input: &str) -> usize {
    let mut length = 0;
    let mut remaining = input;

    while !remaining.is_empty() {
        match parse_chunk_length(remaining) {
            Ok((next_input, chunk_length)) => {
                length += chunk_length;
                remaining = next_input;
            }
            Err(_) => {
                // If parsing fails, count the remaining characters as literal
                length += remaining.len();
                break;
            }
        }
    }

    length
}

fn parse_chunk_length(input: &str) -> IResult<&str, usize> {
    alt((parse_marker_with_repetition_length, parse_literal_length))(input)
}

fn parse_literal_length(input: &str) -> IResult<&str, usize> {
    let (remaining, _) = take(1usize)(input)?;
    Ok((remaining, 1))
}

fn parse_marker_with_repetition_length(input: &str) -> IResult<&str, usize> {
    let (remaining, (num_chars, repeat)) = parse_marker_length(input)?;
    let (remaining, to_repeat) = take(num_chars)(remaining)?;
    // Recursively compute the length of the section and multiply by repeat count
    let decompressed_section_length = decompressed_length(to_repeat);
    Ok((remaining, decompressed_section_length * repeat))
}

fn parse_chunk(input: &str) -> IResult<&str, String> {
    alt((parse_marker_with_repetition, parse_literal))(input)
}

fn parse_literal(input: &str) -> IResult<&str, String> {
    let (remaining, literal) = take(1usize)(input)?;
    Ok((remaining, literal.to_string()))
}

fn parse_marker_with_repetition(input: &str) -> IResult<&str, String> {
    let (remaining, (num_chars, repeat)) = parse_marker(input)?;
    let (remaining, to_repeat) = take(num_chars)(remaining)?;
    let repeated = to_repeat.repeat(repeat);
    Ok((remaining, repeated))
}

fn parse_marker(input: &str) -> IResult<&str, (usize, usize)> {
    let (remaining, marker_content) = delimited(char('('), take_until(")"), char(')'))(input)?;
    let (num_chars, repeat) = marker_content.split_once('x').unwrap();
    let num_chars = num_chars.parse::<usize>().unwrap();
    let repeat = repeat.parse::<usize>().unwrap();
    Ok((remaining, (num_chars, repeat)))
}

fn parse_marker_length(input: &str) -> IResult<&str, (usize, usize)> {
    // Parse the marker in the format (NxM)
    let (remaining, marker_content) = delimited(char('('), take_until(")"), char(')'))(input)?;
    let (num_chars, repeat) = marker_content.split_once('x').unwrap();
    let num_chars = num_chars.parse::<usize>().unwrap();
    let repeat = repeat.parse::<usize>().unwrap();
    Ok((remaining, (num_chars, repeat)))
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(decompress(input.trim()).len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let len = decompressed_length(input.trim()) as u64;

    Some(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(part_one("ADVENT"), Some(6));
    }
    #[test]
    fn test_2() {
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(part_one("A(1x5)BC"), Some(7));
    }
    #[test]
    fn test_3() {
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(part_one("(3x3)XYZ"), Some(9));
    }
    #[test]
    fn test_4() {
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(part_one("A(2x2)BCD(2x2)EFG"), Some(11));
    }
    #[test]
    fn test_5() {
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(part_one("(6x1)(1x3)A"), Some(6));
    }
    #[test]
    fn test_6() {
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
        assert_eq!(part_one("X(8x2)(3x3)ABCY"), Some(18));
    }
    #[test]
    fn test_7() {
        assert_eq!(
            decompress("(8x2)(3x3)ABCY(3x5)ABC"),
            "(3x3)ABC(3x3)ABCYABCABCABCABCABC"
        );
    }
    #[test]
    fn test_8() {
        assert_eq!(decompress("(11x2)ABCABCABCABP"), "ABCABCABCABABCABCABCABP");
    }
    #[test]
    fn test_9() {
        assert_eq!(decompress("(11x1)ABCABCABCABP"), "ABCABCABCABP");
    }

    #[test]
    fn test_2_1() {
        assert_eq!(
            decompressed_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        )
    }

    #[test]
    fn test_2_2() {
        assert_eq!(
            decompressed_length("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.as_ref().unwrap() < &99146);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
