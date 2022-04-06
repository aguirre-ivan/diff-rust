mod diff;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::process;

struct Config {
    filename1: String,
    filename2: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        let len = args.len();

        if len != 3 {
            return Err(format!(
                "ArgsError: Diff takes exactly 2 arguments ({} given)",
                len - 1
            ));
        }

        let filename1 = args[1].clone();
        let filename2 = args[2].clone();

        Ok(Config {
            filename1,
            filename2,
        })
    }
}

fn read_file_contents(file_name: &str) -> Result<String, Error> {
    let mut file = File::open(file_name)?;

    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

fn read_file_lines(file_name: &str) -> Result<Vec<String>, Error> {
    let file_contents = read_file_contents(file_name).unwrap_or_else(|err| {
        println!("FileError: {}: \"{}\"", err, file_name);
        process::exit(1);
    });

    let vec_lines: Vec<String> = file_contents.split('\n').map(|s| s.to_string()).collect();

    Ok(vec_lines)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1);
    });

    let lines_vec1 = match read_file_lines(&config.filename1) {
        Ok(lines_vec1) => lines_vec1,
        _ => return,
    };

    let lines_vec2 = match read_file_lines(&config.filename2) {
        Ok(lines_vec2) => lines_vec2,
        _ => return,
    };

    let diff_handler = diff::DiffHandler::new(&lines_vec1, &lines_vec2);

    diff_handler.print_diff();
}