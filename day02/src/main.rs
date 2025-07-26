use shared::read_input_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎄 Advent of Code 2024 - Day 2 🎄");

    let lines = read_input_lines("day02/input.txt")?;

    let part1_result = solve_part1(&lines);
    let part2_result = solve_part2(&lines);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn solve_part1(lines: &[String]) -> i32 {
    let mut safe_reports: i32 = 0;
    for line in lines {
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(|l| l.parse::<u32>().expect("Must be number"))
            .collect::<Vec<u32>>();

        let differences: Vec<i32> = levels
            .windows(2)
            .map(|pair| pair[1] as i32 - pair[0] as i32)
            .collect();

        let is_safe: bool = differences.iter().all(|&x| x >= -3 && x <= 3 && x != 0)
            && (differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0));

        if is_safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn solve_part2(lines: &[String]) -> i32 {
    let mut safe_reports: i32 = 0;

    for line in lines {
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(|l| l.parse::<u32>().expect("Must be number"))
            .collect::<Vec<u32>>();

        if is_safe_report(&levels) {
            safe_reports += 1;
        } else {
            for i in 0..levels.len() {
                let mut excluded = levels.clone();
                excluded.remove(i);
                if is_safe_report(&excluded) {
                    safe_reports += 1;
                    break; // Found a safe report by excluding one level
                }
            }
        }
    }
    safe_reports
}

fn is_safe_report(levels: &[u32]) -> bool {
    let differences: Vec<i32> = levels
        .windows(2)
        .map(|pair| pair[1] as i32 - pair[0] as i32)
        .collect();

    differences.iter().all(|&x| x >= -3 && x <= 3 && x != 0)
        && (differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_day02() {
        let test_input = vec![
            "7 6 4 2 1".into(),
            "1 2 7 8 9".into(),
            "9 7 6 2 1".into(),
            "1 3 2 4 5".into(),
            "8 6 4 4 1".into(),
            "1 3 6 7 9".into(),
        ];
        assert_eq!(solve_part1(&test_input), 2); // Update expected result
    }

    #[test]
    fn test_solve_part2_day02() {
        let test_input = vec![
            "7 6 4 2 1".into(),
            "1 2 7 8 9".into(),
            "9 7 6 2 1".into(),
            "1 3 2 4 5".into(),
            "8 6 4 4 1".into(),
            "1 3 6 7 9".into(),
        ];
        assert_eq!(solve_part2(&test_input), 4); // Update expected result
    }
}
