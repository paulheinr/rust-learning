use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;
use thiserror::Error;

/// implements the tasks from https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes
fn main() {
    let password = Password::<DefaultPasswordPolicy>::new(String::from("aa")).unwrap();

    match password.check("ThisIsAPassword1") {
        Ok(_) => println!("Password matches!"),
        Err(e) => println!("Error: {}", e),
    }
}

trait PasswordPolicy {
    fn check(candidate: &str) -> Result<(), PasswordCreationError>;
}

struct DefaultPasswordPolicy;

impl PasswordPolicy for DefaultPasswordPolicy {
    fn check(candidate: &str) -> Result<(), PasswordCreationError> {
        if candidate.len() < 8 {
            return Err(PasswordCreationError(candidate.to_string()));
        }
        if !candidate.is_ascii() {
            return Err(PasswordCreationError(candidate.to_string()));
        }
        Ok(())
    }
}

struct NoPasswordPolicy;

impl PasswordPolicy for NoPasswordPolicy {
    fn check(_candidate: &str) -> Result<(), PasswordCreationError> {
        Ok(())
    }
}

// Not sure whether static generics are the right way to go here, but it works
struct Password<P: PasswordPolicy>(u64, PhantomData<P>);

#[derive(Debug, Error, Clone, PartialEq)]
#[error("{0} is not a valid password")]
struct PasswordCreationError(String);

#[derive(Debug, Error, Clone, PartialEq)]
#[error("{0} does not match the password")]
struct PasswordCheckError(String);

impl<P: PasswordPolicy> Password<P> {
    fn new(password: String) -> Result<Password<P>, PasswordCreationError> {
        P::check(password.as_str())?;
        
        let mut hasher = DefaultHasher::default();
        password.hash(&mut hasher);
        Ok(Password(hasher.finish(), PhantomData))
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