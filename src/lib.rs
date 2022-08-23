mod tictactoe;

use std::cell::RefCell;

use tictactoe::TicTacToeGame;
use wasm_bindgen::prelude::*;

thread_local! {
    static TICTACTOE: RefCell<TicTacToeGame> = RefCell::new(TicTacToeGame::new());
}

#[wasm_bindgen(js_name = getState)]
pub fn get_gtate() -> String {
    TICTACTOE.with(|t| t.borrow().render_board(Some("\n")).to_string())
}

#[wasm_bindgen(js_name = checkPlayerWin)]
pub fn check_player_win() -> bool {
    TICTACTOE.with(|t| return t.borrow().check_player_win())
}

#[wasm_bindgen(js_name = getTurn)]
pub fn get_turn() -> i8 {
    TICTACTOE.with(|t| return t.borrow().turn)
}

#[wasm_bindgen(js_name = nextTurn)]
pub fn next_turn() {
    TICTACTOE.with(|t| {
        t.borrow_mut().next_turn();
    });
}

#[wasm_bindgen(js_name = clearGame)]
pub fn clear_game() {
    TICTACTOE.with(|t| {
        t.borrow_mut().clear();
    });
}

#[wasm_bindgen(js_name = takePlayerCell)]
pub fn take_player_cell(row: i8, col: i8) -> bool {
    TICTACTOE.with(|t| {
        return t.borrow_mut().player_take((row, col));
    })
}
