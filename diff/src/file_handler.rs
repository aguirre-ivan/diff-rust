//! File Handler
//!
//! To convert a file content to a Vec<String> with every line.

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

/// Reads the .txt "file_name" and returns a String with file content.
///
/// # Errors
/// This function will return an error if path does not already exist. Other errors may also be returned according to OpenOptions::open.
///
/// If the data in this stream is not valid UTF-8 then an error is returned.
fn read_file_content(file_name: &str) -> Result<String, Error> {
    let mut file = File::open(file_name)?;

    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

/// Reads the .txt "file_name" and returns a Vec<String> with every line.
///
/// # Errors
/// This function will return an error if path does not already exist. Other errors may also be returned according to OpenOptions::open
///
/// If the data in this stream is not valid UTF-8 then an error is returned.
///
/// # Examples
///
/// With file_name = example.txt:
///
/// ```
/// "foo"
/// ```
///
/// ```no_run
/// let file_content = read_file_content("example.txt");
/// let vec_lines: Vec<String> = file_content;
/// assert_eq!(vec_lines, vec!["foo".to_string()]);
/// ```
pub fn read_file_lines(file_name: &str) -> Result<Vec<String>, Error> {
    let file_content = read_file_content(file_name)?;

    let vec_lines: Vec<String> = file_content.split('\n').map(|s| s.to_string()).collect();

    Ok(vec_lines)
}
