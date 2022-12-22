use itertools::Itertools;

pub enum Command {
    Echo(String),
    Ls,
    Cd(String),
    Cat(String),
    Exit,
}

enum ExpectedArguments {
    None,
    Exactly(u8),
    Any,
}

// TODO: Factory function
#[derive(Debug)]
pub struct BadCommandError {
    msg: String,
}

impl Command {
    // TODO: Clean it up with the below function
    // TODO: Test
    pub fn parse(command_string: &str) -> Result<Self, BadCommandError> {
        let mut command_parts = command_string.split_whitespace();
        if let Some(command) = command_parts.next() {
            match command {
                "echo" => {
                    Ok(Command::Echo(command_parts.join(" ")))
                },
                "ls" => {
                    Ok(Command::Ls)
                },
                "cd" => {
                    if let Some(argument) = command_parts.next() {
                        Ok(Command::Cd(String::from(argument)))
                    } else {
                        Err(BadCommandError {
                            msg: String::from("Missing argument to cd")
                        })
                    }
                },
                "cat" => {
                    if let Some(argument) = command_parts.next() {
                        Ok(Command::Cat(String::from(argument)))
                    } else {
                        Err(BadCommandError {
                            msg: String::from("Missing argument to cat")
                        })
                    }
                },
                "exit" => {
                    Ok(Command::Exit)
                },
                _ => {
                    Err(BadCommandError {
                        msg: String::from(format!("Unknown command {}", command))
                    })
                }
            }
        } else {
            Err(BadCommandError {
                msg: String::from("Missing command")
            })
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
