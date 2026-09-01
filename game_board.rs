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

            let mut board = vec![vec![0; w as usize]; h as usize];
            let mut master = vec![vec![0; w as usize]; h as usize];

            let mut rng = rand::rng();
            let mut placed = 0usize;

            while placed < m {
                let row = rng.random_range(0..h);
                let col = rng.random_range(0..w);

                if master[row][col] == 0 {
                    master[row][col] = 9;
                    placed += 1;

                }
            }

            Self { option, board, master }
        }
        fn populate_neighbors(row: usize, col: usize, board: Vec<_>) {
            let range = match (row, col) {
                (1, 1) => [[row,row+1],[col,col+1]],
                (1, 1) => {
                    [[row,row+1],[col-1,col+1]]
                },
            };
        }

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