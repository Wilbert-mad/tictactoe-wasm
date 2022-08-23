const BOARD_SIZE_COL: i8 = 3;
const BOARD_SIZE_ROW: i8 = 3;
const HOW_MANY_TO_SKIP: i8 = 2;

const EMPTY_FIL: &str = " ⬛ ";
const FIRST_FIL: &str = " 🟥 ";
const SECOND_FIL: &str = " 🟪 ";
const LEFT_TO_RIGHT: [(i8, i8); 3] = [(0, 0), (1, 1), (2, 2)];
const RIGHT_TO_LEFT: [(i8, i8); 3] = [(0, 2), (1, 1), (2, 0)];

pub struct TicTacToeGame {
    // (row, col)
    pub first_player_spots_tooken: Vec<(i8, i8)>,
    pub second_player_spots_tooken: Vec<(i8, i8)>,
    pub turn: i8,
}

impl TicTacToeGame {
    pub fn new() -> TicTacToeGame {
        TicTacToeGame {
            first_player_spots_tooken: Vec::new(),
            second_player_spots_tooken: Vec::new(),
            turn: 0,
        }
    }

    pub fn clear(&mut self) {
        self.first_player_spots_tooken = Vec::new();
        self.second_player_spots_tooken = Vec::new();
        self.turn = 0;
    }

    pub fn next_turn(&mut self) {
        if self.turn == 0 {
            return self.turn = 1;
        } else {
            self.turn = 0;
        }
    }

    fn peek_next_turn(&self) -> i8 {
        if self.turn == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    pub fn player_take(&mut self, (row, col): (i8, i8)) -> bool {
        // first player
        if self.turn == 0 {
            if !self.check_place_taken((row, col)) {
                self.first_player_spots_tooken.push((row, col));
                true
            } else {
                false
            }
            // second player
        } else if self.turn == 1 {
            if !self.check_place_taken((row, col)) {
                self.second_player_spots_tooken.push((row, col));
                true
            } else {
                false
            }
            // unknown player
        } else {
            true
        }
    }

    fn check_place_taken(&self, (row, col): (i8, i8)) -> bool {
        let next = self.peek_next_turn();

        if next == 0 {
            return self.first_player_spots_tooken.contains(&(row, col));
        } else if next == 1 {
            return self.second_player_spots_tooken.contains(&(row, col));
        } else {
            false
        }
    }

    pub fn check_player_win(&self) -> bool {
        if self.turn == 0 {
            return self.check_vertical_win(self.first_player_spots_tooken.clone())
                || self.check_horizontal_win(self.first_player_spots_tooken.clone())
                || self.check_slanted_win(self.first_player_spots_tooken.clone());
        } else if self.turn == 1 {
            return self.check_vertical_win(self.second_player_spots_tooken.clone())
                || self.check_horizontal_win(self.second_player_spots_tooken.clone())
                || self.check_slanted_win(self.second_player_spots_tooken.clone());
        } else {
            false
        }
    }

    pub fn check_slanted_win(&self, player_spots: Vec<(i8, i8)>) -> bool {
        if player_spots.is_empty() {
            return false;
        }

        return LEFT_TO_RIGHT
            .iter()
            .all(|&item| player_spots.contains(&item))
            || RIGHT_TO_LEFT
                .iter()
                .all(|&item| player_spots.contains(&item));
    }

    pub fn check_vertical_win(&self, player_spots: Vec<(i8, i8)>) -> bool {
        let mut vertical_exist = false;

        // println!("{:?}", player_spots);
        for row in 0..BOARD_SIZE_ROW {
            // if its 3, row is full
            let mut row_fullness: i8 = 0;

            for col in 0..BOARD_SIZE_COL {
                if player_spots.contains(&(row, col)) {
                    row_fullness += 1
                }
                // println!("{:?} - {:?}", (row, col), row_fullness);
            }

            if row_fullness == 3 {
                vertical_exist = true
            }
        }

        vertical_exist
    }

    pub fn check_horizontal_win(&self, player_spots: Vec<(i8, i8)>) -> bool {
        let mut horizontal_exist = false;

        // println!("{:?}", player_spots);
        let mut col_fullness: i8 = 0;
        let mut howmany_skiped: i8 = 0;
        let mut start_skiping = false;

        for row in 0..BOARD_SIZE_ROW {
            // if its 3, row is full

            for col in 0..BOARD_SIZE_COL {
                if start_skiping && howmany_skiped >= HOW_MANY_TO_SKIP {
                    howmany_skiped = 0;
                } else if start_skiping {
                    howmany_skiped += 1;
                }

                let condition = player_spots.contains(&(row, col))
                    && (col_fullness == 0 || howmany_skiped == 0);
                if condition {
                    start_skiping = true;
                    col_fullness += 1
                }

                // println!(
                //     "{:?} - skiped: {:?} - contains: {:?} - condition: {:?}",
                //     (row, col),
                //     howmany_skiped,
                //     player_spots.contains(&(row, col)),
                //     condition
                // );
            }
        }
        if col_fullness == 3 {
            horizontal_exist = true
        }
        // println!("------");

        horizontal_exist
    }

    /// Returns a string version of the board.
    ///
    /// ⬛⬛⬛
    ///
    /// ⬛⬛🟥
    ///
    /// ⬛⬛⬛
    pub fn render_board(&self, line_break: Option<&str>) -> String {
        let mut print_e = String::new();

        for row in 0..BOARD_SIZE_ROW {
            for col in 0..BOARD_SIZE_COL {
                // println!("ROW: {:?} - COL: {:?}", row, col);
                if self.first_player_spots_tooken.contains(&(row, col)) {
                    print_e.push_str(FIRST_FIL);
                } else if self.second_player_spots_tooken.contains(&(row, col)) {
                    print_e.push_str(SECOND_FIL);
                } else {
                    print_e.push_str(EMPTY_FIL);
                }
            }
            print_e.push_str(line_break.unwrap_or("\n"))
        }

        print_e.clone().trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_tictac_board() {
        let game = TicTacToeGame::new();

        assert_eq!(
            game.render_board(None),
            format!("{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}").trim()
        )
    }

    #[test]
    fn take_a_cell_and_other_player_cant_take_it() {
        let mut game = TicTacToeGame::new();
        // (2, 2)

        assert_eq!(game.player_take((1, 1)), true);

        assert_eq!(game.render_board(None), format!("{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}").trim());

        game.next_turn();
        assert_eq!(game.player_take((1, 1)), false);

        game.turn = 5;
        assert_eq!(game.player_take((1, 1)), true);
    }

    #[test]
    fn win_vertical() {
        let mut game = TicTacToeGame::new();

        assert_eq!(game.player_take((0, 0)), true);
        assert_eq!(game.player_take((0, 1)), true);
        assert_eq!(game.check_player_win(), false);
        assert_eq!(
            game.render_board(None),
            format!("{FIRST_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}").trim()
        );

        assert_eq!(game.player_take((0, 2)), true);
        assert_eq!(game.check_player_win(), true);
        assert_eq!(
            game.render_board(None),
            format!("{FIRST_FIL}{FIRST_FIL}{FIRST_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}").trim()
        );

        let mut other_game = TicTacToeGame::new();

        assert_eq!(other_game.player_take((0, 0)), true);
        assert_eq!(other_game.player_take((0, 1)), true);
        assert_eq!(other_game.check_player_win(), false);
        assert_eq!(
            other_game.render_board(None),
            format!("{FIRST_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}").trim()
        );

        assert_eq!(other_game.player_take((1, 2)), true);
        assert_eq!(other_game.check_player_win(), false);
        assert_eq!(other_game.render_board(None), format!("{FIRST_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{FIRST_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{EMPTY_FIL}").trim());
    }

    #[test]
    fn win_horizontal() {
        let mut game = TicTacToeGame::new();

        assert_eq!(game.player_take((0, 1)), true);
        assert_eq!(game.player_take((1, 1)), true);
        assert_eq!(game.player_take((2, 1)), true);

        assert_eq!(game.check_player_win(), true);

        assert_eq!(game.render_board(None), format!("{EMPTY_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{FIRST_FIL}{EMPTY_FIL}").trim());

        let mut other_game = TicTacToeGame::new();

        assert_eq!(other_game.player_take((0, 1)), true);
        assert_eq!(other_game.player_take((1, 1)), true);
        assert_eq!(other_game.player_take((2, 2)), true);

        assert_eq!(other_game.check_player_win(), false);

        assert_eq!(other_game.render_board(None), format!("{EMPTY_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{FIRST_FIL}").trim());
    }

    #[test]
    fn win_slanted() {
        let mut game = TicTacToeGame::new();

        assert_eq!(game.player_take((0, 0)), true);
        assert_eq!(game.player_take((1, 1)), true);
        assert_eq!(game.player_take((2, 2)), true);

        assert_eq!(game.check_player_win(), true);

        assert_eq!(game.render_board(None), format!("{FIRST_FIL}{EMPTY_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{FIRST_FIL}{EMPTY_FIL}\n{EMPTY_FIL}{EMPTY_FIL}{FIRST_FIL}").trim());

        let mut other_game = TicTacToeGame::new();

        assert_eq!(other_game.player_take((0, 2)), true);
        assert_eq!(other_game.player_take((1, 1)), true);
        assert_eq!(other_game.player_take((2, 0)), true);

        assert_eq!(other_game.check_player_win(), true);

        assert_eq!(other_game.render_board(None), format!("{EMPTY_FIL}{EMPTY_FIL}{FIRST_FIL}\n{EMPTY_FIL}{FIRST_FIL}{EMPTY_FIL}\n{FIRST_FIL}{EMPTY_FIL}{EMPTY_FIL}").trim());
    }

    #[test]
    fn play_game() {
        let mut game = TicTacToeGame::new();

        assert_eq!(game.player_take((0, 0)), true);
        game.next_turn();
        assert_eq!(game.player_take((1, 1)), true);
        game.next_turn();
        assert_eq!(game.player_take((2, 0)), true);
        game.next_turn();
        assert_eq!(game.player_take((1, 2)), true);
        game.next_turn();
        assert_eq!(game.player_take((1, 0)), true);

        assert_eq!(game.check_player_win(), true);
    }
}
