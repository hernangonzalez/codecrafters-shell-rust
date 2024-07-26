use anyhow::{Ok, Result};
use std::{path::PathBuf, str::FromStr};

pub fn resolve(input: &str) -> Result<Command> {
    input.parse()
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Echo(String),
    Exec(PathBuf),
    Exit(i32),
    Type(String),
}

impl Command {
    pub fn is_builtin(&self) -> bool {
        matches!(self, Self::Echo(_) | Self::Exit(_) | Self::Type(_))
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (cmd, args) = s.split_once(' ').unwrap_or((s, ""));
        match cmd {
            "exit" => Ok(Self::Exit(args.parse().unwrap_or_default())),
            "echo" => Ok(Self::Echo(args.to_owned())),
            "type" => Ok(Self::Type(args.to_owned())),
            other => find_path(other).map(Command::Exec),
        }
    }
}

fn find_path(cmd: &str) -> Result<PathBuf> {
    let path = std::env::var("PATH")?;
    let paths = std::env::split_paths(&path);
    let path = paths
        .map(|mut p| {
            p.push(cmd);
            p
        })
        .find(|p| p.exists());
    match path {
        Some(path) => Ok(path),
        None => anyhow::bail!("{cmd}: command not found"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let cmd: Command = "exit 0".parse().unwrap();
        assert_eq!(cmd, Command::Exit(0));

        let cmd: Command = "echo hernan.rs".parse().unwrap();
        assert_eq!(cmd, Command::Echo("hernan.rs".to_string()));

        let cmd: Command = "type type".parse().unwrap();
        assert_eq!(cmd, Command::Type("type".to_string()));

        let res: Result<Command> = "adf asdf".parse();
        assert!(res.is_err());
    }

    #[test]
    fn test_builtin() {
        assert!(Command::Echo("hernan".into()).is_builtin());
        assert!(Command::Exit(0).is_builtin());
        assert!(Command::Type("type".into()).is_builtin());
        assert!(!Command::Exec("/bin/cat".into()).is_builtin());
    }

    #[test]
    fn test_find() {
        let path = find_path("ls").unwrap();
        assert_eq!(path.display().to_string(), "/bin/ls");
    }
}
