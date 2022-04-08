//! A Diff Vecs Handler can print two `Vec<String>` diff.
//!
//! It has two sequences (`Vec<String>`) to apply diff.
//!
//! Also has a grid to implement LCS - Longest Common Subsequence.

use std::cmp::max;

pub struct DiffStringVecsHandler<'a> {
    string_vec_1: &'a [String],
    string_vec_2: &'a [String],
    grid: Vec<Vec<i32>>,
}

impl DiffStringVecsHandler<'_> {
    /// Returns a DiffStringVecsHandler with sequences lifetime.
    ///
    /// Creates a grid to implement LCS - Longest Common Subsequence.
    pub fn new<'a>(
        string_vec_1: &'a [String],
        string_vec_2: &'a [String],
    ) -> DiffStringVecsHandler<'a> {
        let grid = DiffStringVecsHandler::create_grid_array(string_vec_1, string_vec_2);

        DiffStringVecsHandler {
            string_vec_1,
            string_vec_2,
            grid,
        }
    }

    /// Prints sequences diff, it uses LCS - Longest Common Subsequence.
    ///
    /// Examples
    ///
    /// ```no_run
    /// let lines_vec1 = file_handler::read_file_lines("file1.txt").unwrap_or_else(|err| {
    ///     println!("FileError: {}: \"{}\"", err, "file1.txt");
    ///     process::exit(1);
    /// });
    ///
    /// let lines_vec2 = file_handler::read_file_lines("file2.txt").unwrap_or_else(|err| {
    ///     println!("FileError: {}: \"{}\"", err, "file2.txt");
    ///     process::exit(1);
    /// });
    /// let diff_handler = diff::DiffStringVecsHandler::new(&lines_vec1, &lines_vec2);
    /// diff_handler.print_diff();
    /// ```
    /// With file1.txt:
    /// ```
    /// hi
    /// goodbye!
    /// ```
    /// and file2.txt:
    /// ```
    /// hellogoodbye
    /// goodbye!
    /// ```
    /// It prints:
    /// ```
    /// < hi
    /// > hello
    /// goodbye!
    /// ```
    pub fn print_diff(self) {
        DiffStringVecsHandler::_print_diff(
            self.grid,
            self.string_vec_1,
            self.string_vec_2,
            self.string_vec_1.len(),
            self.string_vec_2.len(),
        );
    }

    /// Creates a grid (`Vec<Vec<i32>>`) with LCS implementation
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

    /// Prints diff from LCS grid
    fn _print_diff(c: Vec<Vec<i32>>, x: &[String], y: &[String], i: usize, j: usize) {
        if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
            DiffStringVecsHandler::_print_diff(c, x, y, i - 1, j - 1);
            println!("  {}", x[i - 1]);
        } else if j > 0 && (i == 0 || c[i][j - 1] >= c[i - 1][j]) {
            DiffStringVecsHandler::_print_diff(c, x, y, i, j - 1);
            println!("> {}", y[j - 1]);
        } else if i > 0 && (j == 0 || c[i][j - 1] < c[i - 1][j]) {
            DiffStringVecsHandler::_print_diff(c, x, y, i - 1, j);
            println!("< {}", x[i - 1]);
        } else {
            println!();
        }
    }
}
