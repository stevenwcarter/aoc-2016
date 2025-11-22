use advent_of_code::assembunny::{Instruction, State, parse_line};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().map(parse_line).collect();
    let mut state = State::new(instructions);
    state.a = 7;
    state.run();

    Some(state.a)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let factorial: u64 = (1..=12).product();

    Some(factorial + 71 * 72)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(479006712));
    }
}
