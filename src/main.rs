use anyhow::{Context, Result};
use std::{
    fmt::Display,
    io::{self, Write},
    process::exit,
    str::FromStr,
};

#[derive(Debug)]
enum Command {
    Exit(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let cmd = parts.next().context("missing command")?;
        match cmd {
            "exit" => Ok(Self::Exit(parts.next().unwrap_or("0").parse()?)),
            e => anyhow::bail!("{e}: command not found"),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit(value) => write!(f, "exit {value}"),
        }
    }
}

fn handle_command(c: Command) {
    match c {
        Command::Exit(code) => exit(code),
    }
}

fn main() -> Result<()> {
    loop {
        // Init prompt
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
                Ok(cmd) => handle_command(cmd),
                Err(e) => println!("{e}"),
            }
        }
    }
}
