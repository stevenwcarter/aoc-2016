use advent_of_code::Point;
use hashbrown::HashMap;
use pathfinding::prelude::bfs;

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Open(Option<u32>),
    Wall,
}

pub struct Maze {
    pub grid: HashMap<Point, Type>,
    pub start: Point,
    pub width: usize,
    pub height: usize,
    pub node_count: usize,
}

impl Maze {
    pub fn parse_input(input: &str) -> Self {
        let width = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>()
            .len();
        let mut start: Option<Point> = None;
        let height = input.lines().collect::<Vec<_>>().len();
        let mut node_count = 0;
        let grid: HashMap<Point, Type> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| {
                        let grid_type = match ch {
                            '#' => Type::Wall,
                            '.' => Type::Open(None),
                            d => {
                                node_count += 1;
                                let digit = d.to_digit(10).unwrap();
                                if digit == 0 {
                                    start = Some(Point::from((x, y)));
                                }
                                Type::Open(Some(digit))
                            }
                        };
                        (Point::from((x, y)), grid_type)
                    })
                    .collect::<Vec<(Point, Type)>>()
            })
            .collect();

        Self {
            grid,
            start: start.unwrap(),
            height,
            width,
            node_count,
        }
    }

    pub fn successors(&self, point: Point, visited: u32) -> Vec<(Point, u32)> {
        let neighbors = point.udlr([0, self.height as u32, 0, self.width as u32]);
        neighbors
            .iter()
            .filter_map(|n| match self.grid.get(n) {
                Some(Type::Open(None)) => Some((*n, visited)),
                Some(Type::Open(Some(d))) => {
                    let bit = 1 << d;
                    if visited & bit == 0 {
                        Some((*n, visited | bit))
                    } else {
                        Some((*n, visited))
                    }
                }
                Some(Type::Wall) | None => None,
            })
            .collect::<Vec<(Point, u32)>>()
    }

    pub fn part1_end_condition(&self, visited: &u32) -> bool {
        self.node_count as u32 == visited.count_ones()
    }
    pub fn part2_end_condition(&self, point: &Point, visited: &u32) -> bool {
        *point == self.start && self.node_count as u32 == visited.count_ones()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = Maze::parse_input(input);
    let start = maze.start;

    let start: (Point, u32) = (start, 1 << 0);

    let result = bfs(
        &start,
        |&(point, visited)| maze.successors(point, visited),
        |(_point, visited)| maze.part1_end_condition(visited),
    );

    let result = result.expect("No answer found");

    Some(result.len() - 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = Maze::parse_input(input);
    let start = maze.start;

    let start: (Point, u32) = (start, 1 << 0);

    let result = bfs(
        &start,
        |&(point, visited)| maze.successors(point, visited),
        |(point, visited)| maze.part2_end_condition(point, visited),
    );

    let result = result.expect("No answer found");

    Some(result.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20));
    }
}
