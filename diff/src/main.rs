//! Diff
//!
//! Compare two FILES line by line.
//!
//! It takes two arguments (file names)

mod args_config;
mod diff_string_vecs;
mod file_handler;

use args_config::ArgsConfig;
use diff_string_vecs::DiffStringVecsHandler;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_config = ArgsConfig::new(&args);

    match args_config {
        Ok(args_config) => {
            let lines_vec_1 = file_handler::read_file_lines(&args_config.filename1);
            let lines_vec_2 = file_handler::read_file_lines(&args_config.filename2);

            match lines_vec_1 {
                Ok(lines_vec_1) => match lines_vec_2 {
                    Ok(lines_vec_2) => {
                        let diff_handler = DiffStringVecsHandler::new(&lines_vec_1, &lines_vec_2);
                        diff_handler.print_diff();
                    }
                    Err(err) => {
                        println!("{}: \"{}\"", err, args_config.filename2);
                    }
                },
                Err(err) => {
                    println!("{}: \"{}\"", err, args_config.filename1);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
