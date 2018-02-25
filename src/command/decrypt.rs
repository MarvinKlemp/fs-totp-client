use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

use ring::aead::*;
use ring::{pbkdf2, digest};
use ring::rand::{SystemRandom, SecureRandom};

use super::{Command, Result, CommandError, InvalidInputError};

pub struct DecryptCommand;

impl DecryptCommand {
    pub fn new() -> DecryptCommand {
        DecryptCommand {}
    }
}

impl Command for DecryptCommand {
    fn run(&self, arguments: &[&str]) -> Result<()> {
        if arguments.len() < 2 {
            let err = InvalidInputError {};

            return Err(CommandError::from(err));
        }

        let file_path = Path::new(arguments[0]);
        let target_path = Path::new(arguments[1]);

        let mut encrypted: Vec<u8> = Vec::new();
        {
            let mut f = File::open(file_path)?;
            f.read_to_end(&mut encrypted)?;
        }

        let password = b"password";
        let salt = [0, 1, 2, 3, 4, 5, 6, 7];
        let mut key = [0; 32];
        pbkdf2::derive(&digest::SHA256, 10, &salt, &password[..], &mut key);

        let mut nonce = vec![0; 12];

        let seal_key = OpeningKey::new(&CHACHA20_POLY1305, &key).unwrap();
        let decrypted = open_in_place(
            &seal_key,
            &nonce,
            &[],
            0,
            &mut encrypted
        )?;

        let mut decrypted_file = File::create(target_path)?;
        decrypted_file.write(&decrypted[..])?;

        Ok(())
    }
}