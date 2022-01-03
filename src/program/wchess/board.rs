use regex::Regex;

use super::globs as gl;
use super::globs::{EMPTY};
//use super::board_h::Board;
use super::bitops;
use super::movim;
use super::movim::Movim;
use super::gameline_h;

use super::globs::{BitMap, BOOLTYPE, MAX_MOV_BUFF, MAX_PLY, MAX_GAME_LINE};
//use super::movim::Movim;
use super::gameline_h::GameLineRecord;
use super::legalmoves;
use super::displaymove;

//BitMap es un u64, BOOLTYPE es un i32 (0 ->false, !=0 -> true) (globs.rs)
#[derive(Clone, Copy)]
pub struct Board {
    pub white_king: BitMap, 
    pub white_queens: BitMap, 
    pub white_rooks: BitMap, 
    pub white_bishops: BitMap, 
    pub white_knights: BitMap, 
    pub white_pawns: BitMap,

    pub black_king: BitMap, 
    pub black_queens: BitMap, 
    pub black_rooks: BitMap, 
    pub black_bishops: BitMap, 
    pub black_knights: BitMap, 
    pub black_pawns: BitMap,
    
    pub white_pieces: BitMap, 
    pub black_pieces: BitMap, 
    pub occupied_squares: BitMap,


    pub next_move: u8,        // WHITE_MOVE or BLACK_MOVE
    pub castle_white: u8,     // White's castle status, CANCASTLEOO = 1, CANCASTLEOOO = 2
    pub castle_black: u8,     // Black's castle status, CANCASTLEOO = 1, CANCASTLEOOO = 2
    pub ep_square: i32,                  // En-passant target square after double pawn move
    pub fifty_move: i32,                 // Moves since the last pawn move or capture
    pub full_moves: i32,                 // number of total half-moves

    pub square: [i32; 64],          // incrementally updated, this array is usefull if we want to
                                    // probe what kind of piece is on a particular square.
    pub view_rotated: BOOLTYPE,     // only used for displaying the board. TRUE or FALSE.

    // storing moves:
    pub move_buffer: [Movim; MAX_MOV_BUFF], // all generated moves of the current search tree are stored in this array.
    pub move_buf_len: [i32; MAX_PLY],       // this arrays keeps track of which moves belong to which ply

    pub end_of_game: usize,                 // index for board.gameLine
    pub end_of_search: usize,               // index for board.gameLine
    // almacena las jugadas de la partida
    pub game_line: [GameLineRecord; MAX_GAME_LINE],

}


impl Board {
    pub fn new() -> Self {
        let mut v_square: [i32; 64] = [
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32,
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32,
        ];

        // initial position
        v_square[gl::E1] = gl::WHITE_KING as i32;
        v_square[gl::D1] = gl::WHITE_QUEEN as i32;
        v_square[gl::A1] = gl::WHITE_ROOK as i32;
        v_square[gl::H1] = gl::WHITE_ROOK as i32;
        v_square[gl::B1] = gl::WHITE_KNIGHT as i32;
        v_square[gl::G1] = gl::WHITE_KNIGHT as i32;
        v_square[gl::C1] = gl::WHITE_BISHOP as i32;
        v_square[gl::F1] = gl::WHITE_BISHOP as i32;
        v_square[gl::A2] = gl::WHITE_PAWN as i32;
        v_square[gl::B2] = gl::WHITE_PAWN as i32;
        v_square[gl::C2] = gl::WHITE_PAWN as i32;
        v_square[gl::D2] = gl::WHITE_PAWN as i32;
        v_square[gl::E2] = gl::WHITE_PAWN as i32;
        v_square[gl::F2] = gl::WHITE_PAWN as i32;
        v_square[gl::G2] = gl::WHITE_PAWN as i32;
        v_square[gl::H2] = gl::WHITE_PAWN as i32;

        v_square[gl::E8] = gl::BLACK_KING as i32;
        v_square[gl::D8] = gl::BLACK_QUEEN as i32;
        v_square[gl::A8] = gl::BLACK_ROOK as i32;
        v_square[gl::H8] = gl::BLACK_ROOK as i32;
        v_square[gl::B8] = gl::BLACK_KNIGHT as i32;
        v_square[gl::G8] = gl::BLACK_KNIGHT as i32;
        v_square[gl::C8] = gl::BLACK_BISHOP as i32;
        v_square[gl::F8] = gl::BLACK_BISHOP as i32;
        v_square[gl::A7] = gl::BLACK_PAWN as i32;
        v_square[gl::B7] = gl::BLACK_PAWN as i32;
        v_square[gl::C7] = gl::BLACK_PAWN as i32;
        v_square[gl::D7] = gl::BLACK_PAWN as i32;
        v_square[gl::E7] = gl::BLACK_PAWN as i32;
        v_square[gl::F7] = gl::BLACK_PAWN as i32;
        v_square[gl::G7] = gl::BLACK_PAWN as i32;
        v_square[gl::H7] = gl::BLACK_PAWN as i32;

        let mov = Movim::new();
        let record_game = gameline_h::GameLineRecord::new();

        let mut new_board = Board {
            white_king: 0, 
            white_queens: 0, 
            white_rooks: 0, 
            white_bishops: 0, 
            white_knights: 0, 
            white_pawns: 0,

            black_king: 0, 
            black_queens: 0, 
            black_rooks: 0, 
            black_bishops: 0, 
            black_knights: 0, 
            black_pawns: 0,
            
            white_pieces: 0, 
            black_pieces: 0, 
            occupied_squares: 0,


            next_move: gl::WHITE_MOVE, 
            castle_white: gl::CANCASTLEOO + gl::CANCASTLEOOO,
            castle_black: gl::CANCASTLEOO + gl::CANCASTLEOOO,
            ep_square: 0, 
            fifty_move: 0, 
            full_moves: 1,

            // additional variables:
            view_rotated: false,
            square: v_square,   

            move_buffer: [mov.clone(); gl::MAX_MOV_BUFF], 
            move_buf_len: [0; gl::MAX_PLY],

            end_of_game: 0, 
            end_of_search: 0, 
            game_line: [record_game; gl::MAX_GAME_LINE],
        };

        new_board.init_from_squares (v_square, gl::WHITE_MOVE, 0, 0,
            (gl::CANCASTLEOO + gl::CANCASTLEOOO) as i32, 
            (gl::CANCASTLEOO + gl::CANCASTLEOOO) as i32, new_board.ep_square);

        new_board
    }


    fn init_from_squares (&mut self,
            input: [i32; 64], next: u8, fifty_m: i32, full_m: i32,
            castle_w: i32, castle_b: i32, ep_sq: i32) {

        // sets up the board variables according to the information found in
        // the input[64] array
        // All board & game initializations are done through this function (including readfen and setup).

        // bitboards
        self.white_king    = 0;
        self.white_queens  = 0;
        self.white_rooks   = 0;
        self.white_bishops = 0;
        self.white_knights = 0;
        self.white_pawns   = 0;
        self.black_king    = 0;
        self.black_queens  = 0;
        self.black_rooks   = 0;
        self.black_bishops = 0;
        self.black_knights = 0;
        self.black_pawns   = 0;
        self.white_pieces  = 0;
        self.black_pieces  = 0;
        self.occupied_squares = 0;

        // populate the 12 bitboard:
        for i in 0..64 {
            self.square[i] = input[i];
            // BITSET has all bits set that need to be set:
          unsafe {
            if self.square[i] == gl::WHITE_KING as i32   {self.white_king    = self.white_king    | gl::BITSET[i];}
            if self.square[i] == gl::WHITE_QUEEN as i32  {self.white_queens  = self.white_queens  | gl::BITSET[i];}
            if self.square[i] == gl::WHITE_ROOK as i32   {self.white_rooks   = self.white_rooks   | gl::BITSET[i];}
            if self.square[i] == gl::WHITE_BISHOP as i32 {self.white_bishops = self.white_bishops | gl::BITSET[i];}
            if self.square[i] == gl::WHITE_KNIGHT as i32 {self.white_knights = self.white_knights | gl::BITSET[i];}
            if self.square[i] == gl::WHITE_PAWN as i32   {self.white_pawns   = self.white_pawns   | gl::BITSET[i];}

            if self.square[i] == gl::BLACK_KING as i32   {self.black_king    = self.black_king    | gl::BITSET[i];}
            if self.square[i] == gl::BLACK_QUEEN as i32  {self.black_queens  = self.black_queens  | gl::BITSET[i];}
            if self.square[i] == gl::BLACK_ROOK as i32   {self.black_rooks   = self.black_rooks   | gl::BITSET[i];}
            if self.square[i] == gl::BLACK_BISHOP as i32 {self.black_bishops = self.black_bishops | gl::BITSET[i];}
            if self.square[i] == gl::BLACK_KNIGHT as i32 {self.black_knights = self.black_knights | gl::BITSET[i];}
            if self.square[i] == gl::BLACK_PAWN as i32   {self.black_pawns   = self.black_pawns   | gl::BITSET[i];}
          } // end unsafe
        }

        // queremos tener varios tableros de bits de todas las piezas blancas, negras y casillas ocupadas
        // entonces podemos obtener esto utilizando el operador OR de bits: 
        self.white_pieces = 
            self.white_king | self.white_queens | self.white_rooks | 
            self.white_bishops | self.white_knights | self.white_pawns;
        self.black_pieces = 
            self.black_king | self.black_queens | self.black_rooks | 
            self.black_bishops | self.black_knights | self.black_pawns;
        self.occupied_squares = self.white_pieces | self.black_pieces;

        self.next_move = next;
        self.castle_white = castle_w as u8;
        self.castle_black = castle_b as u8;
        self.ep_square = ep_sq;
        self.fifty_move = fifty_m;
        self.full_moves = full_m;
    }


    pub fn display(self) {
        //int rank, file;

        //std::cout << std::endl;
        {
            if !self.view_rotated {
                
                for rank in (1..=8).rev(){
                    println!( "    +---+---+---+---+---+---+---+---+" );
                    print!("  {} |", rank);
                    for file in 1..= 8 {
                        unsafe{
                        print!(" {}|", 
                            gl::PIECENAMES[self.square[gl::BOARDINDEX[file][rank] as usize] as usize]);
                        }
                    }
                    print!("\n");
                }
                println!( "    +---+---+---+---+---+---+---+---+" );
                println!( "      a   b   c   d   e   f   g   h" );
            }
            
            else {
                println!( "      h   g   f   e   d   c   b   a" );
                for rank in 1..= 8 {
                    println!( "    +---+---+---+---+---+---+---+---+" );
                    print!( "    |");
                    for file in (1..= 8).rev() {
                        unsafe {
                            print!(" {}|", 
                            gl::PIECENAMES[self.square[gl::BOARDINDEX[file][rank] as usize] as usize]);
                        }
                    }
                    println!("  {}", rank);
                }
                println!( "    +---+---+---+---+---+---+---+---+" );
            }
                
        }
    }


    pub fn get_fen_position(&mut self) -> String {
        let mut fen_str = "".to_string();
        let mut empties: i32 = 0;
        let mut p: &str;
        let mut row = 0;

        for rank in (1..=8).rev(){
            if empties != 0 {
                //fen_string += empties; // Add the empties number if it's not 0
                fen_str = format!("{}{}", fen_str, empties);  // Add the empties number if it's not 0
                empties = 0;
            }
            // Jump to the next rank
            if row > 0 && row < 8 {
                fen_str = format!("{}/", fen_str);   //"/";   // Add to mark a new rank, if we're not at the end
            }
            row += 1;
            for file in 1..= 8 {
                
                unsafe {
                  p = gl::REAL_PIECENAMES[self.square[gl::BOARDINDEX[file][rank] as usize] as usize];
                }
                if p == "*" {
                    empties +=1;
                }
                else {
                    if empties != 0 && p != "*" {
                        fen_str = format!("{}{}", fen_str, empties);
                        empties = 0;
                    }
                    
                    fen_str.push_str(p);
                }
            }
        }
        if empties != 0 {
            fen_str = format!("{}{}", fen_str, empties);
        }
        // turn
        if self.next_move == 0 {
            fen_str.push_str(" w ");
        }

        // castles white
        else { fen_str.push_str(" b "); }

        if self.castle_white == 1 {
            fen_str.push_str("K");
        }
        else if self.castle_white == 2 {
            fen_str.push_str("Q");
        }
        else if self.castle_white == 3 {
            fen_str.push_str("KQ");
        }
        // castles black
        if self.castle_black == 1 {
            fen_str.push_str("k");
        }
        else if self.castle_black == 2 {
            fen_str.push_str("q");
        }
        else if self.castle_black == 3 {
            fen_str.push_str("kq");
        }
        if self.castle_black == 0 && self.castle_white == 0 {
            fen_str.push_str("-");
        }


        fen_str.push_str(" ");

        // en passant
        if self.ep_square == 0 {
            fen_str.push_str("-");
        }
        else {
            if gl::FILES[self.ep_square as usize] == 1 { fen_str.push_str("a"); }
            if gl::FILES[self.ep_square as usize] == 2 { fen_str.push_str("b"); }
            if gl::FILES[self.ep_square as usize] == 3 { fen_str.push_str("c"); }
            if gl::FILES[self.ep_square as usize] == 4 { fen_str.push_str("d"); }
            if gl::FILES[self.ep_square as usize] == 5 { fen_str.push_str("e"); }
            if gl::FILES[self.ep_square as usize] == 6 { fen_str.push_str("f"); }
            if gl::FILES[self.ep_square as usize] == 7 { fen_str.push_str("g"); }
            if gl::FILES[self.ep_square as usize] == 8 { fen_str.push_str("h"); }

            fen_str = format!("{}{}", fen_str, gl::RANKS[self.ep_square as usize] );
        }

        fen_str.push_str(" ");

        // fifty moves
        fen_str = format!("{}{}", fen_str, self.fifty_move);
        fen_str.push_str(" ");
        // full half moves
        fen_str = format!("{}{}", fen_str, self.full_moves);

        fen_str
    }
	
	
	pub fn get_fen_truncated(&mut self) -> String {
        let mut fen_str = "".to_string();
        let mut empties: i32 = 0;
        let mut p: &str;
        let mut row = 0;

        for rank in (1..=8).rev(){
            if empties != 0 {
                //fen_string += empties; // Add the empties number if it's not 0
                fen_str = format!("{}{}", fen_str, empties);  // Add the empties number if it's not 0
                empties = 0;
            }
            // Jump to the next rank
            if row > 0 && row < 8 {
                fen_str = format!("{}/", fen_str);   //"/";   // Add to mark a new rank, if we're not at the end
            }
            row += 1;
            for file in 1..= 8 {
                
                unsafe {
                  p = gl::REAL_PIECENAMES[self.square[gl::BOARDINDEX[file][rank] as usize] as usize];
                }
                if p == "*" {
                    empties +=1;
                }
                else {
                    if empties != 0 && p != "*" {
                        fen_str = format!("{}{}", fen_str, empties);
                        empties = 0;
                    }
                    
                    fen_str.push_str(p);
                }
            }
        }
        if empties != 0 {
            fen_str = format!("{}{}", fen_str, empties);
        }
        fen_str
    }


    pub fn setup_fen (&mut self, new_fen: String) {
        let var_fen = new_fen.trim();
        let iter = var_fen.split_whitespace();
        let fen_parts = iter.collect::<Vec<&str>>();
        /*
            fen_parts[0] = pieces position
            fen_parts[1] = turn
            fen_parts[2] = castle rights
            fen_parts[3] = ep
            fen_parts[4] = number of moves since last pawn move
            fen_parts[5] = number of total half moves
        */
        //let mut piece: i32 = 0;
        let mut counter: i32 = 0;
        let mut file = 1;
        let mut rank = 8;

        // now board will be empty
        let mut v_square: [i32; 64] = [
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32,
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, 
            EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32, EMPTY as i32,
        ];

        // loop over the FEN string characters, and populate board.square[]
        // i is used as index for the FEN string
        // counter is the index for board.square[], 0..63
        // file and rank relate to the position on the chess board, 1..8
        // There is no error/legality checking on the FEN string!!

        let lista_car: Vec<char> = fen_parts[0].chars().collect();
        let mut caracter: char;
        let mut i: usize = 0;

        while (counter < 64) && (i < lista_car.len()) {
            caracter = lista_car[i];
            // ascii values
            if ((caracter as usize) > 48) && ((caracter as usize) < 57) {
                file += caracter as i32 - 48;
                counter += caracter as i32 - 48;
            }
            else {  //other characters:
                match caracter {
                    '/' => {
                        rank -= 1;
                        file = 1;
                    },
                    'P' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::WHITE_PAWN.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'N' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::WHITE_KNIGHT.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'B' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::WHITE_BISHOP.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'R' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::WHITE_ROOK.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'Q' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::WHITE_QUEEN.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'K' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::WHITE_KING.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'p' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::BLACK_PAWN.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'n' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::BLACK_KNIGHT.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'b' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::BLACK_BISHOP.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'r' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::BLACK_ROOK.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'q' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::BLACK_QUEEN.into();
                        }
                        file += 1;
                        counter += 1;
                    },
                    'k' => {
                        unsafe {
                            v_square[gl::BOARDINDEX[file as usize][rank as usize] as usize] = gl::BLACK_KING.into();
                        }
                        file += 1;
                        counter += 1;
                    },
					'?' => {    // this is for special searching of positions
                        file += 1;
                        counter += 1;
                    },
                    _ => (),
                };
            }
            i += 1;
        }
        
        let mut next = gl::WHITE_MOVE;
        if fen_parts[1] == "b" { next = gl::BLACK_MOVE;}

        let mut white_castle: i32 = 0;
        let mut black_castle: i32 = 0;
        if fen_parts[2].contains("K") {
            white_castle += gl::CANCASTLEOO as i32;
        }
        if fen_parts[2].contains("Q") {
            white_castle += gl::CANCASTLEOOO as i32;
        }
        if fen_parts[2].contains("k") {
            black_castle += gl::CANCASTLEOO as i32;
        }
        if fen_parts[2].contains("q") {
            black_castle += gl::CANCASTLEOOO as i32;
        }

        let ep_sq;
        if fen_parts[3] == "-" {
            ep_sq = 0;
        }
        else {
            // la casilla se calcula asi: CASILLA = 8 * FILA + COLUMNA - 9; 
            let cars: Vec<char> = fen_parts[3].chars().collect();
            let first_car = cars[0] as usize - 96;
            let second_car = cars[1] as usize - 48;
            ep_sq = first_car + 8 * second_car - 9;
        }

        let fenhalfmoveclock = fen_parts[4].parse::<i32>().unwrap();
        let fenfullmovenumber = fen_parts[5].parse::<i32>().unwrap();

        self.init_from_squares (v_square, next, fenhalfmoveclock, fenfullmovenumber,
            white_castle, black_castle, ep_sq as i32);
    }


    pub fn movegen (&mut self, idx: i32) -> usize {
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // This is winglet's pseudo-legal bitmap move generator,
        // using magic multiplication instead of rotated bitboards.
        // There is no check if a move leaves the king in check
        // The first free location in moveBuffer[] is supplied in index,
        // and the new first free location is returned
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

        let mut index: usize = idx as usize;

        let mut opponent_side: u8;
        let mut from: u32;
        let mut to: u32;
        let mut temp_piece: gl::BitMap;
        let mut temp_move: gl::BitMap;
        let mut target_bitmap: gl::BitMap;
        let mut free_squares: gl::BitMap;

        let mut movim: Movim = Movim::new();
        
        movim.clear();
        opponent_side = self.next_move ^ 1;
        free_squares = !self.occupied_squares;

        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        // Black to move
        // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

        if self.next_move == gl::BLACK_MOVE { // black to move
            target_bitmap = !self.black_pieces; // we cannot capture one of our own pieces!

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // Black Pawns
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::BLACK_PAWN as u32);
            temp_piece = self.black_pawns;
            while temp_piece != 0 {
              unsafe {
                from = bitops::first_one(temp_piece);
                movim.set_from(from);
                temp_move = gl::BLACK_PAWN_MOVES[from as usize] & free_squares; // normal moves
                if gl::RANKS[from as usize] == 7 && temp_move != 0 {
                    temp_move |= gl::BLACK_PAWN_DOUBLE_MOVES[from as usize] & free_squares;  // add double moves
                }
                temp_move |= gl::BLACK_PAWN_ATTACKS[from as usize] & self.white_pieces;       // add captures
                while temp_move != 0 {
                    to = bitops::first_one(temp_move);
                    movim.set_to_sq(to);
                    movim.set_capt(self.square[to as usize] as u32);
                    if (gl::RANKS[to as usize]) == 1 {                                      // add promotions
                            movim.set_prom(gl::BLACK_QUEEN as u32);   
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                            movim.set_prom(gl::BLACK_ROOK as u32);    
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                            movim.set_prom(gl::BLACK_BISHOP as u32);  
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                            movim.set_prom(gl::BLACK_KNIGHT as u32);  
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                            movim.set_prom(gl::EMPTY as u32);
                    }
                    else {
                        self.move_buffer[index].move_int = movim.move_int;
                        index += 1;
                    }
                    temp_move ^= gl::BITSET[to as usize];
                }
                // add en-passant captures:
                if self.ep_square != 0 {   // do a quick check first
                    if gl::BLACK_PAWN_ATTACKS[from as usize] & gl::BITSET[self.ep_square as usize] != 0 {
                            movim.set_prom(gl::BLACK_PAWN as u32);
                            movim.set_capt(gl::WHITE_PAWN as u32);
                            movim.set_to_sq(self.ep_square as u32);
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                        }
                    }
                temp_piece ^= gl::BITSET[from as usize];
                movim.set_prom(gl::EMPTY as u32);
              }  // end unsafe
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // Black Knights
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::BLACK_KNIGHT as u32);
            temp_piece = self.black_knights;
            while temp_piece != 0 {
              unsafe {
                from = bitops::first_one(temp_piece);
                movim.set_from(from);
                temp_move = gl::KNIGHT_ATTACKS[from as usize] & target_bitmap;
                while temp_move != 0 {
                    to = bitops::first_one(temp_move);
                    movim.set_to_sq(to);
                    movim.set_capt(self.square[to as usize] as u32);
                    self.move_buffer[index].move_int = movim.move_int;
                    index += 1;
                    temp_move ^= gl::BITSET[to as usize];
                }
                temp_piece ^= gl::BITSET[from as usize];
              }  // end unsafe
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // Black Bishops
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::BLACK_BISHOP as u32);
            temp_piece = self.black_bishops;
            while temp_piece != 0 {
              unsafe {
                from = bitops::first_one(temp_piece);
                movim.set_from(from);
                
                temp_move = movim::mov_bishopmoves(from, target_bitmap, self.occupied_squares);
                while temp_move != 0 {
                    to = bitops::first_one(temp_move);
                    movim.set_to_sq(to);
                    movim.set_capt(self.square[to as usize] as u32);
                    self.move_buffer[index].move_int = movim.move_int;
                    index +=1;
                    temp_move ^= gl::BITSET[to as usize];
                }
                temp_piece ^= gl::BITSET[from as usize];
              } // end unsafe
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // Black Rooks
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::BLACK_ROOK as u32);
            temp_piece = self.black_rooks;
            while temp_piece != 0 {
                unsafe {
                    from = bitops::first_one(temp_piece);
                    movim.set_from(from);
                    temp_move = movim::mov_rookmoves(from, target_bitmap, self.occupied_squares);
                    while temp_move != 0 {
                        to = bitops::first_one(temp_move);
                        movim.set_to_sq(to);
                        movim.set_capt(self.square[to as usize] as u32);
                        self.move_buffer[index].move_int = movim.move_int;
                        index += 1;
                        temp_move ^= gl::BITSET[to as usize];
                    }
                    temp_piece ^= gl::BITSET[from as usize];
                }
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // Black Queens
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::BLACK_QUEEN as u32);
            temp_piece = self.black_queens;
            while temp_piece != 0 {
                unsafe {
                    from = bitops::first_one(temp_piece);
                    movim.set_from(from);
                    temp_move = movim::mov_queenmoves(from, target_bitmap, self.occupied_squares); 
                    while temp_move != 0 {
                        to = bitops::first_one(temp_move);
                        movim.set_to_sq(to);
                        movim.set_capt(self.square[to as usize] as u32);
                        self.move_buffer[index].move_int = movim.move_int;
                        index += 1;
                        temp_move ^= gl::BITSET[to as usize];
                    }
                    temp_piece ^= gl::BITSET[from as usize];
                }
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // Black King
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::BLACK_KING as u32);
            temp_piece = self.black_king;
            while temp_piece != 0 {
              unsafe {
                from = bitops::first_one(temp_piece);
                movim.set_from(from);
                temp_move = gl::KING_ATTACKS[from as usize] & target_bitmap;
                while temp_move != 0 {
                    to = bitops::first_one(temp_move);
                    movim.set_to_sq(to);
                    movim.set_capt(self.square[to as usize] as u32);
                    self.move_buffer[index].move_int = movim.move_int;
                    index += 1;
                    temp_move ^= gl::BITSET[to as usize];
                }

                //     Black 0-0 Castling:
                if self.castle_black & gl::CANCASTLEOO != 0 {
                    if (gl::MASK_FG[1] & self.occupied_squares) == 0 {
                        if !self.is_attacked(gl::MASK_EG[gl::BLACK_MOVE as usize], gl::WHITE_MOVE) {
                            self.move_buffer[index].move_int = gl::BLACK_OO_CASTL as u32;   // predefined unsigned int
                            index += 1;
                        }
                    }
                }
                //     Black 0-0-0 Castling:
                if self.castle_black & gl::CANCASTLEOOO != 0{
                    if (gl::MASK_BD[1] & self.occupied_squares) == 0 {
                        if !self.is_attacked(gl::MASK_CE[gl::BLACK_MOVE as usize], gl::WHITE_MOVE) {
                            self.move_buffer[index].move_int = gl::BLACK_OOO_CASTL as u32; // predefined unsigned int
                            index += 1;
                        }
                    }
                }
                temp_piece ^= gl::BITSET[from as usize];
                movim.set_prom(gl::EMPTY as u32);
              } // end unsafe
            }

        }
        // white to move
        else {
            target_bitmap = !self.white_pieces; // we cannot capture one of our own pieces!

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // White Pawns
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::WHITE_PAWN as u32);
            temp_piece = self.white_pawns;
            while temp_piece != 0 {
                unsafe {
                    from = bitops::first_one(temp_piece);
                    movim.set_from(from);
                    temp_move = gl::WHITE_PAWN_MOVES[from as usize] & free_squares;   // normal moves
                    if gl::RANKS[from as usize] == 2 && temp_move != 0 {
                        temp_move |= gl::WHITE_PAWN_DOUBLE_MOVES[from as usize] & free_squares;  // add double moves
                    }
                    temp_move |= gl::WHITE_PAWN_ATTACKS[from as usize] & self.black_pieces;       // add captures
                    while temp_move != 0 {
                        to = bitops::first_one(temp_move);
                        movim.set_to_sq(to);
                        movim.set_capt(self.square[to as usize] as u32);
                        if gl::RANKS[to as usize] == 8 {                                       // add promotions
                            movim.set_prom(gl::WHITE_QUEEN as u32);   
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                            movim.set_prom(gl::WHITE_ROOK as u32);    
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                            movim.set_prom(gl::WHITE_BISHOP as u32);  
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                            movim.set_prom(gl::WHITE_KNIGHT as u32);  
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                            movim.set_prom(gl::EMPTY as u32);
                        }
                        else
                        {
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                        }
                        temp_move ^= gl::BITSET[to as usize];
                    }
                    // add en-passant captures:
                    if self.ep_square != 0 {   // do a quick check first
                        if gl::WHITE_PAWN_ATTACKS[from as usize] & gl::BITSET[self.ep_square as usize] != 0 {
                            movim.set_prom(gl::WHITE_PAWN as u32);
                            movim.set_capt(gl::BLACK_PAWN as u32);
                            movim.set_to_sq(self.ep_square as u32);
                            self.move_buffer[index].move_int = movim.move_int;
                            index += 1;
                        }
                    }
                    temp_piece ^= gl::BITSET[from as usize];
                    movim.set_prom(gl::EMPTY as u32);
                } // end unsafe
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // White Knights
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::WHITE_KNIGHT as u32);
            temp_piece = self.white_knights;
            while temp_piece != 0 {
                unsafe {
                    from = bitops::first_one(temp_piece);
                    movim.set_from(from);
                    temp_move = gl::KNIGHT_ATTACKS[from as usize] & target_bitmap;
                    while temp_move != 0 {
                        to = bitops::first_one(temp_move);
                        movim.set_to_sq(to);
                        movim.set_capt(self.square[to as usize] as u32);
                        self.move_buffer[index].move_int = movim.move_int;
                        index += 1;
                        temp_move ^= gl::BITSET[to as usize];
                    }
                    temp_piece ^= gl::BITSET[from as usize];
                }  // end unsafe;
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // White Bishops
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::WHITE_BISHOP as u32);
            temp_piece = self.white_bishops;
            while temp_piece != 0 {
                unsafe {
                    from = bitops::first_one(temp_piece);
                    movim.set_from(from);
                    temp_move = movim::mov_bishopmoves(from, target_bitmap, self.occupied_squares);
                    while temp_move != 0 {
                        to = bitops::first_one(temp_move);
                        movim.set_to_sq(to);
                        movim.set_capt(self.square[to as usize] as u32);
                        self.move_buffer[index].move_int = movim.move_int;
                        index += 1;
                        temp_move ^= gl::BITSET[to as usize];
                    }
                    temp_piece ^= gl::BITSET[from as usize];
                }  // end unsafe
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // White Rooks
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::WHITE_ROOK as u32);
            temp_piece = self.white_rooks;
            while temp_piece != 0 {
                unsafe {
                    from = bitops::first_one(temp_piece);
                    movim.set_from(from);
                    temp_move = movim::mov_rookmoves(from, target_bitmap, self.occupied_squares);
                    while temp_move != 0 {
                        to = bitops::first_one(temp_move);
                        movim.set_to_sq(to);
                        movim.set_capt(self.square[to as usize] as u32);
                        self.move_buffer[index].move_int = movim.move_int;
                        index += 1;
                        temp_move ^= gl::BITSET[to as usize];
                    }
                    temp_piece ^= gl::BITSET[from as usize];
                }  // end unsafe
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // White Queens
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::WHITE_QUEEN as u32);
            temp_piece = self.white_queens;
            while temp_piece != 0 {
                unsafe {
                    from = bitops::first_one(temp_piece);
                    movim.set_from(from);
                    temp_move = movim::mov_queenmoves(from, target_bitmap, self.occupied_squares);
                    while temp_move != 0 {
                        to = bitops::first_one(temp_move);
                        movim.set_to_sq(to);
                        movim.set_capt(self.square[to as usize] as u32);
                        self.move_buffer[index].move_int = movim.move_int;
                        index += 1;
                        temp_move ^= gl::BITSET[to as usize];
                    }
                    temp_piece ^= gl::BITSET[from as usize];
                }  // end unsafe
            }

            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            // White king
            // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            movim.set_piec(gl::WHITE_KING as u32);
            temp_piece = self.white_king;
            while temp_piece != 0 {
                unsafe {
                    from = bitops::first_one(temp_piece);
                    movim.set_from(from);
                    temp_move = gl::KING_ATTACKS[from as usize] & target_bitmap;
                    while temp_move != 0 {
                        to = bitops::first_one(temp_move);
                        movim.set_to_sq(to);
                        movim.set_capt(self.square[to as usize] as u32);
                        self.move_buffer[index].move_int = movim.move_int;
                        index += 1;
                        temp_move ^= gl::BITSET[to as usize];
                    }

                    //     White 0-0 Castling:
                    if self.castle_white & gl::CANCASTLEOO != 0 {
                        if (gl::MASK_FG[0] & self.occupied_squares) == 0 {
                            if !self.is_attacked(gl::MASK_EG[gl::WHITE_MOVE as usize], gl::BLACK_MOVE) {
                                self.move_buffer[index].move_int = gl::WHITE_OO_CASTL as u32; // predefined unsigned int
                                index += 1;
                            }
                        }
                    }

                    //     White 0-0-0 Castling:
                    if self.castle_white & gl::CANCASTLEOOO != 0 {
                        if (gl::MASK_BD[0] & self.occupied_squares) == 0 {
                            if !self.is_attacked(gl::MASK_CE[gl::WHITE_MOVE as usize], gl::BLACK_MOVE) {
                                self.move_buffer[index].move_int = gl::WHITE_OOO_CASTL as u32; // predefined unsigned int
                                index += 1;
                            }
                        }
                    }
                    temp_piece ^= gl::BITSET[from as usize];
                    movim.set_prom(gl::EMPTY as u32);
                }  // end unsafe
            }
        }



        index
    }


    fn is_attacked(self, target_bitmap: gl::BitMap, from_side: u8) -> bool {
    
    //     ===========================================================================
    //     isAttacked is used mainly as a move legality test to see if targetBitmap is
    //     attacked by white or black.
    //  Returns true at the first attack found, and returns false if no attack is found.
    //  It can be used for:
    //   - check detection, and
    //   - castling legality: test to see if the king passes through, or ends up on,
    //     a square that is attacked
    //     ===========================================================================
    
        let mut temp_target: gl::BitMap;
        let mut sliding_attackers: gl::BitMap;
        //int to;

        temp_target = target_bitmap;
        if from_side != 0 {  // test for attacks from BLACK to targetBitmap
          unsafe {
            while temp_target != 0 {
                let to = bitops::first_one(temp_target);

                if self.black_pawns & gl::WHITE_PAWN_ATTACKS[to as usize] != 0 { return true; }
                if self.black_knights & gl::KNIGHT_ATTACKS[to as usize] != 0 { return true; }
                if self.black_king & gl::KING_ATTACKS[to as usize] != 0 { return true; }

                // file / rank attacks
                sliding_attackers = self.black_queens | self.black_rooks;
                if sliding_attackers != 0 {
                    if gl::RANK_ATTACKS[to as usize]
                        [(((self.occupied_squares & gl::RANKMASK[to as usize]) as usize) >> gl::RANKSHIFT[to as usize])] 
                        & sliding_attackers != 0 {
                        return true;
                    }
                    if gl::FILE_ATTACKS[to as usize]
                          [(((self.occupied_squares & gl::FILEMASK[to as usize]) as i128 * 
                                gl::FILEMAGIC[to as usize] as i128) as u64 >> 57) as usize] 
                          & sliding_attackers != 0 {
                        return true;
                    }
                }

                // diagonals
                sliding_attackers = self.black_queens | self.black_bishops;
                if sliding_attackers != 0 {
                    if gl::DIAGA8H1_ATTACKS[to as usize]
                    [(((self.occupied_squares & gl::DIAGA8H1MASK[to as usize]) as i128 * 
                        gl::DIAGA8H1MAGIC[to as usize] as i128) as u64 >> 57) as usize] 
                    & sliding_attackers != 0 {
                        return true;
                    }
                    if gl::DIAGA1H8_ATTACKS[to as usize]
                    [(((self.occupied_squares & gl::DIAGA1H8MASK[to as usize]) as i128 * 
                            gl::DIAGA1H8MAGIC[to as usize] as i128) as u64 >> 57) as usize] 
                        & sliding_attackers != 0 {
                        return true;
                    }
                }

                temp_target ^= gl::BITSET[to as usize];
            }
          }  // end unsafe
        }
        else {  // test for attacks from WHITE to targetBitmap
            while temp_target != 0 {
              unsafe {
                let to = bitops::first_one(temp_target);

                if self.white_pawns & gl::BLACK_PAWN_ATTACKS[to as usize] != 0 { return true; }
                if self.white_knights & gl::KNIGHT_ATTACKS[to as usize] != 0 { return true; }
                if self.white_king & gl::KING_ATTACKS[to as usize] != 0 { return true; }

                // file / rank attacks
                sliding_attackers = self.white_queens | self.white_rooks;
                if sliding_attackers != 0 {
                    if gl::RANK_ATTACKS[to as usize]
                    [((self.occupied_squares & gl::RANKMASK[to as usize]) >> gl::RANKSHIFT[to as usize]) as usize] 
                    & sliding_attackers != 0 {
                        return true;
                    }
                    if gl::FILE_ATTACKS[to as usize]
                    [((((self.occupied_squares & gl::FILEMASK[to as usize]) as i128 * 
                        gl::FILEMAGIC[to as usize] as i128)) as u64 >> 57) as usize] 
                    & sliding_attackers != 0 {
                        return true;
                    }
                }

                // diagonals:
                sliding_attackers = self.white_queens | self.white_bishops;
                if sliding_attackers != 0 {
                    if gl::DIAGA8H1_ATTACKS[to as usize]
                    [(((self.occupied_squares & gl::DIAGA8H1MASK[to as usize]) as i128 * 
                        gl::DIAGA8H1MAGIC[to as usize] as i128) as u64 >> 57) as usize] 
                    & sliding_attackers != 0 {
                        return true;
                    }
                    if gl::DIAGA1H8_ATTACKS[to as usize]
                    [(((self.occupied_squares & gl::DIAGA1H8MASK[to as usize]) as i128 * 
                        gl::DIAGA1H8MAGIC[to as usize] as i128) as u64 >> 57) as usize] 
                    & sliding_attackers != 0 {
                        return true;
                    }
                }

                temp_target ^= gl::BITSET[to as usize];
              }  // end unsafe
            }
        }
        return false;
    }


    pub fn make_move(&mut self, movim: movim::Movim) {
        let from: u32 = movim.get_from();
        let to: u32 = movim.get_to_sq();
        let piece: u32 = movim.get_piec();
        let captured: u32 = movim.get_capt();

        let from_bit_map: gl::BitMap;
        let from_to_bit_map: gl::BitMap;

        unsafe {
            from_bit_map = gl::BITSET[from as usize];
            from_to_bit_map = from_bit_map  | gl::BITSET[to as usize];
        }

        self.game_line[self.end_of_search].movim.move_int = movim.move_int;
        self.game_line[self.end_of_search].castle_white   = self.castle_white;
        self.game_line[self.end_of_search].castle_black   = self.castle_black;
        self.game_line[self.end_of_search].fifty_move     = self.fifty_move;
        self.game_line[self.end_of_search].ep_square      = self.ep_square;
        self.game_line[self.end_of_search].full_moves     = self.full_moves;
        self.end_of_search += 1;

      unsafe {
        match piece {
        
            1 => { // white pawn:
                self.white_pawns           ^= from_to_bit_map;
                self.white_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::WHITE_PAWN as i32;
                self.ep_square            = 0;
                self.fifty_move           = 0;
                //self.full_moves          += 1;
                if gl::RANKS[from as usize] == 2 {
                    if gl::RANKS[to as usize] == 4 { self.ep_square = from as i32 + 8; }
                }
                if captured != 0 {
                    if movim.is_enpassant() {
                        self.black_pawns              ^= gl::BITSET[(to-8) as usize];
                        self.black_pieces             ^= gl::BITSET[(to-8) as usize];
                        self.occupied_squares         ^= (from_to_bit_map | gl::BITSET[(to-8) as usize]) as u64;
                        self.square[(to-8) as usize]   = gl::EMPTY as i32;
                    }
                    else
                    {
                        self.make_capture(captured, to);
                        self.occupied_squares ^= from_bit_map;
                    }
                }
                else { self.occupied_squares ^= from_to_bit_map; }

                if movim.is_promotion() {
                    self.make_white_promotion (movim.get_prom(), to);
                    self.square[to as usize]  = movim.get_prom() as i32;
                }
            },

            2 => { // white king:
                self.white_king             ^= from_to_bit_map;
                self.white_pieces           ^= from_to_bit_map;
                self.square[from as usize]   = gl::EMPTY as i32;
                self.square[to as usize]     = gl::WHITE_KING as i32;
                self.ep_square               = 0;
                self.fifty_move             += 1;
                //self.full_moves             += 1;
                self.castle_white            = 0;
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }

                if movim.is_castle() {
                    if movim.is_castle_oo() {
                        self.white_rooks         ^= gl::BITSET[gl::H1] | gl::BITSET[gl::F1];
                        self.white_pieces        ^= gl::BITSET[gl::H1] | gl::BITSET[gl::F1];
                        self.occupied_squares    ^= gl::BITSET[gl::H1] | gl::BITSET[gl::F1];
                        self.square[gl::H1]       = gl::EMPTY as i32;
                        self.square[gl::F1]       = gl::WHITE_ROOK as i32;
                    }
                    else {
                        self.white_rooks         ^= gl::BITSET[gl::A1] | gl::BITSET[gl::D1];
                        self.white_pieces        ^= gl::BITSET[gl::A1] | gl::BITSET[gl::D1];
                        self.occupied_squares    ^= gl::BITSET[gl::A1] | gl::BITSET[gl::D1];
                        self.square[gl::A1]       = gl::EMPTY as i32;
                        self.square[gl::D1]       = gl::WHITE_ROOK as i32;
                    }
                }
            },

            3 => { // white knight:
                self.white_knights         ^= from_to_bit_map;
                self.white_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::WHITE_KNIGHT as i32;
                self.ep_square            = 0;
                self.fifty_move          += 1;
                //self.full_moves          += 1;
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            5 => { // white bishop:
                self.white_bishops         ^= from_to_bit_map;
                self.white_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::WHITE_BISHOP as i32;
                self.ep_square              = 0;
                self.fifty_move          += 1;
                //self.full_moves          += 1;
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            6 => { // white rook:
                self.white_rooks            ^= from_to_bit_map;
                self.white_pieces           ^= from_to_bit_map;
                self.square[from as usize]   = gl::EMPTY as i32;
                self.square[to as usize]     = gl::WHITE_ROOK as i32;
                self.ep_square               = 0;
                self.fifty_move          += 1;
                //self.full_moves          += 1;
                if from == gl::A1 as u32 { self.castle_white &= !gl::CANCASTLEOOO; }
                if from == gl::H1 as u32 { self.castle_white &= !gl::CANCASTLEOO; }
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            7 => { // white queen:
                self.white_queens          ^= from_to_bit_map;
                self.white_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::WHITE_QUEEN as i32;
                self.ep_square            = 0;
                self.fifty_move          += 1;
                //self.full_moves          += 1;
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            9 => { // black pawn:
                self.black_pawns           ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::BLACK_PAWN as i32;
                self.ep_square            = 0;
                self.fifty_move           = 0;
                self.full_moves          += 1;
                if gl::RANKS[from as usize] == 7 {
                    if gl::RANKS[to as usize] == 5 { self.ep_square = from as i32 - 8; }
                }
                if captured != 0 {
                    if movim.is_enpassant() {
                            self.white_pawns           ^= gl::BITSET[(to+8) as usize];
                            self.white_pieces          ^= gl::BITSET[(to+8) as usize];
                            self.occupied_squares    ^= from_to_bit_map | gl::BITSET[(to+8) as usize];
                            self.square[(to+8) as usize]       = gl::EMPTY as i32;
                    }
                    else {
                            self.make_capture(captured, to);
                            self.occupied_squares ^= from_bit_map;
                    }
                }
                else { self.occupied_squares ^= from_to_bit_map; }

                if movim.is_promotion() {
                    self.make_black_promotion(movim.get_prom(), to);
                    self.square[to as usize]  = movim.get_prom() as i32;
                }
            },

            10 => { // black king:
                self.black_king             ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::BLACK_KING as i32;
                self.ep_square            = 0;
                self.fifty_move          += 1;
                self.full_moves          += 1;
                self.castle_black = 0;
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }

                if movim.is_castle() {
                    if movim.is_castle_oo() {
                        self.black_rooks         ^= gl::BITSET[gl::H8] | gl::BITSET[gl::F8];
                        self.black_pieces        ^= gl::BITSET[gl::H8] | gl::BITSET[gl::F8];
                        self.occupied_squares    ^= gl::BITSET[gl::H8] | gl::BITSET[gl::F8];
                        self.square[gl::H8]       = gl::EMPTY as i32;
                        self.square[gl::F8]       = gl::BLACK_ROOK as i32;
                    }
                    else {
                        self.black_rooks         ^= gl::BITSET[gl::A8] | gl::BITSET[gl::D8];
                        self.black_pieces        ^= gl::BITSET[gl::A8] | gl::BITSET[gl::D8];
                        self.occupied_squares    ^= gl::BITSET[gl::A8] | gl::BITSET[gl::D8];
                        self.square[gl::A8]       = gl::EMPTY as i32;
                        self.square[gl::D8]       = gl::BLACK_ROOK as i32;
                    }
                }
            },

            11 => { // black knight:
                self.black_knights         ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::BLACK_KNIGHT as i32;
                self.ep_square            = 0;
                self.fifty_move          += 1;
                self.full_moves          += 1;
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            13 => { // black bishop:
                self.black_bishops         ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::BLACK_BISHOP as i32;
                self.ep_square            = 0;
                self.fifty_move          += 1;
                self.full_moves          += 1;
                if captured != 0 
                {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            14 => { // black rook:
                self.black_rooks           ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::BLACK_ROOK as i32;
                self.ep_square            = 0;
                self.fifty_move          += 1;
                self.full_moves          += 1;
                if from == gl::A8 as u32 { self.castle_black &= !gl::CANCASTLEOOO; }
                if from == gl::H8 as u32 { self.castle_black &= !gl::CANCASTLEOO; }
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            15 => { // black queen:
                self.black_queens          ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::EMPTY as i32;
                self.square[to as usize]    = gl::BLACK_QUEEN as i32;
                self.ep_square            = 0;
                self.fifty_move          += 1;
                self.full_moves          += 1;
                if captured != 0 {
                    self.make_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            _=> (),
        };
      }  // end unsafe

        self.next_move = self.next_move ^ 1;
    }


    pub fn unmake_move(&mut self, movim: Movim) {
        let piece: u32 = movim.get_piec();
        let captured: u32 = movim.get_capt();
        let from: u32 = movim.get_from();
        let to: u32 = movim.get_to_sq();

        let from_bit_map: gl::BitMap;
        let from_to_bit_map: gl::BitMap;
        unsafe {
            from_bit_map  = gl::BITSET[from as usize];
            from_to_bit_map = from_bit_map  | gl::BITSET[to as usize];
        }
      unsafe {
        match piece {
            1 => { // white pawn:
                self.white_pawns           ^= from_to_bit_map;
                self.white_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::WHITE_PAWN as i32;
                self.square[to as usize]    = gl::EMPTY as i32;
                if captured != 0 {
                    if movim.is_enpassant() {
                        self.black_pawns            ^= gl::BITSET[(to-8) as usize];
                        self.black_pieces           ^= gl::BITSET[(to-8) as usize];
                        self.occupied_squares       ^= from_to_bit_map | gl::BITSET[(to-8) as usize];
                        self.square[(to-8) as usize] = gl::BLACK_PAWN as i32;
                    }
                    else
                    {
                        self.unmake_capture(captured, to);
                        self.occupied_squares ^= from_bit_map;
                    }
                }
                else { self.occupied_squares ^= from_to_bit_map; }

                if movim.is_promotion() {
                    self.unmake_white_promotion(movim.get_prom(), to);
                }
            },

            2 => { // white king:
                self.white_king             ^= from_to_bit_map;
                self.white_pieces           ^= from_to_bit_map;
                self.square[from as usize]   = gl::WHITE_KING as i32;
                self.square[to as usize]     = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }

                if movim.is_castle() {
                    if movim.is_castle_oo() {
                        self.white_rooks         ^= gl::BITSET[gl::H1] | gl::BITSET[gl::F1];
                        self.white_pieces        ^= gl::BITSET[gl::H1] | gl::BITSET[gl::F1];
                        self.occupied_squares    ^= gl::BITSET[gl::H1] | gl::BITSET[gl::F1];
                        self.square[gl::H1]       = gl::WHITE_ROOK as i32;
                        self.square[gl::F1]       = gl::EMPTY as i32;
                    }
                    else {
                        self.white_rooks         ^= gl::BITSET[gl::A1] | gl::BITSET[gl::D1];
                        self.white_pieces        ^= gl::BITSET[gl::A1] | gl::BITSET[gl::D1];
                        self.occupied_squares    ^= gl::BITSET[gl::A1] | gl::BITSET[gl::D1];
                        self.square[gl::A1]       = gl::WHITE_ROOK as i32;
                        self.square[gl::D1]       = gl::EMPTY as i32;
                    }
                }
            },

            3 => { // white knight:
                self.white_knights            ^= from_to_bit_map;
                self.white_pieces            ^= from_to_bit_map;
                self.square[from  as usize]    = gl::WHITE_KNIGHT as i32;
                self.square[to as usize]       = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            5 => { // white bishop:
                self.white_bishops         ^= from_to_bit_map;
                self.white_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::WHITE_BISHOP as i32;
                self.square[to as usize]    = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            6 => { // white rook:
                self.white_rooks           ^= from_to_bit_map;
                self.white_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::WHITE_ROOK as i32;
                self.square[to as usize]    = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            7 => { // white queen:
                self.white_queens           ^= from_to_bit_map;
                self.white_pieces           ^= from_to_bit_map;
                self.square[from as usize]   = gl::WHITE_QUEEN as i32;
                self.square[to as usize]     = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            9 => { // black pawn:
                self.black_pawns            ^= from_to_bit_map;
                self.black_pieces           ^= from_to_bit_map;
                self.square[from as usize]   = gl::BLACK_PAWN as i32;
                self.square[to as usize]     = gl::EMPTY as i32;
                if captured != 0 {
                    if movim.is_enpassant() {
                        self.white_pawns              ^= gl::BITSET[(to+8) as usize];
                        self.white_pieces             ^= gl::BITSET[(to+8) as usize];
                        self.occupied_squares         ^= from_to_bit_map | gl::BITSET[(to+8) as usize];
                        self.square[(to+8) as usize]   = gl::WHITE_PAWN as i32;
                    }
                    else {
                        self.unmake_capture(captured, to);
                        self.occupied_squares ^= from_bit_map;
                    }
                }
                else { self.occupied_squares ^= from_to_bit_map; }

                if movim.is_promotion() {
                    self.unmake_black_promotion(movim.get_prom(), to);
                }
            },

            10 => { // black king:
                self.black_king             ^= from_to_bit_map;
                self.black_pieces           ^= from_to_bit_map;
                self.square[from as usize]   = gl::BLACK_KING as i32;
                self.square[to as usize]     = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }

                if movim.is_castle() {
                    if movim.is_castle_oo() {
                        self.black_rooks         ^= gl::BITSET[gl::H8] | gl::BITSET[gl::F8];
                        self.black_pieces        ^= gl::BITSET[gl::H8] | gl::BITSET[gl::F8];
                        self.occupied_squares    ^= gl::BITSET[gl::H8] | gl::BITSET[gl::F8];
                        self.square[gl::H8]       = gl::BLACK_ROOK as i32;
                        self.square[gl::F8]       = gl::EMPTY as i32;
                    }
                    else
                    {
                        self.black_rooks       ^= gl::BITSET[gl::A8] | gl::BITSET[gl::D8];
                        self.black_pieces      ^= gl::BITSET[gl::A8] | gl::BITSET[gl::D8];
                        self.occupied_squares  ^= gl::BITSET[gl::A8] | gl::BITSET[gl::D8];
                        self.square[gl::A8]     = gl::BLACK_ROOK as i32;
                        self.square[gl::D8]     = gl::EMPTY as i32;
                    }
                }
            },

            11 => { // black knight:
                self.black_knights         ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::BLACK_KNIGHT as i32;
                self.square[to as usize]    = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            13 => { // black bishop:
                self.black_bishops         ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::BLACK_BISHOP as i32;
                self.square[to as usize]    = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            14 => { // black rook:
                self.black_rooks           ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::BLACK_ROOK as i32;
                self.square[to as usize]    = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            15 => { // black queen:
                self.black_queens          ^= from_to_bit_map;
                self.black_pieces          ^= from_to_bit_map;
                self.square[from as usize]  = gl::BLACK_QUEEN as i32;
                self.square[to as usize]    = gl::EMPTY as i32;
                if captured != 0 {
                    self.unmake_capture(captured, to);
                    self.occupied_squares ^= from_bit_map;
                }
                else { self.occupied_squares ^= from_to_bit_map; }
            },

            _ => (),
        };
      }  // end unsafe

        self.end_of_search -= 1;
        self.castle_white         = self.game_line[self.end_of_search].castle_white;
        self.castle_black         = self.game_line[self.end_of_search].castle_black;
        self.ep_square            = self.game_line[self.end_of_search].ep_square;
        self.fifty_move           = self.game_line[self.end_of_search].fifty_move;
        self.full_moves           = self.game_line[self.end_of_search].full_moves;

        self.next_move = self.next_move ^ 1;


    }


    fn make_capture(&mut self, captured: u32, to: u32) {
           // deals with all captures, except en-passant
           let to_bit_map: gl::BitMap;
           unsafe { to_bit_map = gl::BITSET[to as usize]; }
    
           match captured {
                1 => { // white pawn:
                    self.white_pawns           ^= to_bit_map;
                    self.white_pieces          ^= to_bit_map;
                },
                2 => { // white king:
                    self.white_king             ^= to_bit_map;
                    self.white_pieces          ^= to_bit_map;
                },
                3 => { // white knight:
                    self.white_knights         ^= to_bit_map;
                    self.white_pieces          ^= to_bit_map;
                },
                5 => { // white bishop:
                    self.white_bishops         ^= to_bit_map;
                    self.white_pieces          ^= to_bit_map;
                },
                6 => { // white rook:
                    self.white_rooks         ^= to_bit_map;
                    self.white_pieces          ^= to_bit_map;
                    
                    if to == gl::A1 as u32 { self.castle_white &= !gl::CANCASTLEOOO; }
                    if to == gl::H1 as u32 { self.castle_white &= !gl::CANCASTLEOO; }
                },
                7 => { // white queen:
                    self.white_queens          ^= to_bit_map;
                    self.white_pieces          ^= to_bit_map;
                },
                9 => { // black pawn:
                    self.black_pawns           ^= to_bit_map;
                    self.black_pieces          ^= to_bit_map;
                },
                10 => { // black king:
                    self.black_king             ^= to_bit_map;
                    self.black_pieces          ^= to_bit_map;
                },
                11 => { // black knight:
                    self.black_knights         ^= to_bit_map;
                    self.black_pieces          ^= to_bit_map;
                },
                13 => { // black bishop:
                    self.black_bishops         ^= to_bit_map;
                    self.black_pieces          ^= to_bit_map;
                },
                14 => { // black rook:
                    self.black_rooks         ^= to_bit_map;
                    self.black_pieces          ^= to_bit_map;
                    
                    if to == gl::A8 as u32 { self.castle_black &= !gl::CANCASTLEOOO; }
                    if to == gl::H8 as u32 { self.castle_black &= !gl::CANCASTLEOO; }
                },
                15 => { // black queen:
                    self.black_queens          ^= to_bit_map;
                    self.black_pieces          ^= to_bit_map;
                },
                _ => (),
           };
           self.fifty_move = 0;
    }



    fn unmake_capture(&mut self, captured: u32, to: u32) {
        // deals with all captures, except en-passant
        
        let to_bit_map: gl::BitMap;
        unsafe { to_bit_map = gl::BITSET[to as usize]; }

        match captured {
            1 => { // white pawn:
                self.white_pawns           ^= to_bit_map;
                self.white_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::WHITE_PAWN as i32;
            },

            2 => { // white king:
                self.white_king          ^= to_bit_map;
                self.white_pieces        ^= to_bit_map;
                self.square[to as usize]  = gl::WHITE_KING as i32;
            },

            3 => { // white knight:
                self.white_knights         ^= to_bit_map;
                self.white_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::WHITE_KNIGHT as i32;
            },

            5 => { // white bishop:
                self.white_bishops         ^= to_bit_map;
                self.white_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::WHITE_BISHOP as i32;
            },

            6 => { // white rook:
                self.white_rooks         ^= to_bit_map;
                self.white_pieces        ^= to_bit_map;
                self.square[to as usize]  = gl::WHITE_ROOK as i32;
            },

            7 => { // white queen:
                self.white_queens          ^= to_bit_map;
                self.white_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::WHITE_QUEEN as i32;
            },

            9 => { // black pawn:
                self.black_pawns           ^= to_bit_map;
                self.black_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::BLACK_PAWN as i32;
            },

            10 => { // black king:
                self.black_king            ^= to_bit_map;
                self.black_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::BLACK_KING as i32;
            },

            11 => { // black knight:
                self.black_knights         ^= to_bit_map;
                self.black_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::BLACK_KNIGHT as i32;
            },

            13 => { // black bishop:
                self.black_bishops         ^= to_bit_map;
                self.black_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::BLACK_BISHOP as i32;
            },

            14 => { // black rook:
                self.black_rooks         ^= to_bit_map;
                self.black_pieces        ^= to_bit_map;
                self.square[to as usize]  = gl::BLACK_ROOK as i32;
            },

            15 => { // black queen:
                self.black_queens          ^= to_bit_map;
                self.black_pieces          ^= to_bit_map;
                self.square[to as usize]    = gl::BLACK_QUEEN as i32;
            },

            _ => (),
        };
    }



    fn make_white_promotion (&mut self, prom: u32, to: u32) {
        let to_bit_map: gl::BitMap;
        unsafe { to_bit_map = gl::BITSET[to as usize]; }

        self.white_pawns ^= to_bit_map;

        if prom == 7 {
            self.white_queens          ^= to_bit_map;
        }
        else if prom == 6 {
            self.white_rooks         ^= to_bit_map;
        }
        else if prom == 5 {
            self.white_bishops       ^= to_bit_map;
        }
        else if prom == 3 {
            self.white_knights       ^= to_bit_map;
        }
    }


    fn unmake_white_promotion(&mut self, prom: u32, to: u32) {
        
        let to_bit_map: gl::BitMap;
        unsafe { to_bit_map = gl::BITSET[to as usize];}

        self.white_pawns ^= to_bit_map;

        if prom == 7 {
            self.white_queens       ^= to_bit_map;
        }
        else if prom == 6 {
            self.white_rooks        ^= to_bit_map;
        }
        else if prom == 5 {
            self.white_bishops      ^= to_bit_map;
        }
        else if prom == 3 {
            self.white_knights      ^= to_bit_map;
        }
    }


    fn make_black_promotion(&mut self, prom: u32, to: u32) {
        let to_bit_map: gl::BitMap;
        unsafe { to_bit_map = gl::BITSET[to as usize]; }

        self.black_pawns ^= to_bit_map;

        if prom == 15 {
            self.black_queens          ^= to_bit_map;
        }
        else if prom == 14 {
            self.black_rooks         ^= to_bit_map;
        }
        else if prom == 13 {
            self.black_bishops       ^= to_bit_map;
        }
        else if prom == 11 {
            self.black_knights       ^= to_bit_map;
        }
    }


    fn unmake_black_promotion(&mut self, prom: u32, to: u32) {
        
        let to_bit_map: gl::BitMap;
        unsafe { to_bit_map = gl::BITSET[to as usize]; }

        self.black_pawns ^= to_bit_map;

        if prom == 15 {
            self.black_queens    ^= to_bit_map;
        }
        else if prom == 14 {
            self.black_rooks     ^= to_bit_map;
        }
        else if prom == 13 {
            self.black_bishops    ^= to_bit_map;
        }
        else if prom == 11 {
            self.black_knights  ^= to_bit_map;
        }
    }


    pub fn is_other_king_attacked(&mut self) -> bool {
        // check to see if we are leaving our king in check
        if self.next_move != 0 {
            return self.is_attacked(self.white_king, self.next_move);
        }
        else {
            return self.is_attacked(self.black_king, self.next_move);
        }
    }


    pub fn is_own_king_attacked(&mut self) -> bool {
        // check to see if we are leaving our king in check
        if self.next_move == 1 {
            return self.is_attacked(self.black_king, self.next_move^1);
        }
        else {
            return self.is_attacked(self.white_king, self.next_move^1);
        }
    }


    pub fn is_end_of_game(&mut self) -> bool {

        let mut legalmoves: i32;

        // Checks if the current position is end-of-game due to:
        // checkmate, stalemate, 50-move rule, or insufficient material

        let whiteknights: i32;
        let whitebishops: i32;
        let whiterooks: i32;
        let whitequeens: i32;
        let whitetotalmat: i32;

        let blackknights: i32;
        let blackbishops: i32;
        let blackrooks: i32;
        let blackqueens: i32;
        let blacktotalmat: i32;

        // are we checkmating the other side?
        if self.is_other_king_attacked() {
            // i dont want show anything
            /*
            if self.next_move == gl::WHITE_MOVE {
                println!( "1-0 (Black mates)" );  
            }
            else { println!( "1-0 (White mates)" ); }
            */
            return true;
        }

        // how many legal moves do we have?
        legalmoves = 0;
        self.move_buf_len[0] = 0;
        self.move_buf_len[1] = self.movegen(self.move_buf_len[0]) as i32;
        for i in self.move_buf_len[0]..self.move_buf_len[1] {
            self.make_move(self.move_buffer[i as usize]);
            if !self.is_other_king_attacked() {
                legalmoves += 1;
                //singlemove = self.move_buffer[i as usize];
            }
            self.unmake_move(self.move_buffer[i as usize]);
        }

        // checkmate or stalemate?
        if legalmoves == 0 {
            // i dont want show anything
            /*
            if self.is_own_king_attacked() {
                if self.next_move == gl::WHITE_MOVE {
                    println!( "1-0 (White mates)" );
                }
                else {
                    println!( "1-0 (Black mates)" );
                }
            }
            else {
                println!( "1/2-1/2 (stalemate)" );
            }
            */
            return true;
        }

        // draw due to insufficient material:
        if self.white_pawns == 0 && self.black_pawns == 0 {
            whiteknights  = bitops::bit_cnt(self.white_knights);
            whitebishops  = bitops::bit_cnt(self.white_bishops);
            whiterooks    = bitops::bit_cnt(self.white_rooks);
            whitequeens   = bitops::bit_cnt(self.white_queens);
            whitetotalmat = 3 * whiteknights + 3 * whitebishops + 5 * whiterooks + 10 * whitequeens;

            blackknights  = bitops::bit_cnt(self.black_knights);
            blackbishops  = bitops::bit_cnt(self.black_bishops);
            blackrooks    = bitops::bit_cnt(self.black_rooks);
            blackqueens   = bitops::bit_cnt(self.black_queens);
            blacktotalmat = 3 * blackknights + 3 * blackbishops + 5 * blackrooks + 10 * blackqueens;

            // king versus king:
            if (whitetotalmat == 0) && (blacktotalmat == 0) {
                // i dont want show anything
                //println!( "1/2-1/2 (material)" );
                return true;
            }

            // king and knight versus king:
            if ((whitetotalmat == 3) && (whiteknights == 1) && (blacktotalmat == 0)) ||
                    ((blacktotalmat == 3) && (blackknights == 1) && (whitetotalmat == 0)) {
                // i dont want show anything
                println!( "1/2-1/2 (material)" );
                return true;
            }

            // 2 kings with one or more bishops, all bishops on the same colour:
            if (whitebishops + blackbishops) > 0 {
                if (whiteknights == 0) && (whiterooks == 0) && (whitequeens == 0) &&
                        (blackknights == 0) && (blackrooks == 0) && (blackqueens == 0) {
                    unsafe {
                    if ((self.white_bishops | self.black_bishops) & gl::WHITE_SQUARES) == 0 ||
                            ((self.white_bishops | self.black_bishops) & gl::BLACK_SQUARES) == 0 {
                        //println!( "1/2-1/2 (material)" );
                        return true;
                    }
                    } //end unsafe
                }
            }
        }

        if self.fifty_move >= 100 {
            //println!( "1/2-1/2 (50-move rule)" );
            return true;
        }

        false
    }

    // now moves

    pub fn make_san_move(&mut self, san_str: &str) -> legalmoves::LegalMove {
        let emptymove = legalmoves::LegalMove::new();
    
        let mut uci_str = "".to_string();
        let mut when_promo: &str = "";
        if san_str.ends_with("+") || san_str.ends_with("#") {   
            // I think it is better to remove them. Only give problems when promoting pawn
            when_promo = &san_str[0..san_str.len()-1];
        }
        else if san_str.ends_with("Q") || san_str.ends_with("R") ||
                san_str.ends_with("B") || san_str.ends_with("N") {
            when_promo = san_str.clone();
        }
    
        let pattern = r#"^([PNBRQK])?([a-h])?([1-8])?(x)?([a-h][1-8])(=?[qrbnQRBN])?(\+|#)?$"#;
    
        let mut legals : Vec<legalmoves::LegalMove> = self.get_legals();
    
        // 1.- first castles moves (special moves)
        match san_str {
            "O-O" |
            "O-O+" | "O-O#" => {
                if self.next_move == gl::WHITE_MOVE {
                    uci_str = String::from("e1g1");
                }
                else {
                    uci_str = String::from("e8g8");
                }
                let index = legals.iter().position(|r| r.uci == uci_str);   // -> Option<usize>
                match index {
                    Some(idx) => {
                        legals[idx].set_san(san_str.to_string());
                        self.make_move(legals[idx].movim);
                        return legals[idx].clone();
                    },
                    None => {
                        return emptymove;
                    },
                }
            },
            
            "O-O-O" |
            "O-O-O+" | "O-O-O#" => {
                if self.next_move == gl::WHITE_MOVE {
                    uci_str = "e1c1".to_string();
                }
                else {
                    uci_str = "e8c8".to_string();
                }
                let index = legals.iter().position(|r| r.uci == uci_str);   // -> Option<usize>
                match index {
                    Some(idx) => {
                        legals[idx].set_san(san_str.to_string());
                        self.make_move(legals[idx].movim);
                        return legals[idx].clone();
                    },
                    None => {
                        return emptymove;
                    },
                }
            },
            _ => (),
        };
    
        // 1 bis promotion
        if when_promo.ends_with("Q") || when_promo.ends_with("R") ||
                when_promo.ends_with("B") || when_promo.ends_with("N")  {
            
            let mut minus: &str = "";
            let mut to: &str = "";
            if when_promo.contains("=") {
                let split: Vec<&str> = when_promo.split("=").collect();
                match split[1] {
                    "Q" => minus = "q",
                    "R" => minus = "r",
                    "B" => minus = "b",
                    "N" => minus = "n",
                    _ => (),
                }
                if when_promo.len() == 4 {
                    to = split[0];
                }
                else {
                    let tmp = split[0];
                    to = &tmp[tmp.len()-2..];
                }
                
            }
            else {  // sometimes there is'nt the '='
                minus = &when_promo[when_promo.len()-1..];
                match minus {
                    "Q" => minus = "q",
                    "R" => minus = "r",
                    "B" => minus = "b",
                    "N" => minus = "n",
                    _ => (),
                }
                if when_promo.len() > 3 {
                    to = &when_promo[when_promo.len()-3..when_promo.len()-1];
                }
                else {
                    to = &when_promo[0..2];
                }
            }
            let ending = format!("{}{}", to, minus);
            for mut legal in legals.clone() {
                if legal.uci.ends_with(&ending) {
                    legal.set_san(san_str.to_string());
                    self.make_move(legal.movim);
                    return legal;
                }
            }
        }
    
    
        // 2.- begin with regex of SAN move
        let re = Regex::new(pattern);
        if re.is_err() { return emptymove; }   // san string is erroneous
        let re_valid = re.unwrap();
        let tokens = re_valid.captures(san_str).unwrap();
        /*
        ! Eaxmple with largest possible SAN = Nf3xd4=Q+ (len=9)
        ! &tokens[0] = Some(Nf3xd4=Q+)
        ! tokens(1) = N
        ! tokens(2) = f
        ! tokens(3) = 3
        ! tokens(4) = x
        ! tokens(5) = d4
        ! tokens(6) = =Q
        ! tokens(7) = +
        ! each of the tokens is Some(str) or None
        */
    
        // 3.- test if origin square is complete Ng1f3 (g1 is origin from square)
        // tokens(1) is ommited
        if tokens.get(2).is_some() && tokens.get(3).is_some() {
            uci_str.push_str(tokens.get(2).unwrap().as_str());
            uci_str.push_str(tokens.get(3).unwrap().as_str());
            //tokens(5) siempre es la casilla destino
            uci_str.push_str(tokens.get(5).unwrap().as_str());
            // una posible coronacion
            if tokens.get(6).is_some() {
                let corona = tokens.get(6).unwrap().as_str().to_string();
                let pieza_mayus: &[u8];
                // some times there is not the '=' char in promotion san notation
                if corona.contains("=") {
                    pieza_mayus = corona[1..2].as_bytes();
                }
                else {
                    pieza_mayus = corona[0..1].as_bytes();
                }
    
                let pieza_minus = (pieza_mayus[0] + 32_u8) as char;
                uci_str.push(pieza_minus);
            }
    
            let index = legals.iter().position(|r| r.uci == uci_str);   // -> Option<usize>
            match index {
                Some(idx) => {
                    legals[idx].set_san(san_str.to_string());
                    self.make_move(legals[idx].movim);
                    return legals[idx].clone();
                },
                None => {
                    return emptymove;
                },
            }
        }
    
        // this block changed on 27-08-2021
        // 4.- single pawn move d4 or d8=Q (no capture)
        if tokens.get(1).is_none() && tokens.get(2).is_none() &&
                tokens.get(3).is_none() && tokens.get(4).is_none() && tokens.get(5).is_some() {
    
            for mut legal in legals.clone() {
                let to = legal.movim.get_to_sq();
                let piece = legal.movim.get_piec();
    
                if piece == gl::WHITE_PAWN as u32 || piece == gl::BLACK_PAWN as u32 {
                    let to_str = gl::SQUARENAME[to as usize];
                    if to_str ==  tokens.get(5).unwrap().as_str() {
                        legal.set_san(san_str.to_string());
                        self.make_move(legal.movim);
                        return legal;
                    }
                }
                
            }
        }
    
        // 5.- pawn move with capture exd4 o fxd8=Q
        if tokens.get(1).is_none() && tokens.get(2).is_some() &&
                tokens.get(3).is_none() && tokens.get(4).is_some() && tokens.get(5).is_some() {
    
            for mut legal in legals.clone() {
                let capture: bool = legal.movim.is_capture();
                if capture {
                    // get destination square
                    let to = legal.movim.get_to_sq();
                    let to_str = gl::SQUARENAME[to as usize];
    
                    //get the number of column/file
                    let file_nr;
                    match tokens.get(2).unwrap().as_str() {
                        "a" => file_nr = 1,
                        "b" => file_nr = 2,
                        "c" => file_nr = 3,
                        "d" => file_nr = 4,
                        "e" => file_nr = 5,
                        "f" => file_nr = 6,
                        "g" => file_nr = 7,
                        "h" => file_nr = 8,
                        _  => panic!("error en legalmoves linea 247"),
                    };
    
                    if gl::FILES[legal.movim.get_from() as usize] == file_nr &&
                            to_str ==  tokens.get(5).unwrap().as_str() {
                        legal.set_san(san_str.to_string());
                        self.make_move(legal.movim);
                        return legal;
                    }
    
                }
            }
        }
    
        // 6.- now pieces which are not pawns
        // may be i.e. : Nf3, Ngf3, N1f3 (including posible captures x)
        // Ng1f3, Ng1xf3 are covered at 3.-
    
        if tokens.get(1).is_some() && tokens.get(5).is_some() {
            let mut piece_nr = gl::EMPTY;
            let piece_text = tokens.get(1).unwrap().as_str();
            if self.next_move == gl::WHITE_MOVE {
                if piece_text == "K" { piece_nr = gl::WHITE_KING; }
                else if piece_text == "N" { piece_nr = gl::WHITE_KNIGHT; }
                else if piece_text == "B" { piece_nr = gl::WHITE_BISHOP;}
                else if piece_text == "R" { piece_nr = gl::WHITE_ROOK;}
                else if piece_text == "Q" { piece_nr = gl::WHITE_QUEEN; }
                else if piece_text == "P" { piece_nr = gl::WHITE_PAWN; }    // not necessary, but... just in case
            }
            else {
                if piece_text == "K" { piece_nr = gl::BLACK_KING; }
                else if piece_text == "N" { piece_nr = gl::BLACK_KNIGHT; }
                else if piece_text == "B" { piece_nr = gl::BLACK_BISHOP;}
                else if piece_text == "R" { piece_nr = gl::BLACK_ROOK;}
                else if piece_text == "Q" { piece_nr = gl::BLACK_QUEEN; }
                else if piece_text == "P" { piece_nr = gl::BLACK_PAWN; }
            }
    
            // we create a a vec posibility of more than one move
            // i.e.: N1f3 or Ng3 (x included)
            let mut possibles: Vec<legalmoves::LegalMove> = Vec::new();
            for legal in legals.clone() {
                let to = legal.movim.get_to_sq();
                if piece_nr as u32 == legal.movim.get_piec() && 
                        tokens.get(5).unwrap().as_str() == gl::SQUARENAME[to as usize] {
                    
                    possibles.push(legal);
                }
            }
    
            if possibles.len() == 0 {
                return emptymove;
            }
            else if possibles.len() == 1 {  //disambiguation not necessary
                possibles[0].set_san(san_str.to_string());
                self.make_move(possibles[0].movim);
                return possibles[0].clone();
            }
            else if possibles.len() >= 2 {      // maximum 1 disambiguation
                for mut possible in possibles {
                    // now disambiguation
                    // Ngf3
                    if tokens.get(2).is_some() {
                        //get the number of column/file
                        let file_nr;
                        match tokens.get(2).unwrap().as_str() {
                            "a" => file_nr = 1,
                            "b" => file_nr = 2,
                            "c" => file_nr = 3,
                            "d" => file_nr = 4,
                            "e" => file_nr = 5,
                            "f" => file_nr = 6,
                            "g" => file_nr = 7,
                            "h" => file_nr = 8,
                            _  => panic!("error en legalmoves linea 337"),
                        };
    
                        if gl::FILES[possible.movim.get_from() as usize] == file_nr {
                            possible.set_san(san_str.to_string());
                            self.make_move(possible.movim);
                            return possible.clone();
                        }
                    }
                    // N1f3
                    if tokens.get(3).is_some() {
                        //get the number of column/file
                        let file_nr = gl::RANKS[possible.movim.get_from() as usize];
    
                        if  tokens.get(3).unwrap()
                        .as_str().to_string().trim()
                        .parse::<i32>().unwrap() == file_nr {
                            possible.set_san(san_str.to_string());
                            self.make_move(possible.movim);
                            return possible.clone();
                        }
                    }
                }
            }
        }
    
        return emptymove;
    }


    pub fn make_uci_move(&mut self, uci_str: &str) -> legalmoves::LegalMove {
        let mut legals : Vec<legalmoves::LegalMove> = self.get_legals();
    
        //for legal in legals.iter() {
        //    let san = displaymove::get_san_disambig (board, legal.movim);
        //}
    
        let index = legals.iter().position(|r| r.uci == uci_str.to_string());   // -> Option<usize>
    
        match index {
            Some(idx) => {
                let san = displaymove::get_san_disambig (self, legals[idx].movim);
                legals[idx].set_san(san);
                self.make_move(legals[idx].movim);
                return legals[idx].clone();
            },
            None => {
                let empty = legalmoves::LegalMove::new();
                return empty;
            },
        }
    }


    pub fn get_legals (&mut self) -> Vec<legalmoves::LegalMove> {
        let mut legal_moves: Vec<legalmoves::LegalMove> = Vec::new();
    
        self.move_buf_len[0] = 0;
        self.move_buf_len[1] = self.movegen(self.move_buf_len[0]) as i32;
    
        // TODO -> DONE: board.is_end_of_game()
        // if length of array of legal moves == 0, then is Mate or StaleMated
        // since this program will only be used for pgn validation it is faster
        // without test the posiible mate, stalemate, etc...
    
        //let end = board.is_end_of_game();
        //if end {
        //    return legal_moves;
        //}
        //else {
            for i in self.move_buf_len[0]..self.move_buf_len[1]  {
                self.make_move(self.move_buffer[i as usize]);
                if self.is_other_king_attacked() {
                    self.unmake_move(self.move_buffer[i as usize]);
                }
                else {
                    // get uci text
                    let move_uci = self.move_buffer[i as usize].get_uci();
                    let mut legalm = legalmoves::LegalMove::new();
                    legalm.set_movim(self.move_buffer[i as usize]);
                    legalm.set_uci(move_uci);
                    
                    legal_moves.push(legalm);
    
                    self.unmake_move(self.move_buffer[i as usize]);
                }
            }
    
            //  cleanup:
            self.move_buf_len[0] = 0;
            self.move_buf_len[1] = 0;
    
            return legal_moves;
        //}
    
    }
}