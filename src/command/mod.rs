pub trait Command {
    fn run(&self, arguments: &[&str]) -> Result<(), &str>;
}

pub mod encrypt;