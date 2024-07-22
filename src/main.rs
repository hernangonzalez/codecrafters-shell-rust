mod command;

use anyhow::Result;
use command::Command;
use std::{
    io::{self, Write},
    process::exit,
};

fn handle_command(c: Command) {
    match c {
        Command::Exit(code) => exit(code),
        Command::Echo(msg) => println!("{msg}"),
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
        let cmds = input.lines().map(|l| l.parse::<Command>());

        // Handle commands
        cmds.for_each(|cmd| match cmd {
            Ok(cmd) => handle_command(cmd),
            Err(e) => println!("{e}"),
        });
    }
}
