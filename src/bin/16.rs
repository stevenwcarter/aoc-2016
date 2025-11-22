advent_of_code::solution!(16);

// - Call the data you have at this point "a".
// - Make a copy of "a"; call this copy "b".
// - Reverse the order of the characters in "b".
// - In "b", replace all instances of 0 with 1 and all 1s with 0.
// - The resulting data is "a", then a single 0, then "b".

fn invert_and_reverse_binary_string(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => c, // Keep other characters as they are
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}

pub fn add_random(length_needed: usize, a: &str) -> String {
    if a.len() < length_needed {
        let b = invert_and_reverse_binary_string(a);

        add_random(length_needed, &format!("{a}0{b}"))
    } else {
        a[0..length_needed].to_owned()
    }
}

pub fn checksum_step(input: &str, first: bool) -> String {
    if first || input.len().is_multiple_of(2) {
        let input = input
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|c| if c[0] != c[1] { "0" } else { "1" })
            .collect::<String>();
        checksum_step(&input, false)
    } else {
        input.to_owned()
    }
}

pub fn compute_checksum(input: &str) -> String {
    checksum_step(input, true)
}

pub fn part_one(input: &str) -> Option<String> {
    let disk_size = 272;
    let randomized = add_random(disk_size, input.trim_end());
    Some(compute_checksum(&randomized))
}

pub fn part_two(input: &str) -> Option<String> {
    let disk_size = 35651584;
    let randomized = add_random(disk_size, input.trim_end());
    Some(compute_checksum(&randomized))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_random() {
        assert_eq!(
            add_random(25, "111100001010"),
            "1111000010100101011110000".to_string()
        );
    }

    #[test]
    fn test_compute_checksum() {
        assert_eq!(compute_checksum("110010110100"), "100")
    }

    #[test]
    fn test_part_one() {
        let randomized = add_random(20, "10000");
        assert_eq!(randomized, "10000011110010000111".to_owned());
        let result = compute_checksum(&randomized);
        assert_eq!(result, "01100".to_owned());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("01110011011000110".to_owned()));
    }
}
