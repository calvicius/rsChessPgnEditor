use super::globs as gl;
use super::movim;
use super::board::Board;

// san notation with ambiguous notation
pub fn get_san_ambig (movim: movim::Movim) -> String {
    
    let mut move_san = String::from("");
    
    // displays a single move on the console, no disambiguation
    if (movim.get_piec() == gl::WHITE_KING as u32) && (movim.is_castle_oo()) {
        return "O-O".to_string();
    };
    if (movim.get_piec() == gl::WHITE_KING as u32) && (movim.is_castle_ooo()) {
        return "O-O-O".to_string();
    };
    if (movim.get_piec() == gl::BLACK_KING as u32) && (movim.is_castle_oo()) {
        return "O-O".to_string();
    };
    if (movim.get_piec() == gl::BLACK_KING as u32) && (movim.is_castle_ooo()) {
        return "O-O-O".to_string();
    };


    if (movim.get_piec() == gl::WHITE_ROOK as u32)   || (movim.get_piec() == gl::BLACK_ROOK as u32)   {
        move_san.push_str("R");
    }
    if (movim.get_piec() == gl::WHITE_BISHOP as u32) || (movim.get_piec() == gl::BLACK_BISHOP as u32) {
        move_san.push_str("B");
    }
    if (movim.get_piec() == gl::WHITE_KNIGHT as u32) || (movim.get_piec() == gl::BLACK_KNIGHT as u32) {
        move_san.push_str("N");
    }
    if (movim.get_piec() == gl::WHITE_KING as u32)   || (movim.get_piec() == gl::BLACK_KING as u32) {
        move_san.push_str("K");
    }
    if (movim.get_piec() == gl::WHITE_QUEEN as u32)  || (movim.get_piec() == gl::BLACK_QUEEN as u32) {
        move_san.push_str("Q");
    }
    if ((movim.get_piec() == gl::WHITE_PAWN as u32)  || (movim.get_piec() == gl::BLACK_PAWN as u32)) 
    && movim.is_capture() {
            if gl::FILES[movim.get_from() as usize] == 1 { move_san.push_str("a"); }
            if gl::FILES[movim.get_from() as usize] == 2 { move_san.push_str("b"); }
            if gl::FILES[movim.get_from() as usize] == 3 { move_san.push_str("c"); }
            if gl::FILES[movim.get_from() as usize] == 4 { move_san.push_str("d"); }
            if gl::FILES[movim.get_from() as usize] == 5 { move_san.push_str("e"); }
            if gl::FILES[movim.get_from() as usize] == 6 { move_san.push_str("f"); }
            if gl::FILES[movim.get_from() as usize] == 7 { move_san.push_str("g"); }
            if gl::FILES[movim.get_from() as usize] == 8 { move_san.push_str("h"); }
    }

    if movim.is_capture() { move_san.push_str("x"); }; 

    if gl::FILES[movim.get_to_sq() as usize] == 1 { move_san.push_str("a"); }
    if gl::FILES[movim.get_to_sq() as usize] == 2 { move_san.push_str("b"); }
    if gl::FILES[movim.get_to_sq() as usize] == 3 { move_san.push_str("c"); }
    if gl::FILES[movim.get_to_sq() as usize] == 4 { move_san.push_str("d"); }
    if gl::FILES[movim.get_to_sq() as usize] == 5 { move_san.push_str("e"); }
    if gl::FILES[movim.get_to_sq() as usize] == 6 { move_san.push_str("f"); }
    if gl::FILES[movim.get_to_sq() as usize] == 7 { move_san.push_str("g"); }
    if gl::FILES[movim.get_to_sq() as usize] == 8 { move_san.push_str("h"); }

    move_san.push_str( gl::RANKS[movim.get_to_sq() as usize].to_string().as_str() );

    if movim.is_promotion() {
        if (movim.get_prom() == gl::WHITE_ROOK as u32)   || (movim.get_prom() == gl::BLACK_ROOK as u32) {
            move_san.push_str("=R");
        }
        if (movim.get_prom() == gl::WHITE_BISHOP as u32) || (movim.get_prom() == gl::BLACK_BISHOP as u32) {
            move_san.push_str("=B");
        }
        if (movim.get_prom() == gl::WHITE_KNIGHT as u32) || (movim.get_prom() == gl::BLACK_KNIGHT as u32) {
            move_san.push_str("=N");
        }
        if (movim.get_prom() == gl::WHITE_KING as u32)   || (movim.get_prom() == gl::BLACK_KING as u32) {
            move_san.push_str("=K");
        }
        if (movim.get_prom() == gl::WHITE_QUEEN as u32)  || (movim.get_prom() == gl::BLACK_QUEEN as u32) {
            move_san.push_str("=Q");
        }
    }
    
    return move_san;
}

// will convert a move into non-ambiguous SAN-notation
pub fn get_san_disambig (board: &mut Board, movim: movim::Movim) -> String {

    //  ===========================================================================
    //  will convert a move into non-ambiguous SAN-notation, returned in String san_move.
    //  "move" must belong to the current "board". Returns true if successful.
    //  The move is compared with other moves from the current board position.
    //  Ambiguities can arise if two (or more) identical pieces can move to the same square.
    //  In such cases, the piece's initial is followed by (in this priority):
    //  - the from file, if it's unique,
    //  - or else the from rank, if it's unique
    //  - or else the from file and rank (this can happen after pawn promotions,
    //       e.g. with 4 rooks on c3, c7, a5 and e5; they all can move to c5, and then move notation would be: R3c5
    //  'e.p.' is added for an en-passant capture
    //  '+'is added for check, '#' is added for mate.

    let mut san_move = String::from("");

    let ascii_shift: u8 = 'a' as u8;
    let piece = movim.get_piec();
    let from = movim.get_from();
    let to = movim.get_to_sq();
    let capt = movim.get_capt();
    let prom = movim.get_prom();
    let mut ibuf: usize = 0;
    let mut ambig = false;
    let mut ambigfile = 0;
    let mut ambigrank = 0;
    let mut legal = false;
    let mut check = false;
    let mut mate = false;

    //     Generate all pseudo-legal moves to be able to remove any ambiguities
    //     and check legality. Take the next free location in moveBufLen:
    while board.move_buf_len[ibuf+1] != 0 { ibuf += 1; }
    board.move_buf_len[ibuf+1] = board.movegen(board.move_buf_len[ibuf]) as i32;

    //     Loop over the moves to see what kind(s) of ambiguities exist, if any:
    for i in board.move_buf_len[ibuf]..board.move_buf_len[ibuf+1] {

        board.make_move(board.move_buffer[i as usize]);
        if !board.is_other_king_attacked() {
            if board.move_buffer[i as usize].move_int == movim.move_int {
                legal = true;
                // it is check:
                if board.is_own_king_attacked() {
                    check = true;
                    // is it checkmate?
                    let mut k = 0;
                    board.move_buf_len[ibuf+2] = board.movegen(board.move_buf_len[ibuf+1]) as i32;
                    for j in board.move_buf_len[ibuf+1]..board.move_buf_len[ibuf+2] {
                        board.make_move(board.move_buffer[j as usize]);
                        if !board.is_other_king_attacked() { k += 1; }
                        board.unmake_move(board.move_buffer[j as usize]);
                    }
                    if k == 0 { mate = true; }
                }
            }
            // two same pieces can move to the same square:
            if (board.move_buffer[i as usize].move_int != movim.move_int) &&
                (board.move_buffer[i as usize].get_piec() == piece) &&
                (board.move_buffer[i as usize].get_to_sq() == to) 
            {
                ambig = true;
                if gl::FILES[from as usize] == 
                    gl::FILES[board.move_buffer[i as usize].get_from() as usize] { ambigfile += 1; }
                if gl::RANKS[from as usize] == 
                    gl::RANKS[board.move_buffer[i as usize].get_from() as usize] { ambigrank += 1; }
            }
        }
        board.unmake_move(board.move_buffer[i as usize]);
    }

    //  cleanup:
    board.move_buf_len[ibuf+1] = 0;
    board.move_buf_len[ibuf+2] = 0;

    //     construct the SAN string:
    if !legal {
        san_move = String::from("None");
        return san_move;
    }
    else {
        // is castle
        if (movim.get_piec() == gl::WHITE_KING as u32)  && (movim.is_castle_oo()) {
            //return "O-O".to_string();
            if check {
                if mate {
                    return "O-O#".to_string();
                }
                else {
                    return "O-O+".to_string();
                }
            }
            else { return "O-O".to_string(); }
        };
        if (movim.get_piec() == gl::WHITE_KING as u32) && (movim.is_castle_ooo()) {
            //return "O-O-O".to_string();
            if check {
                if mate {
                    return "O-O#".to_string();
                }
                else {
                    return "O-O+".to_string();
                }
            }
            else { return "O-O".to_string(); }
        };
        if (movim.get_piec() == gl::BLACK_KING as u32) && (movim.is_castle_oo()) {
            //return "O-O".to_string();
            if check {
                if mate {
                    return "O-O#".to_string();
                }
                else {
                    return "O-O+".to_string();
                }
            }
            else { return "O-O".to_string(); }
        };
        if (movim.get_piec() == gl::BLACK_KING as u32) && (movim.is_castle_ooo()) {
            //return "O-O-O".to_string();
            if check {
                if mate {
                    return "O-O#".to_string();
                }
                else {
                    return "O-O+".to_string();
                }
            }
            else { return "O-O".to_string(); }
        };
        // start building the string
        if !movim.is_pawn_move() {
            san_move = format!("{}{}", san_move, gl::PIECECHARS[piece as usize]);
            if ambig {
                if ambigfile != 0 {
                    if ambigrank != 0 {
                        san_move = format!("{}{}{}", 
                            san_move, 
                            (gl::FILES[from as usize] as u8+ascii_shift - 1) as char, 
                            gl::RANKS[from as usize]);
                    }
                    else {
                        san_move = format!("{}{}", san_move, gl::RANKS[from as usize]);
                    }
                }
                else {
                    san_move = format!("{}{}", san_move, 
                        (gl::FILES[from as usize] as u8 + ascii_shift - 1) as char);
                }
            }
        }
        else {
            if movim.is_capture() {
                san_move = format!("{}{}", san_move, 
                    (gl::FILES[from as usize] as u8 + ascii_shift - 1) as char);
            }
        }
        if movim.is_capture() {
            san_move.push_str("x");
        }
        
        san_move = format!("{}{}{}", san_move,
            (gl::FILES[to as usize] as u8 + ascii_shift - 1) as char, 
            gl::RANKS[to as usize]);
        if movim.is_enpassant() {
            san_move.push_str(" e.p.")
        }
        if movim.is_promotion() {
            san_move.push('=');
            san_move.push_str(gl::PIECECHARS[prom as usize]);
        }
        if check {
            if mate {
                san_move.push('#');
            }
            else {
                san_move.push('+')
            }
        }
        san_move
    }
    
}