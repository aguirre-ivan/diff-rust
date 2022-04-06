// ! Diff

// ! Compare two FILES line by line.
// ! It takes two arguments (file names)

mod diff;
mod file_handler;

use std::env;
use std::process;

// Args config to use in file_handler and diff
// It contains file's names.
struct ArgsConfig {
    filename1: String,
    filename2: String,
}

impl ArgsConfig {
    // Reutrns struct ArgConfig
    fn new(args: &[String]) -> Result<ArgsConfig, String> {
        let len = args.len();

        if len != 3 {
            return Err(format!(
                "ArgsError: Diff takes exactly 2 arguments ({} given)",
                len - 1
            ));
        }

        let filename1 = args[1].clone();
        let filename2 = args[2].clone();

        Ok(ArgsConfig {
            filename1,
            filename2,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_config = ArgsConfig::new(&args).unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1);
    });

    let lines_vec1 = file_handler::read_file_lines(&args_config.filename1).unwrap_or_else(|err| {
        println!("FileError: {}: \"{}\"", err, args_config.filename1);
        process::exit(1);
    });

    let lines_vec2 = file_handler::read_file_lines(&args_config.filename2).unwrap_or_else(|err| {
        println!("FileError: {}: \"{}\"", err, args_config.filename2);
        process::exit(1);
    });

    let diff_handler = diff::DiffHandler::new(&lines_vec1, &lines_vec2);

    diff_handler.print_diff();
}