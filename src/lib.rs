mod game_engine;

pub use crate::game_engine::game_engine as game;

pub fn run() {

    let mut board = game::Board::new();
    board.print_board();
    board.player_move();
}