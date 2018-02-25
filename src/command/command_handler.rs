use std::collections::HashMap;

use super::{Command, Result};
use command::decrypt::DecryptCommand;
use command::encrypt::EncryptCommand;

pub struct CommandHandler {
    commands: Vec<Box<Command>>
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        let mut commands: Vec<Box<Command>> = Vec::new();

        let encrypt_command = EncryptCommand::new();
        let decrypt_command = DecryptCommand::new();

        commands.push(Box::new(encrypt_command));
        commands.push(Box::new(decrypt_command));

        CommandHandler {
            commands
        }
    }

    pub fn handle(&self, command_name: &str, arguments: &[&str]) -> Result<()> {
        for command in self.commands.iter() {
            if command_name == command.name() {
                return command.run(arguments)
            }
        }

        let err = UnknownCommandError {
            command_name: command_name.to_string()
        };

        Err(CommandError::from(err))
    }
}

use std;
use std::fmt;

use super::CommandError;

#[derive(Debug)]
pub struct UnknownCommandError {
    command_name: String
}

impl fmt::Display for UnknownCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(f, "Unknown Command \"{}\".", self.command_name)
    }
}

impl std::error::Error for UnknownCommandError {
    fn description(&self) -> &str {
        "Unknown Command."
    }
}