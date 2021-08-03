use super::movim::Movim;

#[derive(Clone, Copy)]
pub struct GameLineRecord {
       pub movim: Movim,
       pub castle_white: u8,     // White's castle status, CANCASTLEOO = 1, CANCASTLEOOO = 2
       pub castle_black: u8,     // Black's castle status, CANCASTLEOO = 1, CANCASTLEOOO = 2
       pub ep_square: i32,       // En-passant target square after double pawn move
       pub fifty_move: i32,      // Moves since the last pawn move or capture
       pub full_moves: i32,
}


impl GameLineRecord {
    pub fn new() -> Self {
        GameLineRecord {
            movim: Movim::new(),
            castle_white : 0,
            castle_black : 0,
            ep_square : 0,
            fifty_move : 0,
            full_moves: 0,
        }
    }
}