advent_of_code::solution!(15);

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
};

#[derive(Debug, Clone, Copy)]
pub struct Disk {
    pub position_count: u32,
    pub zero_position: u32,
}

impl Disk {
    pub fn new(input: &(u32, u32)) -> Self {
        Self {
            position_count: input.0,
            zero_position: input.1,
        }
    }
}

// Parse an integer
fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), str::parse::<u32>)(input)
}

// Parse a single line of input
fn parse_disk(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = tag("Disc #")(input)?;
    let (input, _) = parse_u32(input)?; // Skip disc number
    let (input, _) = tag(" has ")(input)?;
    let (input, positions) = parse_u32(input)?;
    let (input, _) = tag(" positions; at time=0, it is at position ")(input)?;
    let (input, position) = parse_u32(input)?;
    let (input, _) = tag(".\n")(input)?;

    Ok((input, (positions, position)))
}

// Parse multiple lines of input
fn parse_all_disks(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    nom::multi::many1(parse_disk)(input)
}

fn find_start(disks: &[Disk]) -> u32 {
    let mut step = 1;

    disks
        .iter()
        .enumerate()
        .fold(0, |mut acc: u32, (i, disk): (usize, &Disk)| {
            while (disk.zero_position + acc + (i + 1) as u32) % disk.position_count != 0 {
                acc += step;
            }

            // keep track of how much to shift so we only test valid future solutions, adding the
            // time for this rotation to come around into the mix
            step *= disk.position_count;

            acc
        })
}
pub fn part_one(input: &str) -> Option<u32> {
    let disks: Vec<Disk> = parse_all_disks(input)
        .unwrap()
        .1
        .iter()
        .map(Disk::new)
        .collect();

    Some(find_start(&disks))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut disks: Vec<Disk> = parse_all_disks(input)
        .unwrap()
        .1
        .iter()
        .map(Disk::new)
        .collect();

    disks.push(Disk {
        position_count: 11,
        zero_position: 0,
    });

    Some(find_start(&disks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(85));
    }
}
