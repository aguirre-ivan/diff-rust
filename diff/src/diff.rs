use std::cmp::max;

pub struct DiffHandler<'a> {
    sequence1: &'a [String],
    sequence2: &'a [String],
    grid: Vec<Vec<i32>>,
}

impl DiffHandler<'_> {
    pub fn new<'a>(sequence1: &'a [String], sequence2: &'a [String]) -> DiffHandler<'a> {
        let grid = DiffHandler::create_grid_array(sequence1, sequence2);

        DiffHandler {
            sequence1,
            sequence2,
            grid,
        }
    }

    pub fn print_diff(self) {
        DiffHandler::_print_diff(
            self.grid,
            self.sequence1,
            self.sequence2,
            self.sequence1.len(),
            self.sequence2.len(),
        );
    }

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
