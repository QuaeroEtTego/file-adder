use std::{
    env,
    fs::{create_dir_all, File},
    io::{Read, Result as IoResult, Write},
    process::ExitCode,
};

use error::Error;

mod dirs;
mod error;
mod interaction;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const INPUT_FILE: &[u8] = include_bytes!(env!("INPUT_PATH"));
const OUTPUT_PATH: &str = env!("OUTPUT_PATH");

fn main() -> ExitCode {
    println!("{APP_NAME} v{APP_VERSION}");

    if let Err(e) = real_main() {
        eprintln!("{e}");
        let _ = wait();

        return ExitCode::FAILURE;
    }

    let _ = wait();

    ExitCode::SUCCESS
}

fn real_main() -> Result<(), Error> {
    let path = dirs::parse_path(OUTPUT_PATH);

    let Some(path_str) = path.to_str() else {
        return Err(Error::InvalidPath);
    };

    println!("File {path_str}");

    let mut interaction = interaction::ask("Do you want this file?")?;

    if !interaction {
        return Ok(());
    }

    let path = dirs::parse_path(OUTPUT_PATH);
    let Some(parent) = path.parent() else {
        return Err(Error::InvalidPath);
    };

    // Create missing directories
    create_dir_all(parent).map_err(|_| Error::DirectoryCreate)?;

    // Check if the file exists
    if path.is_file() {
        interaction = interaction::ask("File exists, overwrite it?")?;

        if !interaction {
            return Ok(());
        }
    }

    // Open/create the file
    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|_| Error::FileOpen)?;

    println!("Created/opened the file.");

    // Write all data in the file
    file.write_all(INPUT_FILE).map_err(|_| Error::FileWrite)?;

    println!("All data have been written in the file.");

    Ok(())
}

fn wait() -> IoResult<()> {
    println!("Press ENTER to leave...");
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer)
}
