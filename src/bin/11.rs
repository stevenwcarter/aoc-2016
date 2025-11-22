#![allow(unused_variables)]
advent_of_code::solution!(11);
use std::hash::{Hash, Hasher};

use hashbrown::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Eq, Debug)]
struct State {
    elevator: usize,            // Current floor of the elevator
    items: Vec<(usize, usize)>, // (generator_floor, microchip_floor)
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        // Check if elevator is on the same floor
        if self.elevator != other.elevator {
            return false;
        }

        // Normalize items by grouping them based on their floors
        let normalize = |items: &[(usize, usize)]| -> Vec<(usize, usize)> {
            let mut normalized: Vec<(usize, usize)> = items
                .iter()
                .map(|&(genn, chip)| {
                    if genn < chip {
                        (genn, chip)
                    } else {
                        (chip, genn)
                    }
                })
                .collect();
            normalized.sort_unstable();
            normalized
        };

        normalize(&self.items) == normalize(&other.items)
    }
}

// impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the elevator position
        self.elevator.hash(state);

        // Normalize and hash the items
        let mut normalized: Vec<(usize, usize)> = self
            .items
            .iter()
            .map(|&(genn, chip)| {
                if genn < chip {
                    (genn, chip)
                } else {
                    (chip, genn)
                }
            })
            .collect();
        normalized.sort_unstable();
        normalized.hash(state);
    }
}

impl State {
    // Check if this state is a goal state
    fn is_goal(&self) -> bool {
        self.items
            .iter()
            .all(|&(genn, chip)| genn == 3 && chip == 3)
    }

    fn next_states(&self) -> Vec<State> {
        let mut next_states = Vec::new();
        let current_floor = self.elevator;

        // Collect all items on the current floor
        let chips_on_floor: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|&(_, &(_, chip))| chip == current_floor)
            .map(|(idx, _)| idx)
            .collect();
        let generators_on_floor: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|&(_, &(genn, _))| genn == current_floor)
            .map(|(idx, _)| idx)
            .collect();

        let mut combinations: HashSet<(_, _)> = HashSet::new();
        chips_on_floor.iter().for_each(|chip_i| {
            combinations.insert((vec![], vec![chip_i]));
            generators_on_floor.iter().for_each(|gen_i| {
                combinations.insert((vec![gen_i], vec![chip_i]));
            });
            chips_on_floor
                .iter()
                .filter(|i| *i != chip_i)
                .for_each(|o_i| {
                    combinations.insert((vec![], vec![chip_i, o_i]));
                });
        });
        generators_on_floor.iter().for_each(|gen_i| {
            combinations.insert((vec![gen_i], vec![]));
            generators_on_floor
                .iter()
                .filter(|i| *i != gen_i)
                .for_each(|o_i| {
                    combinations.insert((vec![gen_i, o_i], vec![]));
                })
        });

        // Try moving the elevator up or down with each combination
        for &next_floor in &[current_floor.wrapping_sub(1), current_floor + 1] {
            // Ensure the next floor is within bounds
            if next_floor >= 4 {
                continue;
            }

            for (gen_is, chip_is) in &combinations {
                // Create a new set of items for the potential next state
                let mut new_items = self.items.clone();

                gen_is.iter().for_each(|gen_i| {
                    new_items[**gen_i].0 = next_floor;
                });
                chip_is.iter().for_each(|chip_i| {
                    new_items[**chip_i].1 = next_floor;
                });

                let new_state = State {
                    elevator: next_floor,
                    items: new_items,
                };

                // Add the new state if it's valid
                if new_state.is_valid() {
                    next_states.push(new_state);
                }
            }
        }

        next_states
    }

    // Check if the current state is valid
    fn is_valid(&self) -> bool {
        for floor in 0..4 {
            let mut has_generator = false;
            let mut unshielded_microchips = false;

            for &(gen_floor, chip_floor) in &self.items {
                if gen_floor == floor {
                    has_generator = true;
                }
                if chip_floor == floor && gen_floor != floor {
                    unshielded_microchips = true;
                }
            }

            // If there's an unshielded microchip and a generator, the floor is invalid
            if unshielded_microchips && has_generator {
                return false;
            }
        }
        true
    }
}

fn solve(initial_state: State) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((initial_state.clone(), 0)); // (State, Steps)
    visited.insert(initial_state);

    while let Some((current_state, steps)) = queue.pop_front() {
        // Check if we've reached the goal
        if current_state.is_goal() {
            return steps;
        }

        // Generate valid next states
        for next_state in current_state.next_states() {
            if !visited.contains(&next_state) {
                visited.insert(next_state.clone());
                queue.push_back((next_state, steps + 1));
            }
        }
    }

    unreachable!("No solution found!");
}

pub fn part_one(input: &str) -> Option<usize> {
    let state = match input {
        "test" => State {
            elevator: 0,
            items: vec![
                (1, 0), // Hydrogen (Generator, Microchip)
                (2, 0), // Lithium
            ],
        },
        _ => State {
            elevator: 0,
            items: vec![
                (0, 0), // Thulium
                (0, 1), //Plutonium
                (0, 1), //Strontium
                (2, 2), //Promethium
                (2, 2), //Ruthenium
            ],
        },
    };

    Some(solve(state))
}

pub fn part_two(input: &str) -> Option<usize> {
    let state = match input {
        "test" => State {
            elevator: 0,
            items: vec![
                (1, 0), // Hydrogen (Generator, Microchip)
                (2, 0), // Lithium
            ],
        },
        _ => State {
            elevator: 0,
            items: vec![
                (0, 0), // Thulium
                (0, 1), //Plutonium
                (0, 1), //Strontium
                (2, 2), //Promethium
                (2, 2), //Ruthenium
                (0, 0),
                (0, 0),
            ],
        },
    };

    Some(solve(state))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("test");
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55));
    }
}
