use std::{collections::VecDeque, u32};

use advent_of_code::Point;
use hashbrown::HashSet;
use pathfinding::prelude::bfs;

advent_of_code::solution!(13);

trait IsWall {
    fn is_wall(&self, fave_number: u32) -> bool;
}

impl IsWall for Point {
    fn is_wall(&self, fave_number: u32) -> bool {
        let (x, y) = (self.x, self.y);

        let result = x.pow(2) + 3 * x + 2 * x * y + y + y * y + fave_number;

        result.count_ones() % 2 != 0
    }
}

fn reachable_locations(start: Point, max_steps: usize, fave_number: u32) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((current, steps)) = queue.pop_front() {
        if steps > max_steps || !visited.insert(current) {
            continue;
        }

        current
            .udlr([0, u32::MAX, 0, u32::MAX])
            .iter()
            .filter(|p| !p.is_wall(fave_number))
            .filter(|&p| !visited.contains(p))
            .for_each(|&p| queue.push_back((p, steps + 1)));
    }

    visited.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let fave_number = input
        .trim_end()
        .parse::<u32>()
        .expect("Could not parse as number");

    let goal = if fave_number == 10 {
        Point::from((7u32, 4))
    } else {
        Point::from((31u32, 39))
    };

    let start = Point::from((1usize, 1));

    let result = bfs(
        &start,
        |p| {
            p.udlr([0, u32::MAX, 0, u32::MAX])
                .iter()
                .filter(|p| !p.is_wall(fave_number))
                .copied()
                .collect::<Vec<Point>>()
        },
        |&p| p == goal,
    )
    .expect("could not complete BFS");
    Some(result.len() - 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let fave_number = input
        .trim_end()
        .parse::<u32>()
        .expect("Could not parse as number");

    let max_steps = if fave_number == 10 { 10 } else { 50 };

    let start = Point::from((1usize, 1));

    let result = reachable_locations(start, max_steps, fave_number);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }
}
