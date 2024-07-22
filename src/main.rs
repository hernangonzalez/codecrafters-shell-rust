use anyhow::Result;
use std::{
    io::{self, Write},
    str::FromStr,
};

#[derive(Debug)]
enum Command {}

impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let msg = format!("{}: command not found", s);
        anyhow::bail!(msg)
    }
}

fn main() -> Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;

    // Parse input
    for line in input.lines() {
        let res: Result<Command> = line.parse();
        match res {
            Ok(_) => todo!(),
            Err(e) => println!("{e}"),
        }
    }

    Ok(())
}
