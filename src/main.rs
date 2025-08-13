//! Rust File Processor
//! Demonstrates file I/O, ownership, and data structures
//! Module 3 Submission for CSE 310 - Applied Programming

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::collections::HashMap;

/// File processor struct demonstrating OOP principles
/// Contains paths for input/output files
struct FileProcessor {
    input_path: String,
    output_path: String,
    word_counts: HashMap<String, u32>, // Data structure requirement
}

impl FileProcessor {
    /// Creates a new FileProcessor instance
    /// # Arguments
    /// * `input` - Path to input file
    /// * `output` - Path to output file
    pub fn new(input: &str, output: &str) -> Self {
        FileProcessor {
            input_path: input.to_string(),
            output_path: output.to_string(),
            word_counts: HashMap::new(),
        }
    }

    /// Main processing workflow
    /// # Returns
    /// `io::Result<()>` - Success or error state
    pub fn process(&mut self) -> io::Result<()> {
        self.read_file()?;
        self.analyze_content();
        self.write_results()
    }

    /// Reads content from input file
    /// # Returns
    /// `io::Result<String>` - File contents or error
    fn read_file(&self) -> io::Result<String> {
        let mut file = File::open(&self.input_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// Analyzes file content to count word frequencies
    /// # Arguments
    /// * `content` - String content to analyze
    fn analyze_content(&mut self) {
        if let Ok(content) = self.read_file() {
            for line in content.lines() { // Loops requirement
                self.process_line(line);
            }
        }
    }

    /// Processes individual line for word counting
    /// # Arguments
    /// * `line` - Line of text to process
    fn process_line(&mut self, line: &str) {
        for word in line.split_whitespace() {
            let cleaned_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
            if !cleaned_word.is_empty() { // Conditionals requirement
                *self.word_counts.entry(cleaned_word).or_insert(0) += 1;
            }
        }
    }

    /// Writes analysis results to output file
    /// # Returns
    /// `io::Result<()>` - Success or error state
    fn write_results(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.output_path)?;

        for (word, count) in &self.word_counts {
            writeln!(file, "{}: {}", word, count)?; // Expressions requirement
        }
        Ok(())
    }

    /// Gets most frequent word (demonstrates HashMap usage)
    /// # Returns
    /// `Option<(&String, &u32)>` - Most frequent word and count if exists
    pub fn get_most_frequent(&self) -> Option<(&String, &u32)> {
        self.word_counts.iter().max_by_key(|entry| entry.1)
    }
}

fn main() {
    // Initialize processor (Variables requirement)
    let mut processor = FileProcessor::new(
        "inputs/sample.txt",
        "outputs/word_counts.txt"
    );

    // Execute processing workflow
    match processor.process() {
        Ok(_) => {
            println!("File processed successfully!");
            
            // Demonstrate additional functionality
            if let Some((word, count)) = processor.get_most_frequent() {
                println!("Most frequent word: '{}' ({} occurrences)", word, count);
            }
        }
        Err(e) => eprintln!("Error: {}", e), // Error handling
    }
}