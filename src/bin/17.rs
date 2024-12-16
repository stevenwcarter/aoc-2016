use std::collections::VecDeque;

advent_of_code::solution!(17);

// https://www.geeksforgeeks.org/breadth-first-search-or-bfs-for-a-graph/#bfs-from-a-given-source

fn is_open(c: char) -> bool {
    matches!(c, 'b'..='f')
}

fn get_doors(passcode: &str, path: &str) -> [bool; 4] {
    let hash = format!("{:x}", md5::compute(format!("{}{}", passcode, path)));
    [
        is_open(hash.chars().nth(0).unwrap()), // Up
        is_open(hash.chars().nth(1).unwrap()), // Down
        is_open(hash.chars().nth(2).unwrap()), // Left
        is_open(hash.chars().nth(3).unwrap()), // Right
    ]
}

fn bfs(passcode: &str) -> Option<String> {
    let start = (0, 0, String::new());
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((x, y, path)) = queue.pop_front() {
        if x == 3 && y == 3 {
            return Some(path);
        }

        let doors = get_doors(passcode, &path);

        if doors[0] && y > 0 {
            queue.push_back((x, y - 1, format!("{}U", path)));
        }
        if doors[1] && y < 3 {
            queue.push_back((x, y + 1, format!("{}D", path)));
        }
        if doors[2] && x > 0 {
            queue.push_back((x - 1, y, format!("{}L", path)));
        }
        if doors[3] && x < 3 {
            queue.push_back((x + 1, y, format!("{}R", path)));
        }
    }

    None
}

fn dfs(passcode: &str, x: usize, y: usize, path: &str) -> Option<String> {
    if x == 3 && y == 3 {
        // If we reached the vault, return the path
        return Some(path.to_string());
    }

    let doors = get_doors(passcode, path);
    let mut longest_path = None;

    if doors[0] && y > 0 {
        if let Some(new_path) = dfs(passcode, x, y - 1, &format!("{}U", path)) {
            longest_path = max_path(longest_path, new_path);
        }
    }
    if doors[1] && y < 3 {
        if let Some(new_path) = dfs(passcode, x, y + 1, &format!("{}D", path)) {
            longest_path = max_path(longest_path, new_path);
        }
    }
    if doors[2] && x > 0 {
        if let Some(new_path) = dfs(passcode, x - 1, y, &format!("{}L", path)) {
            longest_path = max_path(longest_path, new_path);
        }
    }
    if doors[3] && x < 3 {
        if let Some(new_path) = dfs(passcode, x + 1, y, &format!("{}R", path)) {
            longest_path = max_path(longest_path, new_path);
        }
    }

    longest_path
}

fn max_path(a: Option<String>, b: String) -> Option<String> {
    match a {
        None => Some(b),
        Some(existing) => {
            if b.len() > existing.len() {
                Some(b)
            } else {
                Some(existing)
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    bfs(&input[0..input.len() - 1])
}

pub fn part_two(input: &str) -> Option<usize> {
    let longest_path = dfs(&input[0..input.len() - 1], 0, 0, "");
    longest_path.map(|longest_path| longest_path.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("DDRRRD".to_string()));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("DDUDRLRRUDRD".to_string()));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string()));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(370));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(492));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(830));
    }
}
