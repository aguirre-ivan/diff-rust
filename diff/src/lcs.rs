struct Grid {
    grid_array: Vec<usize>,
    length_n: usize,
    length_m: usize,
}

impl Grid {
    fn new(sequence1: Vec<String>, sequence2: Vec<String>) -> Grid {

        let length_n = len(sequence1);
        let length_m = len(sequence2);

        let grid_array = create_grid_array(sequence1, sequence2);

        Grid {
            grid_array,
            length_n,
            length_m,
        }
    }

    fn new(sequence1: Vec<String>, sequence2: Vec<String>) -> Vec<usize> {
        let length_n = len(sequence1);
        let length_m = len(sequence2);

        let mut grid_array = Vec<Vec<usize>>::new();

        for i in 0..(length_n+1) {
            grid_array[i][0].push(0);
        }

        for i in 0..(length_m+1) {
            grid_array[0][i].push(0);
        }

        for i in 0..length_m {
            for j in 0..length_n {
                if sequence1[i] == sequence2[j] {
                    grid_array[i+1][j+1] = grid_array[i][j] + 1;

                }
            }
        }


    }
}

struct DiffHandler {
    sequence1: Vec<String>,
    sequence2: Vec<String>,
    grid: Grid,
}

impl DiffHandler {
    fn new(sequence1: Vec<String>, sequence2: Vec<String>) -> DiffHandler {
        
        let grid = Grid::new(sequence1, sequence2);
        
        DiffHandler {
            sequence1,
            sequence2,
            grid,
        }
    }

    fn print_diff() {
        println!("hello world")
    }

}