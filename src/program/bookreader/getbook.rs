
use crate::program::wchess;

use super::keys;

use std::io::{Read,Seek,SeekFrom};
use std::convert::TryInto;

const POLY_COLS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
const POLY_ROWS: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

// ====== Auxiliary funcs. ================
fn get_col(sq: usize) -> usize {
    sq % 8
}

fn get_row(sq: usize) -> usize {
    sq/8
}

fn get_left(sq: usize) -> Option<usize> {
    match sq {
        0   => return None,
        8   => return None,
        16  => return None,
        24  => return None,
        32  => return None,
        40  => return None,
        48  => return None,
        56  => return None,
        _   => return Some(sq-1)
    }
}

fn get_right(sq: usize) -> Option<usize> {
    match sq {
        7   => return None,
        15  => return None,
        23  => return None,
        31  => return None,
        39  => return None,
        47  => return None,
        55  => return None,
        63  => return None,
        _   => return Some(sq+1)
    }
}

pub fn get_uci_move(mov: Move) -> String {
    
    let promotion: &str;
    if mov.promotion.is_none() {
        promotion = "";
    }
    else {
        let piece_type = mov.promotion.unwrap();
        match piece_type {
            PieceType::Pawn   => promotion = "p",     // never occurs
            PieceType::Knight => promotion = "n",
            PieceType::Bishop => promotion = "b",
            PieceType::Rook   => promotion = "r",
            PieceType::Queen  => promotion = "q",
            PieceType::King   => promotion = "k"      // never occurs
        }
    }
    let mut uci_mov = format!("{}{}{}{}{}",
        POLY_COLS[mov.source.file], POLY_ROWS[mov.source.rank],
        POLY_COLS[mov.dest.file], POLY_ROWS[mov.dest.rank],
        promotion
    );
    // arreglamos los enroques
    if uci_mov == "e1h1" {
        uci_mov = String::from("e1g1");
    }
    else if uci_mov == "e1a1" {
        uci_mov = "e1c1".to_string();
    }
    else if uci_mov == "e8h8" {
        uci_mov = "e8g8".to_string();
    }
    else if uci_mov == "e8a8" {
        uci_mov = "e8c8".to_string();
    }
    uci_mov
}

// =========================================


pub fn init_reader (pathbook: &str, vfen: &str) -> Vec<PolyglotEntry> {
    use std::fs::File;

    let arrfen: &[&str] = &[vfen];
    let mut moves: Vec<PolyglotEntry> = Vec::new();

    let file_res = File::open(pathbook);    //.unwrap();
    if file_res.is_ok() {
        let file = file_res.unwrap();
        let mut reader = PolyglotReader::new(file).unwrap();
        
        for (_i, &fen) in arrfen.iter().enumerate() {
            //let board = wchess::Board::from_str(fen).unwrap();
            let mut board = wchess::new_board();
            wchess::set_fen(&mut board, fen);
            let k = PolyglotKey::from_board(&board);
            moves = reader.get(&k).unwrap();
        }
    }
    return moves;
}


// ================ Side ==============================
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Side {
    White,
    Black
}


// ================ PieceType =========================
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl PieceType {
    pub fn index(self) -> usize {
        match self {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5
        }
    }
}


// ================ Piece =============================
#[derive(Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub side: Side,
    pub square: Square
}

impl Piece {
    pub fn polyglot_hash(&self) -> u64 {
        let kind = self.piece_type.index() * 2 + (self.side == Side::White) as usize;
        keys::RANDOM_PIECE[64 * kind + 8 * self.square.rank + self.square.file]
    }
}


// ================ CastleRights ======================
#[derive(Debug)]
pub struct CastleRights {
    pub queen_side: bool,
    pub king_side: bool
}

impl CastleRights {
    pub fn polyglot_hash(&self, side: Side) -> u64 {
        let mut hash = 0;
        let base = if side == Side::White {
            0
        } else {
            2
        };
        if self.king_side {
            hash ^= keys::RANDOM_CASTLE[base];
        }
        if self.queen_side {
            hash ^= keys::RANDOM_CASTLE[base + 1];
        }
        hash
    }
}


// ================ PolyglotKey =======================
#[derive(Debug)]
pub struct PolyglotKey {
    pub pieces: Vec<Piece>,
    pub white_castle: CastleRights,
    pub black_castle: CastleRights,
    pub en_passant_file: Option<usize>,
    pub turn: Side
}

impl PolyglotKey {

    pub fn polyglot_hash(&self) -> u64 {
        let mut hash = 0;
        for piece in &self.pieces {
            hash ^= piece.polyglot_hash();
        }
        hash ^= self.white_castle.polyglot_hash(Side::White);
        hash ^= self.black_castle.polyglot_hash(Side::Black);
        if let Some(file) = self.en_passant_file {
            hash ^= keys::RANDOM_EN_PASSANT[file];
        }
        if self.turn == Side::White {
            hash ^= keys::RANDOM_TURN;
        }
        hash
    }

    pub fn from_board(board: &wchess::board::Board) -> Self {
        let mut pieces: Vec<Piece> = Vec::new();
        for i in 0..board.square.len() {
            let col = get_col(i);
            let row = get_row(i);
            match board.square[i] as u8 {
                wchess::globs::EMPTY => (),
                wchess::globs::WHITE_PAWN => {
                    let piece_type = PieceType::Pawn;
                    let sq = Square { rank: row, file: col};
                    let color = Side::White;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::WHITE_KING => {
                    let piece_type = PieceType::King;
                    let sq = Square { rank: row, file: col};
                    let color = Side::White;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::WHITE_KNIGHT => {
                    let piece_type = PieceType::Knight;
                    let sq = Square { rank: row, file: col};
                    let color = Side::White;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::WHITE_BISHOP => {
                    let piece_type = PieceType::Bishop;
                    let sq = Square { rank: row, file: col};
                    let color = Side::White;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::WHITE_ROOK => {
                    let piece_type = PieceType::Rook;
                    let sq = Square { rank: row, file: col};
                    let color = Side::White;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::WHITE_QUEEN => {
                    let piece_type = PieceType::Queen;
                    let sq = Square { rank: row, file: col};
                    let color = Side::White;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                // Black pieces
                wchess::globs::BLACK_PAWN => {
                    let piece_type = PieceType::Pawn;
                    let sq = Square { rank: row, file: col};
                    let color = Side::Black;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::BLACK_KING => {
                    let piece_type = PieceType::King;
                    let sq = Square { rank: row, file: col};
                    let color = Side::Black;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::BLACK_KNIGHT => {
                    let piece_type = PieceType::Knight;
                    let sq = Square { rank: row, file: col};
                    let color = Side::Black;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::BLACK_BISHOP => {
                    let piece_type = PieceType::Bishop;
                    let sq = Square { rank: row, file: col};
                    let color = Side::Black;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::BLACK_ROOK => {
                    let piece_type = PieceType::Rook;
                    let sq = Square { rank: row, file: col};
                    let color = Side::Black;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                wchess::globs::BLACK_QUEEN => {
                    let piece_type = PieceType::Queen;
                    let sq = Square { rank: row, file: col};
                    let color = Side::Black;
                    let piece = Piece {
                        piece_type,
                        square: sq,
                        side: color,
                    };
                    pieces.push(piece);
                },
                
                _ => (),
            }
        }

        // Used for castling:
        //pub const CANCASTLEOO: u8 = 1;
        //pub const CANCASTLEOOO: u8 = 2;
        // white
        let mut w_castles: CastleRights = CastleRights { queen_side: false, king_side: false };
        // it is set already
        //if board.castle_white == 0 {
        //    w_castles = CastleRights { queen_side: false, king_side: false };
        //}
        if board.castle_white == 1 {
            w_castles = CastleRights { queen_side: false, king_side: true };
        }
        if board.castle_white == 2 {
            w_castles = CastleRights { queen_side: true, king_side: false };
        }
        if board.castle_white == 3 {
            w_castles = CastleRights { queen_side: true, king_side: true };
        }
        // black
        let mut b_castles: CastleRights = CastleRights { queen_side: false, king_side: false };
        // it is set already
        //if board.castle_black == 0 {
        //    b_castles = CastleRights { queen_side: false, king_side: false };
        //}
        if board.castle_black == 1 {
            b_castles = CastleRights { queen_side: false, king_side: true };
        }
        if board.castle_black == 2 {
            b_castles = CastleRights { queen_side: true, king_side: false };
        }
        if board.castle_black == 3 {
            b_castles = CastleRights { queen_side: true, king_side: true };
        }

        // enpassant 
        //let sq_enpassant: Option<i32>;
        let mut pawn_to_capture_sq = board.ep_square;
        //if no ep --> board.ep_square == 0
        if pawn_to_capture_sq > 0 {
            if board.next_move == wchess::globs::WHITE_MOVE {
                pawn_to_capture_sq -= 8;
            }
            else {
                pawn_to_capture_sq += 8;
            }
        }
        
        let col_passant = Some(pawn_to_capture_sq).and_then(|en_passant_sq| {
            let left = get_left(en_passant_sq.try_into().unwrap());
            let right = get_right(en_passant_sq.try_into().unwrap());
            
            [left, right]
                .iter()
                .flatten()
                .find_map(|&sq| {
                    if (board.square[sq] == wchess::globs::WHITE_PAWN as i32 && 
                        board.next_move == wchess::globs::WHITE_MOVE) ||
                       (board.square[sq] == wchess::globs::BLACK_PAWN as i32 &&
                        board.next_move == wchess::globs::BLACK_MOVE)
                    {
                        Some(get_col(en_passant_sq.try_into().unwrap()))
                    }
                    else {
                        None
                    }
                })
        });

        // turn
        let turn: Side;
        if board.next_move == wchess::globs::WHITE_MOVE {
            turn = Side::White;
        }
        else {
            turn = Side::Black;
        }

        Self {
            pieces,
            white_castle: w_castles,
            black_castle: b_castles,
            en_passant_file: col_passant,
            turn
        }
        
    }

}

// ================ Square ============================
#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub rank: usize,
    pub file: usize
}


// ================ Move ==============================
#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub source: Square,
    pub dest: Square,
    pub promotion: Option<PieceType>
}

impl Move {
    pub fn from_u16(mv: u16) -> Self {
        fn index(mv: u16, i: usize) -> usize {
            ((mv >> (i * 3)) & 0b111) as usize
        }
        Self {
            dest: Square {
                file: index(mv, 0),
                rank: index(mv, 1)
            },
            source: Square {
                file: index(mv, 2),
                rank: index(mv, 3)
            },
            promotion: match index(mv, 4) {
                0 => None,
                1 => Some(PieceType::Knight),
                2 => Some(PieceType::Bishop),
                3 => Some(PieceType::Rook),
                4 => Some(PieceType::Queen),
                p => unreachable!("Invalid promotion {}", p)
            }
        }
    }
}

// ================ PolygotEntry ======================
#[derive(Copy, Clone, Debug)]
pub struct PolyglotEntry {
    pub mv: Move,
    pub weight: u16
}

impl PolyglotEntry {
    pub const SIZE: usize = 16;
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut mv = [0; 2];
        mv.copy_from_slice(&bytes[0..2]);

        let mut weight = [0; 2];
        weight.copy_from_slice(&bytes[2..4]);

        // The rest is the learn value, but it's not implemented.

        Self {
            mv: Move::from_u16(u16::from_be_bytes(mv)),
            weight: u16::from_be_bytes(weight)
        }
    }
}


// ================ PolyglotReader ====================
#[derive(Debug)]
pub struct PolyglotReader<I> {
    inner: I,
    len: u64
}

impl <I: Seek + Read> PolyglotReader<I> {
    pub fn new(inner: I) -> Result<Self, std::io::Error> {
        let mut inner = inner;

        Ok(Self {
            len: inner.seek(SeekFrom::End(0))? / PolyglotEntry::SIZE as u64,
            inner
        })
    }

    pub fn get(&mut self, key: &PolyglotKey) -> Result<Vec<PolyglotEntry>, std::io::Error> {
        let hash = key.polyglot_hash();
        let mut entry_exists = false;
        let mut left = 0;
        let mut right = self.len - 1;
        
        while left < right {
            let middle = (left + right) / 2;
            self.inner.seek(SeekFrom::Start(middle * PolyglotEntry::SIZE as u64))?;

            let mut entry_key = [0; 8];
            self.inner.read_exact(&mut entry_key)?;
            let entry_key = u64::from_be_bytes(entry_key);

            if entry_key < hash {
                left = middle + 1;
            } else {
                if entry_key == hash {
                    entry_exists = true;
                }
                right = middle;
            }
        }

        if !entry_exists {
            return Ok(Vec::new());
        }
        let lower_bound = left;
        
        left = 0;
        right = self.len - 1;
        while left < right {
            let middle = (left + right + 1) / 2;
            self.inner.seek(SeekFrom::Start(middle * PolyglotEntry::SIZE as u64))?;
            
            let mut entry_key = [0; 8];
            self.inner.read_exact(&mut entry_key)?;
            let entry_key = u64::from_be_bytes(entry_key);

            if entry_key > hash {
                right = middle - 1;
            } else {
                left = middle;
            }
        }

        let upper_bound = right + 1;
        
        let mut entries = vec![0; (upper_bound - lower_bound) as usize * PolyglotEntry::SIZE];
        self.inner.seek(SeekFrom::Start(lower_bound * PolyglotEntry::SIZE as u64))?;
        self.inner.read_exact(&mut entries)?;

        let entries = entries.chunks(PolyglotEntry::SIZE)
            .map(|entry| PolyglotEntry::from_bytes(&entry[8..]))
            .collect();
            
        Ok(entries)
    }
}


