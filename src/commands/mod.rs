pub mod help;
pub mod ls;
pub mod cd;
pub mod cat;

use crate::filesystem::FileSystem;
use crate::session::Session;

pub enum Command {
    Help,
    Ls,
    Cd(String),
    Cat(String),
    Unknown,
}

pub fn parse(input: &str) -> Command {
    let mut parts = input.trim().split_whitespace();

    match parts.next() {
        Some("help") => Command::Help,
        Some("ls") => Command::Ls,

        Some("cd") => {
            let dir = parts.next().unwrap_or("abracadabra");
            Command::Cd(dir.to_string())
        }

        Some("cat") => {
            let file = parts.next().unwrap_or("abracadabra");
            Command::Cat(file.to_string())
        }

        _ => Command::Unknown,
    }
}

pub fn execute(
    command: Command,
    session: &mut Session,
    fs: &FileSystem,
) -> Result<String,String> {
    match command {
        Command::Help => help::execute(),
        Command::Ls => ls::execute(session, fs),
        Command::Cd(path) => cd::execute(session, fs, &path),
        Command::Cat(file) => cat::execute(session, fs, &file),
        Command::Unknown => {
            Ok("No such command, jennie ready to turn you full dumbass\n".to_string())
        }
    }
}
