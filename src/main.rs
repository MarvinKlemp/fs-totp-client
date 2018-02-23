use std::env;
use std::io;
use std::io::prelude::*;
use std::ffi::OsString;

#[macro_use] extern crate text_io;
extern crate ring;

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
        let command_string: String = read!("{}\n");
        let command_vec: Vec<&str> = command_string.split(' ').collect();

        if let Some((command_name, arguments)) = command_vec.split_first() {
            if command_name== &"encrypt" {
                let command: EncryptCommand;

                match EncryptCommand::create(arguments) {
                    Some(cmd) => command = cmd,
                    None => {println!("Unable to run command."); continue}
                }

                match command.run() {
                    Ok(()) => println!("Successfully ran command"),
                    Err(message) => println!("{}", message)
                }
            }
        }
    }
}

trait Command {
    fn run(&self) -> Result<(), String>;
}

#[derive(Debug)]
struct EncryptCommand {
    path: OsString
}

impl EncryptCommand {
    fn create(vec: &[&str]) -> Option<EncryptCommand> {

        if vec.len() < 1 {
            return None;
        }

        Some(EncryptCommand {
            path: OsString::from(vec[0].to_string())
        })
    }
}

impl Command for EncryptCommand {
    fn run(&self) -> Result<(), String> {
        use std::io::prelude::*;
        use std::fs::File;

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut f = File::open(&self.path).unwrap();

            f.read_to_end(&mut buffer);
        }

        {
            use ring::aead::*;
            use ring::{pbkdf2, digest};

            let password = b"password";
            let salt = [0, 1, 2, 3, 4, 5, 6, 7];

            let mut key = [0; 32];
            pbkdf2::derive(&digest::SHA256, 10, &salt, &password[..], &mut key);

            let mut encrypted = buffer.clone();
            for _ in 0..CHACHA20_POLY1305.tag_len() {
                encrypted.push(0);
            }

            let opening_key = OpeningKey::new(&CHACHA20_POLY1305, &key).unwrap();
            let sealing_key = SealingKey::new(&CHACHA20_POLY1305, &key).unwrap();

            use ring::rand::{SystemRandom, SecureRandom};
            let random = SystemRandom::new();
            let mut nonce = vec![0; 12];
            random.fill(&mut nonce).unwrap();

            seal_in_place(
                &sealing_key,
                &nonce,
                &[],
                &mut encrypted,
                CHACHA20_POLY1305.tag_len()
            ).unwrap();

            let mut encrypted_file = File::create("test.txt").unwrap();
            encrypted_file.write(&encrypted[..]);
        }

        Ok(())
    }
}