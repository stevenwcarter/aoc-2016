use advent_of_code::assembunny::{parse_line, Instruction, State};

advent_of_code::solution!(12);

// * `cpy x y` *copies* `x` (either an integer or the *value* of a register) into register `y`.
// * `inc x` *increases* the value of register `x` by one.
// * `dec x` *decreases* the value of register `x` by one.
// * `jnz x y` *jumps* to an instruction `y` away (positive means forward; negative means backward), but only if `x` is *not zero*.

pub fn part_one(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().map(parse_line).collect();
    let mut state = State::new_c(instructions, 0);
    state.run();

    Some(state.a)
}

pub fn part_two(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().map(parse_line).collect();
    let mut state = State::new_c(instructions, 1);
    state.run();

    Some(state.a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}
