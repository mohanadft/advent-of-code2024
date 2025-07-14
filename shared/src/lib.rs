use std::fs;
use std::path::Path;

/// Reads the content of a file and returns it as a String.
///
/// # Arguments
/// * `file_path` - A string slice that holds the path to the file
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - The file content as a string or an error
///
/// # Example
/// ```no_run
/// use shared::read_input;
///
/// let content = read_input("input.txt").expect("Failed to read input file");
/// println!("File content: {}", content);
/// ```
pub fn read_input(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("File '{}' does not exist", file_path).into());
    }

    let content = fs::read_to_string(path)?;
    Ok(content.trim_end().to_string()) // Remove trailing newline/whitespace
}

/// Reads the content of a file and returns it as a vector of lines.
///
/// # Arguments
/// * `file_path` - A string slice that holds the path to the file
///
/// # Returns
/// * `Result<Vec<String>, Box<dyn std::error::Error>>` - The file content as lines or an error
///
/// # Example
/// ```no_run
/// use shared::read_input_lines;
///
/// let lines = read_input_lines("input.txt").expect("Failed to read input file");
/// for line in lines {
///     println!("Line: {}", line);
/// }
/// ```
pub fn read_input_lines(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = read_input(file_path)?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_input() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello\nWorld\n").unwrap();

        let content = read_input(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(content, "Hello\nWorld");
    }

    #[test]
    fn test_read_input_lines() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Line 1\nLine 2\nLine 3").unwrap();

        let lines = read_input_lines(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_input("nonexistent_file.txt");
        assert!(result.is_err());
    }
}
