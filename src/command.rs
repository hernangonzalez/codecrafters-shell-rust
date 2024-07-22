use anyhow::Result;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Exit(i32),
    Echo(String),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (cmd, args) = s.split_once(' ').unwrap_or((s, ""));
        match cmd {
            "exit" => Ok(Self::Exit(args.parse()?)),
            "echo" => Ok(Self::Echo(args.to_owned())),
            e => anyhow::bail!("{e}: command not found"),
        }
    }
}
