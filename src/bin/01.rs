advent_of_code::solution!(1);

use aoc_mine::Coord;

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;

impl Direction {
    fn new_direction(c: &str, direction: Self) -> Self {
        if c == "R" {
            match direction {
                North => East,
                East => South,
                South => West,
                West => North,
            }
        } else {
            match direction {
                North => West,
                West => South,
                South => East,
                East => North,
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut coord: Coord<i32> = Coord(0, 0);

    let mut current_direction = Direction::North;
    input
        .trim()
        .split(',')
        .map(|i| i.trim())
        .for_each(|instruction| {
            let (direction, count) = instruction.split_at(1);
            let count: u32 = count.parse().expect("could not parse {count}");

            current_direction = Direction::new_direction(direction, current_direction);
            (0..count).for_each(|_| match current_direction {
                North => coord.move_up(),
                East => coord.move_right(),
                South => coord.move_down(),
                West => coord.move_left(),
            });
        });

    Some(coord.x().unsigned_abs() + coord.y().unsigned_abs())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut coord: Coord<i32> = Coord(0, 0);
    let mut visited: Vec<Coord<i32>> = vec![coord];
    let mut found = false;

    let mut current_direction = Direction::North;
    input
        .trim()
        .split(',')
        .map(|i| i.trim())
        .for_each(|instruction| {
            if !found {
                let (direction, count) = instruction.split_at(1);
                let count: u32 = count.parse().expect("could not parse {count}");

                current_direction = Direction::new_direction(direction, current_direction);
                (0..count).for_each(|_| match current_direction {
                    North => coord.move_up(),
                    East => coord.move_right(),
                    South => coord.move_down(),
                    West => coord.move_left(),
                });

                if visited.contains(&coord) {
                    found = true;
                } else {
                    visited.push(coord);
                }
            }
        });

    Some(coord.x().unsigned_abs() + coord.y().unsigned_abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        assert_eq!(part_one("R2, L3"), Some(5));
    }

    #[test]
    fn test_part_two_1() {
        assert_eq!(part_two("R8, R4, R4, R8"), Some(4));
    }

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
