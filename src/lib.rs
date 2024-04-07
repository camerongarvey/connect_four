mod game_engine;

pub use crate::game_engine::game_engine as game;

pub fn run() {

    let mut board = game::Board::new();
    loop {
        board.print_board();
        if board.player_move() {break}
        if board.turn == 'y' {
            board.turn = 'r'
        } else {
            board.turn = 'y'
        }
    }
}