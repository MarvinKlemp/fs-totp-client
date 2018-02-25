#[macro_use] extern crate text_io;
extern crate ring;

use std::env;

mod command;
use command::Command;
use command::encrypt::EncryptCommand;
use command::decrypt::DecryptCommand;
use command::command_handler::CommandHandler;

fn print_flush(output: &str) {
    use std::io;
    use std::io::prelude::*;

    print!("{}", output);
    io::stdout().flush().unwrap();
}

fn main() {
    // login
    print_flush("Input your username: ");
    let username: String = read!("{}\n");
    print_flush("Input your password: ");
    let password: String = read!("{}\n");
    println!("Successfully logged in as: {}", username);

    let command_handler = CommandHandler::new();
    // run
    loop {
        print_flush("> ");
        let command_string: String = read!("{}\n");
        let command_vec: Vec<&str> = command_string.split(' ').collect();

        if let Some((command_name, arguments)) = command_vec.split_first() {
            command_handler.handle(command_name, arguments);

            match command_handler.handle(command_name, arguments) {
                Ok(()) => println!("Successfully ran command"),
                Err(err) => println!("{}", err)
            }
        }
    }
}