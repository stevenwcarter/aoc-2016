use hashbrown::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<String> {
    let input = input.trim();
    let mut pos = 0;

    let mut interesting: Vec<char> = Vec::with_capacity(8);

    while interesting.len() < 8 {
        let check = format!("{input}{pos}");
        let hash = md5::compute(&check);
        if hash.0.first().unwrap() == &0 && hash.0.get(1).unwrap() == &0 {
            let result = format!("{:x}", hash);
            if result.starts_with("00000") {
                interesting.push(result.chars().nth(5).unwrap());
            }
        }
        pos += 1;
    }

    Some(interesting.iter().collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input.trim();
    let mut pos = 0;

    let mut solved: HashMap<u8, bool> = HashMap::new();
    let mut interesting: Vec<char> = vec!['0'; 8];

    while solved.len() < 8 {
        let check = format!("{input}{pos}");
        let hash = md5::compute(&check);
        if hash.0.first().unwrap() == &0 && hash.0.get(1).unwrap() == &0 {
            let result = format!("{:x}", hash);
            if result.starts_with("00000") {
                let digit_to_check = result
                    .chars()
                    .nth(5)
                    .unwrap()
                    .to_string()
                    .parse::<u8>()
                    .unwrap_or(9);
                let char_value = result.chars().nth(6).unwrap();
                if digit_to_check < 8 && !solved.get(&digit_to_check).unwrap_or(&false) {
                    let position = interesting.get_mut(digit_to_check as usize).unwrap();
                    solved.entry(digit_to_check).or_insert(true);
                    *position = char_value;
                }
            }
        }
        pos += 1;
    }

    Some(interesting.iter().collect::<String>())
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
