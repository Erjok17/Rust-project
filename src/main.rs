use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

fn read_file(path: &Path) -> io::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(String::from).collect())
}

fn write_file(path: &Path, lines: &[String]) -> io::Result<()> {
    // Create output directory if it doesn't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    
    file.write_all(lines.join("\n").as_bytes())
}

fn main() {
    let input_path = Path::new("inputs/sample.txt");
    let output_path = Path::new("outputs/processed.txt");

    match read_file(input_path) {
        Ok(lines) => {
            println!("Original content ({} lines):", lines.len());
            for (i, line) in lines.iter().enumerate() {
                println!("{}. {}", i + 1, line);
            }

            let processed: Vec<_> = lines.iter()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_uppercase())
                .collect();

            if let Err(e) = write_file(output_path, &processed) {
                eprintln!("Error writing file: {}", e);
            } else {
                println!("\nSuccessfully wrote to {}", output_path.display());
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}