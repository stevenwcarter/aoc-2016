advent_of_code::solution!(21);

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res, opt},
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnPosition(char),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
}

pub fn scramble(input: &mut Vec<char>, instruction: &Instruction) {
    match instruction {
        Instruction::SwapPosition(a, b) => input.swap(*a, *b),
        Instruction::SwapLetter(l1, l2) => {
            let a_idx = input.iter().position(|c| c == l1).unwrap();
            let b_idx = input.iter().position(|c| c == l2).unwrap();
            input.swap(a_idx, b_idx);
        }
        Instruction::RotateLeft(amount) => input.rotate_left(*amount),
        Instruction::RotateRight(amount) => input.rotate_right(*amount),
        Instruction::RotateBasedOnPosition(position) => {
            let index = input.iter().position(|c| c == position).unwrap();
            let rotation_amount = if index >= 4 { index + 2 } else { index + 1 };

            let input_len = input.len();

            input.rotate_right(rotation_amount % input_len);
        }
        Instruction::ReversePositions(a, b) => {
            let portion = input.to_owned();
            let (a, b) = if a > b { (b, a) } else { (a, b) };
            let portion = &portion[*a..=*b];

            input.splice(*a..=*b, portion.iter().rev().copied());
        }
        Instruction::MovePosition(a, b) => {
            let a_char = input.remove(*a);
            input.insert(*b, a_char);
        }
    }
}
pub fn unscramble(input: &mut Vec<char>, instruction: &Instruction) {
    match instruction {
        Instruction::SwapPosition(a, b) => input.swap(*a, *b),
        Instruction::SwapLetter(l1, l2) => {
            let a_idx = input.iter().position(|c| c == l1).unwrap();
            let b_idx = input.iter().position(|c| c == l2).unwrap();
            input.swap(a_idx, b_idx);
        }
        Instruction::RotateLeft(amount) => input.rotate_right(*amount),
        Instruction::RotateRight(amount) => input.rotate_left(*amount),
        Instruction::RotateBasedOnPosition(_) => {
            let input_copy = input.clone();

            // gave up on reverse engineering this one, I don't think it is possible
            loop {
                input.rotate_left(1);
                let mut input_check = input.clone();
                scramble(&mut input_check, instruction);
                if input_check == input_copy {
                    return;
                }
            }
        }
        Instruction::ReversePositions(a, b) => {
            let portion = input.to_owned();
            let (a, b) = if a > b { (b, a) } else { (a, b) };
            let portion = &portion[*a..=*b];

            input.splice(*a..=*b, portion.iter().rev().copied());
        }
        Instruction::MovePosition(a, b) => {
            let a_char = input.remove(*b);
            input.insert(*a, a_char);
        }
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn parse_char(input: &str) -> IResult<&str, char> {
    take_while1(|c: char| c.is_alphabetic())(input)
        .map(|(next_input, matched)| (next_input, matched.chars().next().unwrap()))
}

pub fn parse_swap_position(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            tag("swap position "),
            parse_usize,
            tag(" with position "),
            parse_usize,
        )),
        |(_, x, _, y)| Instruction::SwapPosition(x, y),
    )(input)
}

pub fn parse_swap_letter(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            tag("swap letter "),
            parse_char,
            tag(" with letter "),
            parse_char,
        )),
        |(_, x, _, y)| Instruction::SwapLetter(x, y),
    )(input)
}

pub fn parse_rotate(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(
            tuple((
                tag("rotate left "),
                parse_usize,
                space1,
                tag("step"),
                opt(tag("s")),
            )),
            |(_, x, _, _, _)| Instruction::RotateLeft(x),
        ),
        map(
            tuple((
                tag("rotate right "),
                parse_usize,
                space1,
                tag("step"),
                tag("s"),
            )),
            |(_, x, _, _, _)| Instruction::RotateRight(x),
        ),
    ))(input)
}

pub fn parse_rotate_based_on_position(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("rotate based on position of letter "), parse_char),
        Instruction::RotateBasedOnPosition,
    )(input)
}

pub fn parse_reverse_positions(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            tag("reverse positions "),
            parse_usize,
            tag(" through "),
            parse_usize,
        )),
        |(_, x, _, y)| Instruction::ReversePositions(x, y),
    )(input)
}

pub fn parse_move_position(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            tag("move position "),
            parse_usize,
            tag(" to position "),
            parse_usize,
        )),
        |(_, x, _, y)| Instruction::MovePosition(x, y),
    )(input)
}

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    terminated(
        alt((
            parse_swap_position,
            parse_swap_letter,
            parse_rotate,
            parse_rotate_based_on_position,
            parse_reverse_positions,
            parse_move_position,
        )),
        newline,
    )(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_instruction)(input)
}

pub fn part_one(input: &str) -> Option<String> {
    let steps = parse_input(input).unwrap().1;

    let passcode = if steps.len() > 10 {
        "abcdefgh"
    } else {
        "abcde"
    };
    let mut passcode = passcode.chars().collect::<Vec<char>>();

    steps.iter().for_each(|s| scramble(&mut passcode, s));

    Some(passcode.iter().collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let steps = parse_input(input).unwrap().1;

    let passcode = if steps.len() > 10 {
        "fbgdceah"
    } else {
        "decab"
    };
    let mut passcode = passcode.chars().collect::<Vec<char>>();

    steps
        .iter()
        .rev()
        .for_each(|s| unscramble(&mut passcode, s));

    Some(passcode.iter().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_unscramble_test {
        ($name:ident, $instruction:expr) => {
            #[test]
            fn $name() {
                let passcode_check: Vec<char> = "abcdefgh".chars().collect();
                println!("Starting with {:?}", passcode_check);
                let mut passcode = passcode_check.clone();
                scramble(&mut passcode, &$instruction);
                println!("Scrambled to  {:?}", passcode);
                unscramble(&mut passcode, &$instruction);
                println!("Trying {:?}", $instruction);
                assert_eq!(passcode, passcode_check);
            }
        };
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("decab".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("abcde".to_string()));
    }

    #[test]
    fn test_parse_swap_position() {
        assert_eq!(
            parse_swap_position("swap position 4 with position 0"),
            Ok(("", Instruction::SwapPosition(4, 0)))
        );
    }

    #[test]
    fn test_parse_swap_letter() {
        assert_eq!(
            parse_swap_letter("swap letter d with letter b"),
            Ok(("", Instruction::SwapLetter('d', 'b')))
        );
    }

    #[test]
    fn test_parse_rotate() {
        assert_eq!(
            parse_rotate("rotate left 1 step"),
            Ok(("", Instruction::RotateLeft(1)))
        );
        assert_eq!(
            parse_rotate("rotate right 3 steps"),
            Ok(("", Instruction::RotateRight(3)))
        );
    }

    #[test]
    fn test_parse_rotate_based_on_position() {
        assert_eq!(
            parse_rotate_based_on_position("rotate based on position of letter b"),
            Ok(("", Instruction::RotateBasedOnPosition('b')))
        );
    }

    #[test]
    fn test_parse_reverse_positions() {
        assert_eq!(
            parse_reverse_positions("reverse positions 0 through 4"),
            Ok(("", Instruction::ReversePositions(0, 4)))
        );
    }

    #[test]
    fn test_parse_move_position() {
        assert_eq!(
            parse_move_position("move position 3 to position 0"),
            Ok(("", Instruction::MovePosition(3, 0)))
        );
    }

    #[test]
    fn test_swap_position() {
        let mut test = "12345".chars().collect::<Vec<char>>();
        scramble(&mut test, &Instruction::SwapPosition(4, 0));

        let test = test.iter().collect::<String>();

        assert_eq!(test, "52341");
    }
    #[test]
    fn test_swap_letter() {
        let mut test = "12345".chars().collect::<Vec<char>>();
        scramble(&mut test, &Instruction::SwapLetter('1', '5'));

        let test = test.iter().collect::<String>();

        assert_eq!(test, "52341");
    }

    #[test]
    fn test_unscramble_last_step() {
        let mut test = "decab".chars().collect::<Vec<char>>();
        unscramble(&mut test, &Instruction::RotateBasedOnPosition('d'));

        let test = test.iter().collect::<String>();

        assert_eq!(test, "ecabd");
    }

    generate_unscramble_test!(test_swap_letter_a_b, Instruction::SwapLetter('a', 'b'));
    generate_unscramble_test!(test_swap_position_2_4, Instruction::SwapPosition(2, 4));
    generate_unscramble_test!(test_rotate_left_4, Instruction::RotateLeft(4));
    generate_unscramble_test!(test_rotate_right_2, Instruction::RotateRight(2));
    generate_unscramble_test!(
        test_rotate_based_on_a,
        Instruction::RotateBasedOnPosition('a')
    );
    generate_unscramble_test!(
        test_rotate_based_on_b,
        Instruction::RotateBasedOnPosition('b')
    );
    generate_unscramble_test!(
        test_rotate_based_on_c,
        Instruction::RotateBasedOnPosition('c')
    );
    generate_unscramble_test!(
        test_rotate_based_on_d,
        Instruction::RotateBasedOnPosition('d')
    );
    generate_unscramble_test!(
        test_rotate_based_on_e,
        Instruction::RotateBasedOnPosition('e')
    );
    generate_unscramble_test!(
        test_reverse_positions_0_1,
        Instruction::ReversePositions(0, 1)
    );
    generate_unscramble_test!(
        test_reverse_positions_0_4,
        Instruction::ReversePositions(0, 4)
    );
    generate_unscramble_test!(
        test_reverse_positions_5_2,
        Instruction::ReversePositions(5, 2)
    );
    generate_unscramble_test!(test_move_position_0_1, Instruction::MovePosition(0, 1));
    generate_unscramble_test!(test_move_position_0_4, Instruction::MovePosition(0, 4));
    generate_unscramble_test!(test_move_position_5_2, Instruction::MovePosition(5, 2));
}
