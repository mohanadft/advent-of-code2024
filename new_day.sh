#!/bin/bash

# Script to create a new Advent of Code day solution

if [ $# -eq 0 ]; then
    echo "Usage: $0 <day_number>"
    echo "Example: $0 2"
    exit 1
fi

DAY_NUM=$1
DAY_DIR="day$(printf "%02d" $DAY_NUM)"

if [ -d "$DAY_DIR" ]; then
    echo "Day $DAY_NUM already exists!"
    exit 1
fi

echo "Creating $DAY_DIR..."

# Create directory structure
mkdir -p "$DAY_DIR/src"

# Create Cargo.toml
cat > "$DAY_DIR/Cargo.toml" << EOF
[package]
name = "$DAY_DIR"
version = "0.1.0"
edition = "2021"

[dependencies]
shared = { workspace = true }
EOF

# Create main.rs
cat > "$DAY_DIR/src/main.rs" << EOF
use shared::{read_input, read_input_lines};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎄 Advent of Code 2024 - Day $DAY_NUM 🎄");
    
    let lines = read_input_lines("$DAY_DIR/input.txt")?;
    
    let part1_result = solve_part1(&lines);
    let part2_result = solve_part2(&lines);
    
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
    
    Ok(())
}

fn solve_part1(lines: &[String]) -> i32 {
    // TODO: Implement your Part 1 solution here
    0
}

fn solve_part2(lines: &[String]) -> i32 {
    // TODO: Implement your Part 2 solution here
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let test_input = vec![
            // Add your test input here
        ];
        assert_eq!(solve_part1(&test_input), 0); // Update expected result
    }

    #[test]
    fn test_solve_part2() {
        let test_input = vec![
            // Add your test input here
        ];
        assert_eq!(solve_part2(&test_input), 0); // Update expected result
    }
}
EOF

# Create empty input file
touch "$DAY_DIR/input.txt"

# Create README file
cat > "$DAY_DIR/README.md" << EOF
# Day $DAY_NUM: [Problem Title]

**Problem:** [Advent of Code 2024 - Day $DAY_NUM](https://adventofcode.com/2024/day/$DAY_NUM)

## Running

\`\`\`bash
cargo run --bin $DAY_DIR
\`\`\`

## Testing

\`\`\`bash
cargo test --bin $DAY_DIR
\`\`\`
EOF

# Add to workspace members
if ! grep -q "\"$DAY_DIR\"" Cargo.toml; then
    # Find the line with the last day and add the new one after it
    sed -i "/\"day[0-9][0-9]\"/a\\    \"$DAY_DIR\"," Cargo.toml
fi

# Create and checkout to new branch
echo "📝 Creating and checking out to branch '$DAY_DIR'..."
git checkout -b "$DAY_DIR"

echo "✅ Created $DAY_DIR successfully!"
echo "🌿 Switched to branch '$DAY_DIR'"
echo "📝 Don't forget to add your puzzle input to $DAY_DIR/input.txt"
echo "🚀 Run with: cargo run --bin $DAY_DIR"
