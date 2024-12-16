advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Trap,
    Safe,
}

use Tile::*;

pub struct State {
    pub prior_row: Vec<Tile>,
    pub row_length: usize,
    pub needed: usize,
    pub safe_tile_count: u32,
}

impl State {
    pub fn parse_input(input: &str) -> Self {
        let (row, needed) = input.split_once(' ').unwrap();
        let mut row_length = 0;
        let mut safe_tile_count = 0;
        let prior_row: Vec<Tile> = row
            .chars()
            .map(|ch| {
                row_length += 1;
                match ch {
                    '.' => {
                        safe_tile_count += 1;
                        Tile::Safe
                    }
                    '^' => Tile::Trap,
                    _ => unreachable!("Cannot parse {ch}"),
                }
            })
            .collect();
        let needed = needed.trim_end().parse::<usize>().unwrap();

        Self {
            prior_row,
            safe_tile_count,
            needed,
            row_length,
        }
    }

    pub fn set_row_count(&mut self, needed: usize) {
        self.needed = needed;
    }

    pub fn process(&mut self) {
        (0..self.needed - 1).for_each(|_| {
            let new_row: Vec<Tile> = (0..self.row_length)
                .map(|idx| {
                    let p1 = self
                        .prior_row
                        .get(idx.wrapping_sub(1))
                        .unwrap_or(&Tile::Safe);
                    let p2 = self.prior_row.get(idx).unwrap_or(&Tile::Safe);
                    let p3 = self.prior_row.get(idx + 1).unwrap_or(&Tile::Safe);

                    match (p1, p2, p3) {
                        (Trap, Trap, Safe) => Trap,
                        (Safe, Trap, Trap) => Trap,
                        (Trap, Safe, Safe) => Trap,
                        (Safe, Safe, Trap) => Trap,
                        _ => {
                            self.safe_tile_count += 1;
                            Safe
                        }
                    }
                })
                .collect();
            self.prior_row = new_row;
        })
    }

    pub fn count_safe(&self) -> Option<u32> {
        Some(self.safe_tile_count)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut state = State::parse_input(input);

    state.process();

    state.count_safe()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut state = State::parse_input(input);
    state.set_row_count(400_000);

    state.process();

    state.count_safe()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(38));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
