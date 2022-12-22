use std::str::SplitWhitespace;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Echo(Echo),
    Ls(Ls),
    Cd(Cd),
    Cat(Cat),
    Exit(Exit),
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

#[derive(Debug, Eq, PartialEq)]
pub struct Ls { }

impl Ls {
    pub fn new() -> Self {
        Ls {}
    }
}

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

#[derive(Debug, Eq, PartialEq)]
pub struct Exit {}

impl Exit {
    pub fn new() -> Self {
        Exit {}
    }
}


enum ExpectedArguments {
    None,
    Exactly(usize),
    Any,
}

trait ArgumentReceiver {
    fn expected_arguments() -> ExpectedArguments;
}

impl ArgumentReceiver for Echo {
   fn expected_arguments() -> ExpectedArguments {
       ExpectedArguments::Any
   } 
}

impl ArgumentReceiver for Ls {
    fn expected_arguments() -> ExpectedArguments {
        ExpectedArguments::None
    }
}

impl ArgumentReceiver for Cd {
    fn expected_arguments() -> ExpectedArguments {
        ExpectedArguments::Exactly(1)
    }
}

impl ArgumentReceiver for Cat {
    fn expected_arguments() -> ExpectedArguments {
        ExpectedArguments::Exactly(1)
    }
}

impl ArgumentReceiver for Exit {
    fn expected_arguments() -> ExpectedArguments {
        ExpectedArguments::None
    }
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

#[derive(Debug, PartialEq, Eq)]
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
    pub fn parse(command_string: &str) -> Result<Self, BadCommandError> {
        let mut command_parts = command_string.split_whitespace();
        if let Some(command) = command_parts.next() {
            match command {
                "echo" => {
                    let parsed_args = parse_arguments(command_parts, Echo::expected_arguments())?;
                    Ok(Command::Echo(Echo::new(parsed_args.join(" "))))
                },
                "ls" => {
                    parse_arguments(command_parts, Ls::expected_arguments())?;
                    Ok(Command::Ls(Ls::new()))
                },
                "cd" => {
                    let mut parsed_args = parse_arguments(command_parts, Cd::expected_arguments())?;
                    Ok(Command::Cd(Cd::new(parsed_args.remove(0))))
                },
                "cat" => {
                    let mut parsed_args = parse_arguments(command_parts, Cat::expected_arguments())?;
                    Ok(Command::Cat(Cat::new(parsed_args.remove(0))))
                },
                "exit" => {
                    parse_arguments(command_parts, Exit::expected_arguments())?;
                    Ok(Command::Exit(Exit::new()))
                },
                _ => {
                    Err(BadCommandError::from_string(format!("Unknown command {}", command)))
                }
            }
        } else {
            Err(BadCommandError::from_str("Missing command"))
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

    #[test]
    fn incorrect_arg_number_cd() {
        let result = Command::parse("cd dir1 dir2");

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), BadCommandError::wrong_arg_number(1, 2));
    }

    #[test]
    fn parses_exit() {
        let result = Command::parse("exit");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::Exit(Exit::new()));
    }
}
