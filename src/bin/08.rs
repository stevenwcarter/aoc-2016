advent_of_code::solution!(8);

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{preceded, separated_pair},
    IResult,
};

#[cfg(test)]
const ROWS: u32 = 3;
#[cfg(test)]
const COLS: u32 = 7;
#[cfg(not(test))]
const ROWS: u32 = 6;
#[cfg(not(test))]
const COLS: u32 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    x: u32,
    y: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotateRow {
    y: u32,
    offset: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotateColumn {
    x: u32,
    offset: u32,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_rect, parse_rotate_row, parse_rotate_column))(input)
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_rect(input: &str) -> IResult<&str, Instruction> {
    let parser = preceded(tag("rect "), separated_pair(parse_u32, tag("x"), parse_u32));
    map(parser, |(x, y)| Instruction::Rect(Rect { x, y }))(input)
}

fn parse_rotate_row(input: &str) -> IResult<&str, Instruction> {
    let parser = preceded(
        tag("rotate row y="),
        separated_pair(parse_u32, tag(" by "), parse_u32),
    );
    map(parser, |(y, offset)| {
        Instruction::RotateRow(RotateRow { y, offset })
    })(input)
}

fn parse_rotate_column(input: &str) -> IResult<&str, Instruction> {
    let parser = preceded(
        tag("rotate column x="),
        separated_pair(parse_u32, tag(" by "), parse_u32),
    );
    map(parser, |(x, offset)| {
        Instruction::RotateColumn(RotateColumn { x, offset })
    })(input)
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Rect(Rect),
    RotateRow(RotateRow),
    RotateColumn(RotateColumn),
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        match parse_instruction(line) {
            Ok((_, instruction)) => instructions.push(instruction),
            Err(err) => eprintln!("Failed to parse '{}': {:?}", line, err),
        }
    }

    instructions
}

fn handle_rect(grid: &mut [bool], instruction: Rect) {
    for x in 0..instruction.x {
        for y in 0..instruction.y {
            let pos = (y * COLS + x) as usize;
            grid[pos] = true;
        }
    }
}

fn handle_row(grid: &mut [bool], instruction: RotateRow) {
    let start = (instruction.y * COLS) as usize;
    let end = ((instruction.y + 1) * COLS) as usize;
    let mut row_slice = grid[start..end].to_vec();
    row_slice.rotate_right(instruction.offset as usize);

    grid[start..end].copy_from_slice(&row_slice);
}

fn handle_column(grid: &mut [bool], instruction: RotateColumn) {
    let mut col: Vec<bool> = Vec::new();

    (0..ROWS).for_each(|row| {
        col.push(*grid.get((row * COLS + instruction.x) as usize).unwrap());
    });

    col.rotate_right(instruction.offset as usize);

    (0..ROWS).for_each(|row| {
        let pos = (row * COLS + instruction.x) as usize;
        let value = col.get(row as usize).unwrap();
        grid[pos] = *value;
    });
}

fn print_grid(grid: &[bool]) {
    for row in 0..ROWS {
        let start = (row * COLS) as usize;
        let end = ((row + 1) * COLS) as usize;
        let line = grid[start..end]
            .iter()
            .map(|v| match v {
                true => '#',
                false => ' ',
            })
            .collect::<String>();
        println!("{line}");
    }
    println!("---");
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<bool> = vec![false; (ROWS * COLS) as usize];
    let instructions = parse_input(input);
    // print_grid(&grid);
    for instruction in instructions {
        match instruction {
            Instruction::Rect(rect) => handle_rect(&mut grid, rect),
            Instruction::RotateRow(row) => handle_row(&mut grid, row),
            Instruction::RotateColumn(col) => handle_column(&mut grid, col),
        }
        // print_grid(&grid);
    }

    Some(grid.iter().filter(|v| **v).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        let input = "rect 1x1
rotate row y=0 by 20
rotate column x=4 by 10";
        let instructions = parse_input(input);
        assert_eq!(
            *instructions.first().unwrap(),
            Instruction::Rect(Rect { x: 1, y: 1 })
        );
        assert_eq!(
            *instructions.get(1).unwrap(),
            Instruction::RotateRow(RotateRow { y: 0, offset: 20 })
        );
        assert_eq!(
            *instructions.get(2).unwrap(),
            Instruction::RotateColumn(RotateColumn { x: 4, offset: 10 })
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
