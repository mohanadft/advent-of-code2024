# Advent of Code 2024 🎄

This is a Rust workspace for solving Advent of Code 2024 problems.

## Project Structure

```
advent-of-code24/
├── Cargo.toml          # Workspace configuration
├── shared/             # Shared utilities library
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs      # Helper functions (like read_input)
├── day01/              # Day 1 solution
│   ├── Cargo.toml
│   ├── README.md       # Problem link and instructions
│   ├── src/
│   │   └── main.rs
│   └── input.txt       # Input file for day 1
├── day02/              # Day 2 solution (create as needed)
│   ├── Cargo.toml
│   ├── README.md       # Problem link and instructions
│   ├── src/
│   │   └── main.rs
│   └── input.txt       # Input file for day 2
└── ...
```

## Shared Helper Functions

The `shared` crate provides utility functions that can be used across all day solutions:

- `read_input(file_path: &str)` - Reads a file and returns its content as a String
- `read_input_lines(file_path: &str)` - Reads a file and returns its content as a Vec<String> of lines

## Usage

### Running a specific day

```bash
# Run day 1
cargo run --bin day01

# Run with release optimizations
cargo run --release --bin day01
```

### Running tests

```bash
# Run all tests
cargo test

# Run tests for a specific day
cargo test --bin day01

# Run tests for the shared library
cargo test --package shared
```

### Adding a new day

**Quick method (recommended):**

Use the provided script to automatically create a new day:

```bash
./new_day.sh 3  # Creates day03 with all necessary files
```

This script will:

- Create the directory structure (`day03/src/`)
- Generate `Cargo.toml` with proper dependencies
- Create `main.rs` with template code
- Create `README.md` with the Advent of Code problem link
- Add the new day to the workspace `Cargo.toml`
- Create an empty `input.txt` file
- Create and switch to a new branch named `day03`

**Manual method:**

1. Add the new day to the workspace members in the root `Cargo.toml`:

   ```toml
   members = [
       "shared",
       "day01",
       "day02",  # Add this line
   ]
   ```

2. Create the new day directory and files:

   ```bash
   mkdir day02
   ```

3. Create `day02/Cargo.toml`:

   ```toml
   [package]
   name = "day02"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   shared = { workspace = true }
   ```

4. Create `day02/src/main.rs` (you can copy from day01 as a template)

5. Create `day02/input.txt` with your puzzle input

## Example Usage of Helper Functions

```rust
use shared::{read_input, read_input_lines};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read entire file as string
    let content = read_input("day01/input.txt")?;

    // Read file as lines
    let lines = read_input_lines("day01/input.txt")?;

    // Your solution logic here...

    Ok(())
}
```

## Tips

- Place your puzzle input in `dayXX/input.txt`
- Use the shared helper functions to read input consistently
- Write tests for your solutions using sample data from the problem description
- Use `cargo run --release` for better performance on computationally intensive problems
