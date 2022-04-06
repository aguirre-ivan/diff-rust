use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn read_file_contents(file_name: &str) -> Result<String, Error> {
    let mut file = File::open(file_name)?;

    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

pub fn read_file_lines(file_name: &str) -> Result<Vec<String>, Error> {
    let file_contents = read_file_contents(file_name)?;

    let vec_lines: Vec<String> = file_contents.split('\n').map(|s| s.to_string()).collect();

    Ok(vec_lines)
}