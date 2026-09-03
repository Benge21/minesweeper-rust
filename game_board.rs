pub mod game_board {
    use std::vec;

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

            let _row_max = master.len() - 1;
            let _col_max = master[0].len() - 1;

            while placed < m {
                let row = rng.random_range(0..h);
                let col = rng.random_range(0..w);
                
                if master[row][col] == 0 {
                    master[row][col] = 9;
                    placed += 1;
                    for i in row.saturating_sub(1)..=(row + 1).min(_row_max) { // row
                        for j in col.saturating_sub(1)..=(col + 1).min(_col_max) { // col
                            if master[i][j] != 9 {
                                master[i][j] += 1;
                            };
                        };
                    };
                }
            }

            Self { option, board, master }
        }

        pub fn get_board(&self) -> &Vec<Vec<i8>> {
            &self.board
        }

        pub fn get_master(&self) -> &Vec<Vec<i8>> {
            &self.master
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