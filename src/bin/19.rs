advent_of_code::solution!(19);

use std::collections::VecDeque;

#[derive(Debug)]
struct Person {
    id: u32,
    presents: u32,
}

fn find_winner(num_people: u32) -> u32 {
    let mut people: VecDeque<Person> = (1..=num_people)
        .map(|id| Person { id, presents: 1 })
        .collect();

    while people.len() > 1 {
        // find the next person with presents
        if let Some(mut current) = people.pop_front() {
            // steal from the next person (and remove them)
            if let Some(next) = people.pop_front() {
                current.presents += next.presents;
            }
            people.push_back(current);
        }
    }

    // the last person standing is the winner
    people.front().unwrap().id
}

fn find_winner_part2(num_people: u32) -> u32 {
    let mut left: VecDeque<u32> = VecDeque::with_capacity((num_people / 2 + 1) as usize);
    let mut right: VecDeque<u32> = VecDeque::with_capacity((num_people / 2 + 1) as usize);

    (1..(num_people / 2) + 1).for_each(|i| {
        left.push_back(i);
    });
    ((num_people / 2) + 1..num_people + 1).for_each(|i| {
        right.push_front(i);
    });

    while !left.is_empty() && !right.is_empty() {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_back();
        }

        right.push_front(left.pop_front().unwrap());
        left.push_back(right.pop_back().unwrap());
    }

    if !left.is_empty() { left[0] } else { right[0] }
}

pub fn part_one(input: &str) -> Option<u32> {
    let starting_elf_count = &input.trim_end().parse::<u32>().unwrap();

    Some(find_winner(*starting_elf_count))
}

pub fn part_two(input: &str) -> Option<u32> {
    let starting_elf_count = &input.trim_end().parse::<u32>().unwrap();

    // (5..100).for_each(|s| {
    //     println!("{: >3}: {}", s, find_winner_part2(s));
    // });

    // None
    Some(find_winner_part2(*starting_elf_count))
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
