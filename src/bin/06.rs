use hashbrown::HashMap;

advent_of_code::solution!(6);

pub fn get_frequencies(input: &str) -> (usize, HashMap<usize, HashMap<char, u32>>) {
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.first().unwrap().len();
    let mut frequencies: HashMap<usize, HashMap<char, u32>> = HashMap::new();

    lines.iter().for_each(|l| {
        l.chars().enumerate().for_each(|(idx, char)| {
            let pos = frequencies.entry(idx).or_default();
            *pos.entry(char).or_insert(0) += 1;
        });
    });

    (len, frequencies)
}

pub fn part_one(input: &str) -> Option<String> {
    let (len, frequencies) = get_frequencies(input);
    Some(
        (0..len)
            .map(|idx| {
                let pos = frequencies.get(&idx).unwrap();
                pos.iter()
                    .max_by(|a, b| a.1.cmp(b.1))
                    .map(|(k, _v)| k)
                    .unwrap()
            })
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (len, frequencies) = get_frequencies(input);
    Some(
        (0..len)
            .map(|idx| {
                let pos = frequencies.get(&idx).unwrap();
                pos.iter()
                    .max_by(|a, b| b.1.cmp(a.1))
                    .map(|(k, _v)| k)
                    .unwrap()
            })
            .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("easter".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("advent".to_string()));
    }
}
