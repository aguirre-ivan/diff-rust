use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::process;

fn create_grid_array(x: &[String], y: &[String]) -> Vec<Vec<i32>> {
    let m = x.len();
    let n = y.len();

    let mut grid = vec![vec![0; n + 1]; m + 1];

    for i in 0..m {
        for (j, line) in y.iter().enumerate().take(n) {
            if x[i] == *line {
                grid[i + 1][j + 1] = grid[i][j] + 1;
            } else {
                grid[i + 1][j + 1] = max(grid[i + 1][j], grid[i][j + 1]);
            }
        }
    }

    grid
}

struct DiffHandler<'a> {
    sequence1: &'a [String],
    sequence2: &'a [String],
    grid: Vec<Vec<i32>>,
}

impl DiffHandler<'_> {
    fn new<'a>(sequence1: &'a [String], sequence2: &'a [String]) -> DiffHandler<'a> {
        let grid = create_grid_array(sequence1, sequence2);

        DiffHandler {
            sequence1,
            sequence2,
            grid,
        }
    }

    fn print_diff(self) {
        DiffHandler::_print_diff(
            self.grid,
            self.sequence1,
            self.sequence2,
            self.sequence1.len(),
            self.sequence2.len(),
        );
    }

    fn _print_diff(c: Vec<Vec<i32>>, x: &[String], y: &[String], i: usize, j: usize) {
        if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
            DiffHandler::_print_diff(c, x, y, i - 1, j - 1);
            println!("  {}", x[i - 1]);
        } else if j > 0 && (i == 0 || c[i][j - 1] >= c[i - 1][j]) {
            DiffHandler::_print_diff(c, x, y, i, j - 1);
            println!("> {}", y[j - 1]);
        } else if i > 0 && (j == 0 || c[i][j - 1] < c[i - 1][j]) {
            DiffHandler::_print_diff(c, x, y, i - 1, j);
            println!("< {}", x[i - 1]);
        } else {
            println!();
        }
    }
}
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

    let diff_handler = DiffHandler::new(&lines_vec1, &lines_vec2);

    diff_handler.print_diff();

    // println!("{:?}", lines_vec1);
    // println!("{:?}", lines_vec2);
}
