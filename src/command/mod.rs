use std;
use std::fmt;

pub type CommandError = Box<std::error::Error>;
pub type Result<T> = std::result::Result<T, CommandError>;

pub trait Command {
    fn run(&self, arguments: &[&str]) -> Result<()>;
}

pub mod encrypt;

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
