//! `util` contains reusable functions.
use std::io;

/// Asks for an user for input.
pub fn input(prompt: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("{}", prompt);
    stdin.read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}
