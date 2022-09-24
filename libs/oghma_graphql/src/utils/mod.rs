use std::error::Error;
use std::fmt::Display;

pub fn parse_error<E: Error, T, R: Display>(error: E, response: R) -> Vec<T> {
    if format!("{error}").contains("invalid length 0") {
        Vec::new()
    } else {
        panic!("Failed to parse users!\n\nWith Error: {error}\n\nResponse Body: {response}\n\n")
    }
}
