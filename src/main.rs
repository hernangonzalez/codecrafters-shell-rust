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
        Command::Type(cmd) => handle_type(&cmd),
        Command::Exec(_path) => todo!(),
    }
}

fn handle_type(input: &str) {
    let Ok(cmd) = command::resolve(input) else {
        println!("{input}: not found");
        return;
    };

    match cmd {
        c if c.is_builtin() => println!("{input} is a shell builtin"),
        Command::Exec(p) => println!("{input} is {}", p.display()),
        _ => unimplemented!(),
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
        let cmds = input.lines().map(command::resolve);

        // Handle commands
        cmds.for_each(|cmd| match cmd {
            Ok(cmd) => handle_command(cmd),
            Err(e) => println!("{e}"),
        });
    }
}
