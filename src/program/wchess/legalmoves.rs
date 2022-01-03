use regex::Regex;

use super::globs as gl;
use super::movim::Movim;
use super::board::Board;
use super::displaymove;


#[derive(Clone)]
pub struct LegalMove {
    pub movim: Movim,
    pub uci: String,
    pub san: String,
}


impl LegalMove {
    pub fn new() -> Self {
        LegalMove {
            movim: Movim::new(),
            uci: String::new(),
            san: String::new(),
        }
    }

    pub fn set_movim (&mut self, mov: Movim) {
        self.movim = mov;
    }

    pub fn set_uci (&mut self, ucistr: String) {
        self.uci = ucistr;
    }

    pub fn set_san (&mut self, sanstr: String) {
        self.san = sanstr;
    }
}


/*
pub fn get_legals (board: &mut Board) -> Vec<LegalMove> {
    let mut legal_moves: Vec<LegalMove> = Vec::new();

    board.move_buf_len[0] = 0;
    board.move_buf_len[1] = board.movegen(board.move_buf_len[0]) as i32;

	// TODO -> DONE: board.is_end_of_game()
	// if length of array of legal moves == 0, then is Mate or StaleMated
    // since this program will only be used for pgn validation it is faster
    // without test the posiible mate, stalemate, etc...

    //let end = board.is_end_of_game();
    //if end {
    //    return legal_moves;
    //}
    //else {
        for i in board.move_buf_len[0]..board.move_buf_len[1]  {
            board.make_move(board.move_buffer[i as usize]);
            if board.is_other_king_attacked() {
                board.unmake_move(board.move_buffer[i as usize]);
            }
            else {
                // get uci text
                let move_uci = board.move_buffer[i as usize].get_uci();
                let mut legalm = LegalMove::new();
                legalm.set_movim(board.move_buffer[i as usize]);
                legalm.set_uci(move_uci);
                
                legal_moves.push(legalm);

                board.unmake_move(board.move_buffer[i as usize]);
            }
        }

        //  cleanup:
        board.move_buf_len[0] = 0;
        board.move_buf_len[1] = 0;

        return legal_moves;
    //}

}
*/

/*
pub fn make_uci_move(board: &mut Board, uci_str: &str) -> LegalMove {
    let mut legals : Vec<LegalMove> = get_legals(board);

    //for legal in legals.iter() {
    //    let san = displaymove::get_san_disambig (board, legal.movim);
    //}

    let index = legals.iter().position(|r| r.uci == uci_str.to_string());   // -> Option<usize>

    match index {
        Some(idx) => {
            let san = displaymove::get_san_disambig (board, legals[idx].movim);
            legals[idx].set_san(san);
            board.make_move(legals[idx].movim);
            return legals[idx].clone();
        },
        None => {
            let empty = LegalMove::new();
            return empty;
        },
    }
}
*/

/*
pub fn make_san_move(board: &mut Board, san_str: &str) -> LegalMove {
    let emptymove = LegalMove::new();

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

    let mut legals : Vec<LegalMove> = get_legals(board);

    // 1.- first castles moves (special moves)
    match san_str {
        "O-O" |
        "O-O+" | "O-O#" => {
            if board.next_move == gl::WHITE_MOVE {
                uci_str = String::from("e1g1");
            }
            else {
                uci_str = String::from("e8g8");
            }
            let index = legals.iter().position(|r| r.uci == uci_str);   // -> Option<usize>
            match index {
                Some(idx) => {
                    legals[idx].set_san(san_str.to_string());
                    board.make_move(legals[idx].movim);
                    return legals[idx].clone();
                },
                None => {
                    return emptymove;
                },
            }
        },
        
        "O-O-O" |
        "O-O-O+" | "O-O-O#" => {
            if board.next_move == gl::WHITE_MOVE {
                uci_str = "e1c1".to_string();
            }
            else {
                uci_str = "e8c8".to_string();
            }
            let index = legals.iter().position(|r| r.uci == uci_str);   // -> Option<usize>
            match index {
                Some(idx) => {
                    legals[idx].set_san(san_str.to_string());
                    board.make_move(legals[idx].movim);
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
                board.make_move(legal.movim);
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
                board.make_move(legals[idx].movim);
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
                    board.make_move(legal.movim);
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
                    board.make_move(legal.movim);
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
        if board.next_move == gl::WHITE_MOVE {
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
        let mut possibles: Vec<LegalMove> = Vec::new();
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
            board.make_move(possibles[0].movim);
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
                        board.make_move(possible.movim);
                        return possible.clone();
                    }
                }
                // N1f3
                if tokens.get(3).is_some() {
                    //get the number of column/file
                    let file_nr = gl::RANKS[possible.movim.get_from() as usize];

                    if  tokens.get(3).unwrap().as_str().to_string().trim().parse::<i32>().unwrap() == file_nr {
                        possible.set_san(san_str.to_string());
                        board.make_move(possible.movim);
                        return possible.clone();
                    }
                }
            }
        }
    }

    return emptymove;
}
*/