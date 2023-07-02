use std::io;

use super::error::Error;

pub fn ask(message: &str) -> Result<bool, Error> {
    println!("{message} [y/n]");

    loop {
        match parse_user_input(read_user_input()?.to_lowercase().trim()) {
            Some(interaction) => return Ok(interaction),
            None => println!("{message} [y/n]"),
        }
    }
}

fn read_user_input() -> Result<String, Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(_) => Err(Error::Interaction),
    }
}

fn parse_user_input(input: &str) -> Option<bool> {
    match input {
        "y" | "yes" => Some(true),
        "n" | "no" => Some(false),
        _ => None,
    }
}
