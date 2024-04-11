use std::fs::File;
use std::io::{self, Read};
use thiserror::Error;


#[derive(Debug, Error)]
enum ReadUsernameError {
    #[error("Could not read: `{0}`")]
    IoError(io::Error),
    #[error("Empty file `{0}`")]
    EmptyFile(String),
    #[error("Not found `{name}` in `{filename}`")]
    NameNotFound { name: String, filename: String },
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> ReadUsernameError {
        ReadUsernameError::IoError(err)
    }
}

fn read_username(path: &str, name: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyFile(String::from(path)));
    } else if !username.contains(name) {
        return Err(ReadUsernameError::NameNotFound {
            name: name.to_string(),
            filename: path.to_string(),
        });
    } else {
        Ok(username)
    }
}

fn main() {
    match read_username("aki.txt", "lucas") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}
