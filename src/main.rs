use anyhow::Result;
use std::io::{self, Write};

fn main() -> Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;

    Ok(())
}
