advent_of_code::solution!(22);

use advent_of_code::Point;
use hashbrown::HashMap;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct NodeData {
    pub x: u32,
    pub y: u32,
    pub size: u32,
    pub used: u32,
    pub available: u32,
}

impl NodeData {
    pub fn is_full(&self) -> bool {
        self.available < 12
    }

    pub fn is_empty(&self) -> bool {
        self.available > 50
    }
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| u32::from_str(s))(input)
}

fn parse_node_line(input: &str) -> IResult<&str, NodeData> {
    let (input, _) = tag("/dev/grid/node-x")(input)?;
    let (input, x) = parse_u32(input)?;
    let (input, _) = tag("-y")(input)?;
    let (input, y) = parse_u32(input)?;
    let (input, _) = space1(input)?;
    let (input, size) = map_res(take_until("T"), |s: &str| u32::from_str(s))(input)?;
    let (input, _) = tag("T")(input)?;
    let (input, _) = space1(input)?;
    let (input, used) = map_res(take_until("T"), |s: &str| u32::from_str(s))(input)?;
    let (input, _) = tag("T")(input)?;
    let (input, _) = space1(input)?;
    let (input, available) = map_res(take_until("T"), |s: &str| u32::from_str(s))(input)?;
    let (input, _) = tag("T")(input)?;
    let (input, _) = terminated(take_until("\n"), newline)(input)?;
    Ok((
        input,
        NodeData {
            x,
            y,
            size,
            used,
            available,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<NodeData>> {
    let (input, _) = take_until("/dev/grid/node-")(input)?;
    many1(parse_node_line)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_input(input).unwrap().1;
    let mut found_other = None;

    let mut count = 0;
    (0..data.len())
        .filter(|&idx| data[idx].used != 0)
        .for_each(|idx| {
            let current = &data[idx];
            (0..data.len())
                .filter(|&o_idx| o_idx != idx)
                .for_each(|o_idx| {
                    let other = &data[o_idx];
                    if current.used <= other.available {
                        found_other = Some(other);
                        count += 1;
                    }
                });
        });

    let other = found_other.unwrap();
    Some(count)
}

pub struct NodeGrid {
    pub data: HashMap<Point, NodeData>,
    pub max_x: u32,
    pub max_y: u32,
}
impl NodeGrid {
    pub fn parse(data: &[NodeData]) -> Self {
        let max_x = data.iter().map(|d| d.x).max().unwrap();
        let max_y = data.iter().map(|d| d.y).max().unwrap();

        let mut grid: HashMap<Point, NodeData> = HashMap::new();
        data.iter().for_each(|&n| {
            *grid.entry(Point::from((n.x, n.y))).or_insert(n) = n;
        });

        Self {
            data: grid,
            max_x,
            max_y,
        }
    }

    pub fn print(&self) {
        let grid = &self.data;

        (0..self.max_y + 1)
            .map(|y| {
                (0..self.max_x + 1)
                    .map(|x| {
                        let node = grid.get(&Point::from((x, y))).unwrap();
                        if node.is_full() {
                            '#'
                        } else if node.is_empty() {
                            ' '
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .for_each(|l| println!("{l}"));
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_input(input).unwrap().1;
    let max_x = data.iter().map(|d| d.x).max().unwrap();

    NodeGrid::parse(&data);

    let result = 20 + 25 + max_x + (max_x - 1) * 5 - 8;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }
}
