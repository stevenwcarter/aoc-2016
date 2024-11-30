use std::fmt;

advent_of_code::solution!(2);

#[derive(Clone, Copy, Debug)]
enum Keypad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    // Zero,
}
#[derive(Clone, Copy, Debug)]
enum KeypadTwo {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    // Zero,
}

impl fmt::Display for Keypad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            // Self::Zero => write!(f, "0"),
        }
    }
}
impl fmt::Display for KeypadTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

impl Keypad {
    pub fn parse(start: Option<Keypad>, instructions: &str) -> Self {
        let mut val = start.unwrap_or(Keypad::Five);
        let instructions: Vec<char> = instructions.chars().collect();
        instructions.iter().for_each(|i| match i {
            'U' => val = val.up(),
            'D' => val = val.down(),
            'L' => val = val.left(),
            'R' => val = val.right(),
            _ => panic!("Invalid instruction {i}"),
        });

        val
    }
    fn up(&self) -> Self {
        match self {
            Self::One => Self::One,
            Self::Two => Self::Two,
            Self::Three => Self::Three,
            Self::Four => Self::One,
            Self::Five => Self::Two,
            Self::Six => Self::Three,
            Self::Seven => Self::Four,
            Self::Eight => Self::Five,
            Self::Nine => Self::Six,
            // Self::Zero => Self::Eight,
        }
    }
    fn down(&self) -> Self {
        match self {
            Self::One => Self::Four,
            Self::Two => Self::Five,
            Self::Three => Self::Six,
            Self::Four => Self::Seven,
            Self::Five => Self::Eight,
            Self::Six => Self::Nine,
            Self::Seven => Self::Seven,
            Self::Eight => Self::Eight,
            Self::Nine => Self::Nine,
            // Self::Zero => Self::Zero,
        }
    }
    fn right(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Three,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Six,
            Self::Seven => Self::Eight,
            Self::Eight => Self::Nine,
            Self::Nine => Self::Nine,
            // Self::Zero => Self::Zero,
        }
    }
    fn left(&self) -> Self {
        match self {
            Self::One => Self::One,
            Self::Two => Self::One,
            Self::Three => Self::Two,
            Self::Four => Self::Four,
            Self::Five => Self::Four,
            Self::Six => Self::Five,
            Self::Seven => Self::Seven,
            Self::Eight => Self::Seven,
            Self::Nine => Self::Eight,
            // Self::Zero => Self::Zero,
        }
    }
}
impl KeypadTwo {
    pub fn parse(start: Option<KeypadTwo>, instructions: &str) -> Self {
        let mut val = start.unwrap_or(KeypadTwo::Five);
        let instructions: Vec<char> = instructions.chars().collect();
        instructions.iter().for_each(|i| match i {
            'U' => val = val.up(),
            'D' => val = val.down(),
            'L' => val = val.left(),
            'R' => val = val.right(),
            _ => panic!("Invalid instruction {i}"),
        });

        val
    }
    fn up(&self) -> Self {
        match self {
            Self::One => Self::One,
            Self::Two => Self::Two,
            Self::Three => Self::One,
            Self::Four => Self::Four,
            Self::Five => Self::Five,
            Self::Six => Self::Two,
            Self::Seven => Self::Three,
            Self::Eight => Self::Four,
            Self::Nine => Self::Nine,
            Self::A => Self::Six,
            Self::B => Self::Seven,
            Self::C => Self::Eight,
            Self::D => Self::B,
        }
    }
    fn down(&self) -> Self {
        match self {
            Self::One => Self::Three,
            Self::Two => Self::Six,
            Self::Three => Self::Seven,
            Self::Four => Self::Eight,
            Self::Five => Self::Five,
            Self::Six => Self::A,
            Self::Seven => Self::B,
            Self::Eight => Self::C,
            Self::Nine => Self::Nine,
            Self::A => Self::A,
            Self::B => Self::D,
            Self::C => Self::C,
            Self::D => Self::D,
        }
    }
    fn right(&self) -> Self {
        match self {
            Self::One => Self::One,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Four,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
            Self::Eight => Self::Nine,
            Self::Nine => Self::Nine,
            Self::A => Self::B,
            Self::B => Self::C,
            Self::C => Self::C,
            Self::D => Self::D,
        }
    }
    fn left(&self) -> Self {
        match self {
            Self::One => Self::One,
            Self::Two => Self::Two,
            Self::Three => Self::Two,
            Self::Four => Self::Three,
            Self::Five => Self::Five,
            Self::Six => Self::Five,
            Self::Seven => Self::Six,
            Self::Eight => Self::Seven,
            Self::Nine => Self::Eight,
            Self::A => Self::A,
            Self::B => Self::A,
            Self::C => Self::B,
            Self::D => Self::D,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut last_result: Option<Keypad> = None;

    let result: String = lines
        .iter()
        .map(|l| {
            last_result = Some(Keypad::parse(last_result, l));
            last_result.unwrap().to_string()
        })
        .collect();

    Some(result.parse::<u32>().unwrap())
}

pub fn part_two(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut last_result: Option<KeypadTwo> = None;

    let result: String = lines
        .iter()
        .map(|l| {
            last_result = Some(KeypadTwo::parse(last_result, l));
            last_result.unwrap().to_string()
        })
        .collect();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1985));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5DB3".to_string()));
    }
}
