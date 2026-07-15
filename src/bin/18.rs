advent_of_code::solution!(18);

/// Parse the puzzle input into the first row's trap bitmask (bit `i` set means
/// column `i` is a trap), the row width, and the number of rows requested by the
/// input. `^` sets a bit, `.` leaves it clear.
fn parse(input: &str) -> (u128, usize, usize) {
    let (row, needed) = input.split_once(' ').unwrap_or((input.trim_end(), ""));
    let width = row.trim_end().chars().count();
    let row0 = row
        .trim_end()
        .chars()
        .enumerate()
        .filter(|&(_, ch)| ch == '^')
        .fold(0u128, |acc, (i, _)| acc | (1u128 << i));
    let needed = needed.trim().parse::<usize>().unwrap_or(0);
    (row0, width, needed)
}

/// Count the safe tiles across `needed` rows, starting from `row0`.
///
/// Every trap rule reduces to "the left and right neighbours differ", so the
/// next row is `(row << 1) ^ (row >> 1)` masked to the grid width; the shifts
/// bring in safe (0) tiles off the edges, which is the desired behaviour.
fn count_safe(row0: u128, width: usize, needed: usize) -> u32 {
    if needed == 0 {
        return 0;
    }
    let mask = (1u128 << width) - 1;
    let width = width as u32;
    let mut row = row0;
    let mut safe_total = width - row.count_ones();
    for _ in 0..needed - 1 {
        row = ((row << 1) ^ (row >> 1)) & mask;
        safe_total += width - row.count_ones();
    }
    safe_total
}

pub fn part_one(input: &str) -> Option<u32> {
    let (row0, width, needed) = parse(input);
    Some(count_safe(row0, width, needed))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (row0, width, _) = parse(input);
    Some(count_safe(row0, width, 400_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(38));
    }
}
