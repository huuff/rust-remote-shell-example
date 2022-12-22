use std::str::SplitWhitespace;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Echo(Echo),
    Ls,
    Cd(Cd),
    Cat(Cat),
    Exit,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Echo {
    pub message: String,
}

impl Echo {
    fn new(message: String) -> Self {
        Echo { message }
    }
}

pub struct Ls { }

#[derive(Debug, Eq, PartialEq)]
pub struct Cd {
    pub target_directory: String,
}

impl Cd {
    fn new(target_directory: String) -> Self {
        Cd { target_directory }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cat {
    pub file: String,
}

impl Cat {
    fn new(file: String) -> Self {
        Cat { file }
    }
}

pub struct Exit {}


enum ExpectedArguments {
    None,
    Exactly(usize),
    Any,
}

fn parse_arguments(args: SplitWhitespace, expected_arguments: ExpectedArguments) -> Result<Vec<String>, BadCommandError> {
    let args = args.map(str::to_string).collect::<Vec<String>>();
    match expected_arguments {
        ExpectedArguments::None => {
            if args.len() == 0 {
                Ok(Vec::new())
            } else {
                Err(BadCommandError::wrong_arg_number(0, args.len()))
            }
        },
        ExpectedArguments::Any => {
            Ok(args)
        }
        ExpectedArguments::Exactly(n) => {
            if args.len() == n {
                Ok(args)
            } else {
                Err(BadCommandError::wrong_arg_number(n, args.len()))
            }
        }
    }
}

#[derive(Debug)]
pub struct BadCommandError {
    msg: String,
}

impl BadCommandError {
    pub fn from_string(msg: String) -> Self {
        BadCommandError { msg }
    }
    pub fn from_str(msg: &str) -> Self {
        BadCommandError::from_string(String::from(msg))
    }
    pub fn wrong_arg_number(expected: usize, got: usize) -> Self {
        BadCommandError::from_string(format!("Expected {} arguments but received {}", expected, got))
    }
}

impl Command {
    // TODO: Clean it up with the below function
    // TODO: Test
    pub fn parse(command_string: &str) -> Result<Self, BadCommandError> {
        let mut command_parts = command_string.split_whitespace();
        if let Some(command) = command_parts.next() {
            match command {
                "echo" => {
                    Ok(Command::Echo(Echo::new(command_parts.join(" "))))
                },
                "ls" => {
                    Ok(Command::Ls)
                },
                "cd" => {
                    if let Some(argument) = command_parts.next() {
                        Ok(Command::Cd(Cd::new(String::from(argument))))
                    } else {
                        Err(BadCommandError::from_str("Missing argument to cd"))
                    }
                },
                "cat" => {
                    if let Some(argument) = command_parts.next() {
                        Ok(Command::Cat(Cat::new(String::from(argument))))
                    } else {
                        Err(BadCommandError::from_str("Missing argument to cat"))
                    }
                },
                "exit" => {
                    Ok(Command::Exit)
                },
                _ => {
                    Err(BadCommandError::from_string(format!("Unknown command {}", command)))
                }
            }
        } else {
            Err(BadCommandError::from_str("Missing command"))
        }
    }

    fn expected_arguments(&self) -> ExpectedArguments {
        match self {
            Command::Echo(_) => ExpectedArguments::Any,
            Command::Ls => ExpectedArguments::None,
            Command::Cd(_) => ExpectedArguments::Exactly(1),
            Command::Cat(_) => ExpectedArguments::Exactly(1),
            Command::Exit => ExpectedArguments::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_echo() {
        let result = Command::parse("echo some sample text");
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::Echo(Echo::new(String::from("some sample text"))));
    }

    #[test]
    fn parses_cd() {
        let result = Command::parse("cd dir");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::Cd(Cd::new(String::from("dir"))));
    }
}
