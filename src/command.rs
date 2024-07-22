use anyhow::{Ok, Result};
use std::str::FromStr;

pub fn is_builtin(cmd: &str) -> bool {
    ["exit", "echo", "type"].contains(&cmd)
}

#[derive(Debug)]
pub enum Command {
    Echo(String),
    Exit(i32),
    Type(String),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (cmd, args) = s.split_once(' ').unwrap_or((s, ""));
        match cmd {
            "exit" => Ok(Self::Exit(args.parse()?)),
            "echo" => Ok(Self::Echo(args.to_owned())),
            "type" => Ok(Self::Type(args.to_owned())),
            e => anyhow::bail!("{e}: command not found"),
        }
    }
}
