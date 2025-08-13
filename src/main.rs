use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

// Demonstrates all requirements:
// - Variables (mutable/immutable)
// - Functions with ownership/borrowing
// - Error handling (Result, match)
// - Conditionals (if let)
// - Vec (extra requirement)
// - Path handling for cross-platform compatibility

fn read_file(path: &Path) -> io::Result<Vec<String>> {
    // Immutable file handle
    let mut file = File::open(path)?;
    
    // Mutable String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Return Vec<&str> (borrowed slices)
    Ok(contents.lines().map(String::from).collect())
}

fn write_file(path: &Path, lines: &[String]) -> io::Result<()> {
    // Mutable file handle
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    
    // Join lines with newlines
    let output = lines.join("\n");
    file.write_all(output.as_bytes())
}

fn process_content(lines: Vec<String>) -> Vec<String> {
    // Demonstrate:
    // - Iterators (loops)
    // - Filter (conditionals)
    // - Vec manipulation
    lines.into_iter()
        .filter(|line| !line.trim().is_empty()) // Remove empty lines
        .map(|line| line.to_uppercase()) // Transform data
        .collect()
}

fn main() {
    // Immutable path variables (cross-platform)
    let input_path = Path::new("inputs/sample.txt");
    let output_path = Path::new("outputs/processed.txt");
    
    // 1. Read file (with error handling)
    let lines = match read_file(input_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    
    println!("Original content ({} lines):", lines.len());
    for (i, line) in lines.iter().enumerate() {
        println!("{}. {}", i + 1, line);
    }
    
    // 2. Process content
    let processed = process_content(lines);
    
    // 3. Write file (with error handling)
    if let Err(e) = write_file(output_path, &processed) {
        eprintln!("Error writing file: {}", e);
        return;
    }
    
    println!("\nSuccessfully wrote {} lines to {}", 
        processed.len(), 
        output_path.display());
    
    // Bonus: Show first 3 lines of output
    if let Ok(new_content) = read_file(output_path) {
        println!("\nFirst 3 lines of output:");
        for line in new_content.iter().take(3) {
            println!("> {}", line);
        }
    }
}