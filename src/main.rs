#[macro_use] extern crate text_io;
extern crate ring;

mod command;
use command::Command;
use command::encrypt::EncryptCommand;

use std::env;


fn print_flush(output: &str) {
    use std::io;
    use std::io::prelude::*;

    print!("{}", output);
    io::stdout().flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // login
    print_flush("Input your username: ");
    let username: String = read!("{}\n");
    print_flush("Input your password: ");
    let password: String = read!("{}\n");
    println!("Successfully logged in as: {}", username);

    // run
    loop {
        print_flush("> ");
        let command_string: String = read!("{}\n");
        let command_vec: Vec<&str> = command_string.split(' ').collect();

        if let Some((command_name, arguments)) = command_vec.split_first() {

            // @TODO: Command Handler
            if command_name== &"encrypt" {
                let command
                = EncryptCommand::new();

                match command.run(arguments) {
                    Ok(()) => println!("Successfully ran command"),
                    Err(message) => println!("{}", message)
                }
            }
        }
    }
}