advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut count = 0;
    lines.iter().for_each(|line| {
        let parts: Vec<u32> = line
            .split_whitespace()
            .map(|p| p.parse::<u32>().unwrap())
            .collect();

        if parts[0] + parts[1] > parts[2]
            && parts[0] + parts[2] > parts[1]
            && parts[1] + parts[2] > parts[0]
        {
            count += 1;
        }
    });

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|p| p.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let mut count = 0;
    for col in 0..3 {
        let mut pos = 0;
        let max_len = lines.len();
        while pos < max_len - 1 {
            let parts: Vec<u32> = vec![lines[pos][col], lines[pos + 1][col], lines[pos + 2][col]];
            if parts[0] + parts[1] > parts[2]
                && parts[0] + parts[2] > parts[1]
                && parts[1] + parts[2] > parts[0]
            {
                count += 1;
            }
            pos += 3;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
