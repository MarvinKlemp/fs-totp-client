use std;
use std::fmt;

pub type CommandError = Box<std::error::Error>;
pub type Result<T> = std::result::Result<T, CommandError>;

pub trait Command {
    fn name(&self) -> &str;

    fn run(&self, arguments: &[&str]) -> Result<()>;
}

pub mod command_handler;
pub mod encrypt;
pub mod decrypt;

#[derive(Debug)]
pub struct InvalidInputError;

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(f, "Invalid input passed.")
    }
}

impl std::error::Error for InvalidInputError {
    fn description(&self) -> &str {
        "invalid input passed."
    }
}
