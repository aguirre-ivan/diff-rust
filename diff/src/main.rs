// ! Diff

// ! Compare two FILES line by line.
// ! It takes two arguments (file names)

mod diff_string_vecs;
mod file_handler;
mod args_config;

use std::env;
use std::process;
use args_config::ArgsConfig;
use diff_string_vecs::DiffStringVecsHandler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_config = ArgsConfig::new(&args);

    let lines_vec1 = file_handler::read_file_lines(&args_config.filename1).unwrap_or_else(|err| {
        println!("FileError: {}: \"{}\"", err, args_config.filename1);
        process::exit(1);
    });

    let lines_vec2 = file_handler::read_file_lines(&args_config.filename2).unwrap_or_else(|err| {
        println!("FileError: {}: \"{}\"", err, args_config.filename2);
        process::exit(1);
    });

    let diff_handler = DiffStringVecsHandler::new(&lines_vec1, &lines_vec2);

    diff_handler.print_diff();
}