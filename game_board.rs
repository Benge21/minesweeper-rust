pub mod game_board {
    use rand::RngExt;

    pub struct GameBoard {
        option: i8,
        board: Vec<Vec<i8>>,
        master: Vec<Vec<i8>>,
    }

    impl GameBoard {
        pub fn new(option: i8) -> Self {
            let (w, h, m) = match option {
                1 => (9, 9, 8),
                2 => (16, 16, 13),
                3 => (30, 16, 16),
                _ => (9, 9, 8),
            };

            let board      = vec![vec![0; w as usize]; h as usize];
            let mut master = vec![vec![0; w as usize]; h as usize];

            let mut rng = rand::rng();
            let mut placed = 0usize;

            while placed < m {
                let row = rng.random_range(0..h);
                let col = rng.random_range(0..w);

                if master[row][col] == 0 {
                    master[row][col] = 9;
                    placed += 1;
                    unsafe {
                        for i in row-1..row+1 { // row
                            for j in col-1..col+1 { // col
                                master[i][j] += 1;
                            };
                        };
                    }
                }
            }

            Self { option, board, master }
        }
        // fn populate_neighbors(row: usize, col: usize, master: Vec<Vec<i8>>) {
        //     let row_max = self.master.len() - 1;
        //     let col_max = self.master[0].len() - 1;
        //     let range = match (row, col) {
        //         (0, 0)                           => [[row,row+1],[col,col+1]],
        //         (0, col_max)              => [[row,row+1],[col-1,col]],
        //         (row_max, 0)              => [[row-1,row],[col,col+1]],
        //         (row_max, col_max) => [[row-1,row],[col-1,col]],
        //         (row_max, _)              => [[row-1,row],[col-1,col+1]],
        //         (0, _)                           => [[row,row+1],[col-1,col+1]],
        //         (_, 0)                           => [[row-1,row+1],[col,col+1]],
        //         (_, col_max)              => [[row-1,row+1],[col-1,col]],
        //         _ => [[row-1,row+1],[col-1,col+1]],
        //     };
        //     for i in range[0][0]..range[0][1] { // row
        //         for j in range[1][0]..range[1][1] { // col
        //             self.master[i][j] += 1;
        //         };
        //     };
        // }

        pub fn get_board(&self) -> &Vec<Vec<i8>> {
            &self.board
        }

        pub fn make_guess(&mut self, x: i32, y: i32) {
            if x < 0 || y < 0 {
                return;
            }

            let x = x as usize;
            let y = y as usize;

            if let Some(row) = self.board.get_mut(y) {
                if let Some(cell) = row.get_mut(x) {
                    *cell = 1;
                }
            }
        }
    }
}