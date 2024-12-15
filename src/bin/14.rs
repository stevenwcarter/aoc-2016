use cached::proc_macro::cached;

advent_of_code::solution!(14);

fn hasher(input: &str, is_part_2: bool) -> String {
    let mut hash = format!("{:x}", md5::compute(input));

    if is_part_2 {
        (0..2016).for_each(|_| hash = format!("{:x}", md5::compute(&hash)));
    }

    hash
}

fn has_n_consecutive_chars(s: &str, n: u8, letter: Option<char>) -> Option<char> {
    if n == 0 {
        panic!("Invalid `n` value, must be non-zero");
    }

    let mut count = 1; // Counter for consecutive matches
    let mut prev = None; // Previous character

    for c in s.chars() {
        if Some(c) == prev {
            if let Some(oc) = letter {
                if c != oc {
                    continue;
                }
            }
            count += 1;
            if count >= n {
                return Some(c);
            }
        } else {
            count = 1;
        }
        prev = Some(c);
    }

    None
}

fn five_in_next_thousand(input: &str, pos: usize, letter: Option<char>, is_part_2: bool) -> bool {
    (pos + 1..pos + 1001).any(|pos| {
        let hash = get_hash(input.to_owned(), pos, is_part_2);
        has_n_consecutive_chars(&hash, 5, letter).is_some()
    })
}

#[cached(size = 1000)]
fn get_hash(prefix: String, pos: usize, is_part_2: bool) -> String {
    let hash = format!("{prefix}{pos}");
    hasher(&hash, is_part_2)
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();
    let mut pos = 0;
    let mut inputs: Vec<usize> = vec![];
    (0..64).for_each(|_| {
        let mut found = false;
        while !found {
            let hash = get_hash(input.to_owned(), pos, false);
            if let Some(letter) = has_n_consecutive_chars(&hash, 3, None) {
                if five_in_next_thousand(input, pos, Some(letter), false) {
                    inputs.push(pos);
                    found = true;
                }
            }
            pos += 1;
        }
    });

    inputs.get(63).copied()
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();
    let mut pos = 0;
    let mut inputs: Vec<usize> = vec![];
    (0..64).for_each(|_| {
        let mut found = false;
        while !found {
            let hash = get_hash(input.to_string(), pos, true);
            if let Some(letter) = has_n_consecutive_chars(&hash, 3, None) {
                if five_in_next_thousand(input, pos, Some(letter), true) {
                    inputs.push(pos);
                    println!("{} {pos} {hash}", inputs.len());
                    found = true;
                }
            }
            pos += 1;
        }
    });

    let result = inputs.get(63).copied();

    assert!(result < Some(26617));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2_hasher() {
        let input = format!(
            "{}0",
            &advent_of_code::template::read_file("examples", DAY).trim()
        );
        let hash = hasher(&input, true);
        assert_eq!(hash, "a107ff634856bb300138cac6568c0f24");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22728));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22551));
    }
}
