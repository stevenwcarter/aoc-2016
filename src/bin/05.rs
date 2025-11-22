use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc::{Receiver, channel},
    },
    thread,
};

use md5::Digest;

advent_of_code::solution!(5);

const TARGET_LEN: usize = 8;
const NUM_CORES: usize = 14;

#[inline(always)]
fn starts_with_zeroes(bytes: &Digest) -> Option<(char, char)> {
    let skip = bytes[0] == 0 && bytes[1] == 0 && (bytes[2] & 0xF0) == 0;
    if !skip {
        return None;
    }

    let fifth_hex = bytes[2] & 0x0F;
    let sixth_hex = bytes[3] >> 4;

    let fifth_char = std::char::from_digit(fifth_hex as u32, 16).unwrap();
    let sixth_char = std::char::from_digit(sixth_hex as u32, 16).unwrap();
    Some((fifth_char, sixth_char))
}

pub fn part_one(input: &str) -> Option<String> {
    let input = input.trim().to_owned();
    let num_threads = NUM_CORES.max(1); // Ensure at least 1 thread

    let next_pos = Arc::new(AtomicUsize::new(0));

    let mut collected_count = 0;

    let (tx, rx) = channel();

    for _ in 0..num_threads {
        let input_clone = input.clone();
        let tx_clone = tx.clone();
        let next_pos_ref = next_pos.clone();

        thread::spawn(move || {
            let mut prefix_buffer = String::with_capacity(input_clone.len() + 20);

            loop {
                let pos = next_pos_ref.fetch_add(1, Ordering::Relaxed);

                if pos > 10_000_000 {
                    break;
                }

                prefix_buffer.clear();
                prefix_buffer.push_str(&input_clone);
                prefix_buffer.push_str(&pos.to_string());

                let hash = md5::compute(&prefix_buffer);

                if let Some((digit, _)) = starts_with_zeroes(&hash)
                    && tx_clone.send((pos, digit)).is_err()
                {
                    break;
                }
            }
        });
    }

    drop(tx);

    let mut sorted_results = Vec::with_capacity(TARGET_LEN);

    for (pos, digit) in rx.iter() {
        if collected_count < TARGET_LEN {
            sorted_results.push((pos, digit));
            collected_count += 1;

            if collected_count == TARGET_LEN {
                break;
            }
        }
    }

    sorted_results.sort_by_key(|(pos, _)| *pos);

    let final_password: String = sorted_results.into_iter().map(|(_, digit)| digit).collect();

    if final_password.len() == TARGET_LEN {
        Some(final_password)
    } else {
        None
    }
}

struct PartTwoResult {
    position: u8,
    value: char,
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input.trim().to_owned();
    let num_threads = NUM_CORES.max(1);

    let (tx, rx): (
        std::sync::mpsc::Sender<PartTwoResult>,
        Receiver<PartTwoResult>,
    ) = channel();

    let mut interesting: [Option<char>; TARGET_LEN] = [None; TARGET_LEN];
    let mut solved_count = 0;

    for i in 0..num_threads {
        let input_clone = input.clone();
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let mut prefix_buffer = String::with_capacity(input_clone.len() + 20);

            // Each thread starts at an offset and increments by the number of threads
            let mut pos = i;

            while pos < usize::MAX - num_threads {
                prefix_buffer.clear();
                prefix_buffer.push_str(&input_clone);
                prefix_buffer.push_str(&pos.to_string());

                let hash = md5::compute(&prefix_buffer);

                if let Some((digit_char, char_value)) = starts_with_zeroes(&hash)
                    && let Some(digit) = digit_char.to_digit(10)
                {
                    let position = digit as u8;

                    if position < TARGET_LEN as u8 {
                        let result = PartTwoResult {
                            position,
                            value: char_value,
                        };
                        if tx_clone.send(result).is_err() {
                            break;
                        }
                    }
                }

                pos += num_threads;
            }
        });
    }

    drop(tx);

    for result in rx.iter() {
        let pos_usize = result.position as usize;

        if interesting[pos_usize].is_none() {
            interesting[pos_usize] = Some(result.value);
            solved_count += 1;

            if solved_count == TARGET_LEN {
                break; // All positions are solved
            }
        }
    }

    let final_password: String = interesting.iter().map(|o| o.unwrap_or('_')).collect();

    if solved_count == TARGET_LEN {
        Some(final_password)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("18f47a30".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("05ace8e3".to_string()));
    }
}
