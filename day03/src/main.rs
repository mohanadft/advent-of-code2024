use regex::Regex;
use shared::read_input_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎄 Advent of Code 2024 - Day 3 🎄");

    let lines = read_input_lines("day03/input.txt")?;

    println!("Input contains {} lines", lines.len());

    let part1_result = solve_part1(&lines);
    let part2_result = solve_part2(&lines);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn solve_part1(lines: &[String]) -> usize {
    let mut result = 0;

    for line in lines {
        let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

        for cap in regex.captures_iter(&line) {
            // get the number inside the parentheses
            let numbers: Vec<usize> = cap[0]
                .trim_start_matches("mul(")
                .trim_end_matches(")")
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();

            result += numbers.iter().product::<usize>();
        }
    }

    result
}

fn solve_part2(lines: &[String]) -> usize {
    let mut result = 0;

    let mut enabled = true;

    for line in lines {
        let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\))").unwrap();

        for cap in regex.captures_iter(&line) {
            let numbers: Vec<usize> = if cap[0].starts_with('m') {
                cap[0]
                    .trim_start_matches("mul(")
                    .trim_end_matches(")")
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect()
            } else {
                if &cap[0] == "do()" {
                    enabled = true;
                    continue;
                } else if &cap[0] == "don\'t()" {
                    enabled = false;
                    continue;
                } else {
                    continue; // skip if not a valid mul or do/don't
                }
            };

            if enabled {
                result += numbers.iter().product::<usize>();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_day03() {
        let test_input = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];
        assert_eq!(solve_part1(&test_input), 161); // Update expected result
    }

    #[test]
    fn test_solve_part2_day03() {
        let test_input = vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ];
        assert_eq!(solve_part2(&test_input), 48); // Update expected result
    }
}
