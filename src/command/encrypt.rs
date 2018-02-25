use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

use ring::aead::*;
use ring::{pbkdf2, digest};
use ring::rand::{SystemRandom, SecureRandom};

use super::{Command, Result, CommandError, InvalidInputError};

pub struct EncryptCommand;

impl EncryptCommand {
    pub fn new() -> EncryptCommand {
        EncryptCommand {}
    }
}

impl Command for EncryptCommand {
    fn name(&self) -> &str {
        &"encrypt"
    }

    fn run(&self, arguments: &[&str]) -> Result<()> {
        if arguments.len() < 2 {
            let err = InvalidInputError {};

            return Err(CommandError::from(err));
        }

        let file_path = Path::new(arguments[0]);
        let target_path = Path::new(arguments[1]);

        let mut buffer: Vec<u8> = Vec::new();
        {
            let mut f = File::open(file_path)?;
            f.read_to_end(&mut buffer)?;
        }

        let password = b"password";
        let salt = [0, 1, 2, 3, 4, 5, 6, 7];

        let mut key = [0; 32];
        pbkdf2::derive(&digest::SHA256, 10, &salt, &password[..], &mut key);

        let mut encrypted = buffer.clone();
        for _ in 0..CHACHA20_POLY1305.tag_len() {
            encrypted.push(0);
        }

        let random = SystemRandom::new();
        let mut nonce = vec![0; 12];
        random.fill(&mut nonce).unwrap();

        seal_in_place(
            &SealingKey::new(&CHACHA20_POLY1305, &key).unwrap(),
            &nonce,
            &[],
            &mut encrypted,
            CHACHA20_POLY1305.tag_len()
        )?;

        let mut encrypted_file = File::create(target_path)?;
        encrypted_file.write(&[&nonce[..], &encrypted[..]].concat())?;

        Ok(())
    }
}