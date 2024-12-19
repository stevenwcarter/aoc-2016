use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Split(pub u32, pub u32);

impl Split {
    fn combine(&mut self, other: Self) -> Option<Self> {
        if self.0 < other.0 && self.1 > other.1 {
            return None;
        }
        if other.0 < self.0 && other.1 > self.1 {
            self.0 = other.0;
            self.1 = other.1;
            return None;
        }
        if self.0 < other.0 && self.1 < other.1 && other.0 < self.1 {
            self.1 = other.1;
            return None;
        }
        if other.0 < self.0 && other.1 < self.1 && self.0 < other.1 {
            self.0 = other.0;
            return None;
        }
        if self.1.saturating_add(1) == other.0 {
            self.1 = other.1;
            return None;
        }
        if other.1.saturating_add(1) == self.0 {
            self.0 = other.0;
            return None;
        }

        // println!("Fallthrough");
        Some(other)
    }
}

fn parse_input(input: &str) -> VecDeque<Split> {
    let mut vec: Vec<Split> = input
        .lines()
        .map(|l| {
            let sp: Vec<u32> = l.split('-').map(|c| c.parse::<u32>().unwrap()).collect();
            Split(sp[0], sp[1])
        })
        .collect();

    vec.sort_unstable();

    vec.into_iter().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut splits = parse_input(input);

    condense_splits(&mut splits);

    let mut vec: Vec<Split> = splits.into_iter().collect();

    vec.sort_unstable();

    Some(vec[0].1 + 1)
}

fn condense_splits(splits: &mut VecDeque<Split>) {
    let condense_iters = splits.len() / 8;
    (0..condense_iters).for_each(|_| {
        let mut next = splits.pop_front().unwrap();
        (0..splits.len()).for_each(|_| {
            if let Some(other) = next.combine(splits.pop_front().unwrap()) {
                splits.push_back(other);
            }
        });
        splits.push_back(next);
    });
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut splits = parse_input(input);

    condense_splits(&mut splits);

    let mut vec: Vec<Split> = splits.into_iter().collect();
    vec.sort_unstable();

    Some(
        vec.iter()
            .tuple_windows()
            .filter(|(a, b)| b.0 > a.1)
            .map(|(a, b)| b.0 - a.1 - 1)
            .sum(),
    )
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
        assert_eq!(result, Some(2));
    }
}
