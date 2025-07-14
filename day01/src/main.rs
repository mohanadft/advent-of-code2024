use shared::{read_input, read_input_lines};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎄 Advent of Code 2024 - Day 1 🎄");

    // Example: Read the entire input as a string
    let input_content = read_input("day01/input.txt")?;
    println!("Input content length: {} characters", input_content.len());

    let lines = read_input_lines("day01/input.txt")?;

    let part1_result = solve_part1(&lines);
    let part2_result = solve_part2(&lines);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn solve_part1(lines: &[String]) -> i32 {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let left_value = parts[0].parse::<u32>().unwrap_or_default();
            let right_value = parts[1].parse::<u32>().unwrap_or_default();

            left_list.push(left_value);
            right_list.push(right_value);
        }
    }

    left_list.sort();
    right_list.sort();

    // return the distance between the two lists
    let distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (*l as i32 - *r as i32).abs())
        .sum();

    distance
}

fn solve_part2(lines: &[String]) -> i32 {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let left_value = parts[0].parse::<u32>().unwrap_or_default();
            let right_value = parts[1].parse::<u32>().unwrap_or_default();

            left_list.push(left_value);
            right_list.push(right_value);
        }
    }

    let mut occurrences = std::collections::HashMap::new();
    for &right_value in &right_list {
        *occurrences.entry(right_value).or_insert(0) += 1;
    }
    let sum: i32 = left_list
        .iter()
        .map(|&left_value| {
            occurrences
                .get(&left_value)
                .map_or(0, |&count| left_value as i32 * count as i32)
        })
        .sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_day01() {
        // Use a relative path that works from the workspace root when tests run
        let test_input = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];
        assert_eq!(solve_part1(&test_input), 11);
    }

    #[test]
    fn test_solve_part2_day01() {
        let test_input = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];
        assert_eq!(solve_part2(&test_input), 31); // 3 + 5 = 8
    }
}
