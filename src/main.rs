use std::env;
use std::io;
use std::io::prelude::*;

#[macro_use] extern crate text_io;

fn print_flush(output: &str) {
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
        let command: String = read!("{}\n");
    }
}
