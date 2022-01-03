#![allow(warnings, unused)]

pub mod globs;
pub mod data;
pub mod board;
pub mod bitops;
pub mod movim;
pub mod displaymove;
pub mod gameline_h;
pub mod perft;
pub mod legalmoves;


pub fn new_board() -> board::Board {
    data::data_init();
	let board = board::Board::new();
    board
}


pub fn set_fen(board: &mut board::Board, fen_str: &str) {
    board.setup_fen(fen_str.to_string());
}


pub fn get_fen(board: &mut board::Board) -> String {
    board.get_fen_position()
}


pub fn get_truncated_fen(board: &mut board::Board) -> String {
    board.get_fen_truncated()
}


pub fn draw_board(board: &mut board::Board) {
    board.display();
}


pub fn move_uci(board: &mut board::Board, uci: &str) -> legalmoves::LegalMove {
    let mov = board.make_uci_move(uci);
    mov
}


pub fn move_san(board: &mut board::Board, san: &str) -> legalmoves::LegalMove {
    let mov = board.make_san_move(san);
    mov
}


pub fn undo_move(board: &mut board::Board, mov: legalmoves::LegalMove) {
    board.unmake_move(mov.movim);
}


pub fn game_ended(board: &mut board::Board) -> bool {
    board.is_end_of_game()
}

// intended for ui interface
pub fn graphical_board(board: &mut board::Board) -> Vec<&str> {
    
    let mut p: &str;
    let mut graph_board : Vec<&str> = Vec::new();

    for rank in (1..=8).rev(){
        for file in 1..= 8 {
            unsafe {
              p = globs::REAL_PIECENAMES[board.square[globs::BOARDINDEX[file][rank] as usize] as usize];
            }
            graph_board.push(p);
        }
    }
    graph_board
}


pub fn get_piece_at_alphasquare(board: &mut board::Board, square: &str) -> String {
	let idx: Option<usize>;
	idx = globs::SQUARENAME.iter().position(|&r| r == square);   // -> Option<usize>
	
	let piece: i32;
	match idx {
		Some(i) => {
			piece = board.square[i];
			match piece {
				0 => return "None".to_string(),
				//white pieces
				1 => return "P".to_string(),
				2 => return "K".to_string(),
				3 => return "N".to_string(),
				5 => return "B".to_string(),
				6 => return "R".to_string(),
				7 => return "Q".to_string(),
				//Black
				9 => return "p".to_string(),
				10 => return "k".to_string(),
				11 => return "n".to_string(),
				13 => return "b".to_string(),
				14 => return "r".to_string(),
				15 => return "q".to_string(),
				_  => return "None".to_string(),
			};
		},
		None => return "None".to_string(),
	};	
}

pub fn get_board_array(board: &mut board::Board) -> [i32; 64] {
	board.square
}