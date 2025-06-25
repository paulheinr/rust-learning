use std::hash::{DefaultHasher, Hash, Hasher};
use thiserror::Error;

/// implements the tasks from https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes
fn main() {
    let password = Password::new(String::from("ThisIsAPassword")).unwrap();

    match password.check("ThisIsAPassword1") {
        Ok(_) => println!("Password matches!"),
        Err(e) => println!("Error: {}", e),
    }
}

struct Password(u64);

#[derive(Debug, Error, Clone, PartialEq)]
#[error("{0} is not a valid password")]
struct PasswordCreationError(String);

#[derive(Debug, Error, Clone, PartialEq)]
#[error("{0} does not match the password")]
struct PasswordCheckError(String);

impl Password {
    fn new(password: String) -> Result<Password, PasswordCreationError> {
        if password.len() < 8 {
            Err(PasswordCreationError(password))
        } else if !password.is_ascii() {
            Err(PasswordCreationError(password))
        } else {
            let mut hasher = DefaultHasher::default();
            password.hash(&mut hasher);
            Ok(Password(hasher.finish()))
        }
    }
    
    pub fn check(&self, candidate: &str) -> Result<(), PasswordCheckError> {
        let mut hasher = DefaultHasher::default();
        candidate.hash(&mut hasher);
        if hasher.finish() != self.0 {
            return Err(PasswordCheckError(candidate.to_string()));
        }
        Ok(())
    }
}