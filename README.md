# Rust File Processor

## Overview
This project demonstrates core Rust programming concepts through a file processing utility that analyzes text documents. The program reads input files, calculates word frequencies using Rust's ownership model and HashMap data structure, and generates formatted output - all while showcasing Rust's memory safety guarantees.

My goal was to deepen my understanding of systems programming concepts by implementing a practical tool that highlights Rust's unique features like zero-cost abstractions and thread-safe memory management without garbage collection.

[Software Demo Video](https://your-video-link-here.com) *(to be added after recording)*

## Development Environment
- **IDE**: Visual Studio Code with Rust Analyzer extension
- **Toolchain**: Rust 1.70.0 (stable-x86_64-pc-windows-msvc)
- **Key Dependencies**:
  - Standard library features: `std::collections::HashMap`, `std::fs`
  - Cargo for package management

## Key Features
- Implements OOP patterns with `struct` and `impl`
- Demonstrates ownership/borrowing in file operations
- Uses HashMap for efficient word counting
- Handles I/O errors gracefully with Rust's Result type

## Useful Websites
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

## Code Examples
```rust
// Demonstrating ownership transfer
fn process_file(path: String) -> io::Result<()> {
    let content = fs::read_to_string(path)?; // path moved here
    // ... processing ...
}