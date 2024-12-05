use std::collections::VecDeque;

use hashbrown::HashMap;

advent_of_code::solution!(10);

#[derive(Debug)]
enum Target {
    Bot(usize),
    Output(usize),
}

#[derive(Debug)]
struct BotInstruction {
    low_target: Target,
    high_target: Target,
}

fn solver(input: &str, exit_early: bool) -> (Option<u32>, HashMap<usize, usize>) {
    let mut bot_instructions = HashMap::new();
    let mut bot_chips = HashMap::new();
    let mut outputs = HashMap::new();
    let mut initial_values = VecDeque::new();

    for line in input.lines() {
        if line.starts_with("value") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let value: usize = parts[1].parse().unwrap();
            let bot: usize = parts[5].parse().unwrap();
            initial_values.push_back((value, bot));
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let bot: usize = parts[1].parse().unwrap();
            let low_target = match parts[5] {
                "bot" => Target::Bot(parts[6].parse().unwrap()),
                "output" => Target::Output(parts[6].parse().unwrap()),
                _ => panic!("Unknown target type"),
            };
            let high_target = match parts[10] {
                "bot" => Target::Bot(parts[11].parse().unwrap()),
                "output" => Target::Output(parts[11].parse().unwrap()),
                _ => panic!("Unknown target type"),
            };
            bot_instructions.insert(
                bot,
                BotInstruction {
                    low_target,
                    high_target,
                },
            );
        }
    }

    for (value, bot) in initial_values {
        bot_chips.entry(bot).or_insert_with(Vec::new).push(value);
    }

    // Process bots
    let mut responsible_bot = None;
    while let Some((&bot, chips)) = bot_chips.iter_mut().find(|(_, chips)| chips.len() == 2) {
        // Sort the chips
        chips.sort_unstable();
        let low = chips[0];
        let high = chips[1];
        chips.clear();

        // Check if this bot is responsible for comparing the target values
        if low == 17 && high == 61 {
            responsible_bot = Some(bot);
            if exit_early {
                break;
            }
        }

        // Perform the bot's actions
        if let Some(instruction) = bot_instructions.get(&bot) {
            match instruction.low_target {
                Target::Bot(target_bot) => bot_chips
                    .entry(target_bot)
                    .or_insert_with(Vec::new)
                    .push(low),
                Target::Output(output) => {
                    outputs.insert(output, low);
                }
            }
            match instruction.high_target {
                Target::Bot(target_bot) => bot_chips
                    .entry(target_bot)
                    .or_insert_with(Vec::new)
                    .push(high),
                Target::Output(output) => {
                    outputs.insert(output, high);
                }
            }
        }
    }

    let mut result: Option<u32> = None;
    if let Some(bot) = responsible_bot {
        result = Some(bot as u32);
    } else {
        println!("No bot compared 61 and 17.");
    }

    (result, outputs)
}

pub fn part_one(input: &str) -> Option<u32> {
    solver(input, true).0
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut outputs = solver(input, false).1;
    let zero = *outputs.entry(0).or_default();
    let one = *outputs.entry(1).or_default();
    let two = *outputs.entry(2).or_default();

    Some((zero * one * two) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
