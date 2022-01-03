/*
    Chess
*/

pub const DEFAULT_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/*
    TYPES
*/ 

pub const MAX_MOV_BUFF: usize = 4096;    // Number of moves that we can store (all plies)
pub const MAX_PLY: usize      = 64;      // Search depth
pub const MAX_GAME_LINE: usize= 4096;    // Number of moves in the (game + search) line that we can store

pub type BitMap = u64;
pub type BOOLTYPE = bool;    // == 0 => false; != 0 => true


/*
    GLOBALS
*/

pub const A8: usize = 56; pub const B8: usize = 57; pub const C8: usize = 58; pub const D8: usize = 59;
pub const E8: usize = 60; pub const F8: usize = 61; pub const G8: usize = 62; pub const H8: usize = 63;
pub const A7: usize = 48; pub const B7: usize = 49; pub const C7: usize = 50; pub const D7: usize = 51;
pub const E7: usize = 52; pub const F7: usize = 53; pub const G7: usize = 54; pub const H7: usize = 55;
pub const A6: usize = 40; pub const B6: usize = 41; pub const C6: usize = 42; pub const D6: usize = 43;
pub const E6: usize = 44; pub const F6: usize = 45; pub const G6: usize = 46; pub const H6: usize = 47;
pub const A5: usize = 32; pub const B5: usize = 33; pub const C5: usize = 34; pub const D5: usize = 35;
pub const E5: usize = 36; pub const F5: usize = 37; pub const G5: usize = 38; pub const H5: usize = 39;
pub const A4: usize = 24; pub const B4: usize = 25; pub const C4: usize = 26; pub const D4: usize = 27;
pub const E4: usize = 28; pub const F4: usize = 29; pub const G4: usize = 30; pub const H4: usize = 31;
pub const A3: usize = 16; pub const B3: usize = 17; pub const C3: usize = 18; pub const D3: usize = 19;
pub const E3: usize = 20; pub const F3: usize = 21; pub const G3: usize = 22; pub const H3: usize = 23;
pub const A2: usize =  8; pub const B2: usize =  9; pub const C2: usize = 10; pub const D2: usize = 11;
pub const E2: usize = 12; pub const F2: usize = 13; pub const G2: usize = 14; pub const H2: usize = 15;
pub const A1: usize =  0; pub const B1: usize =  1; pub const C1: usize =  2; pub const D1: usize =  3;
pub const E1: usize =  4; pub const F1: usize =  5; pub const G1: usize =  6; pub const H1: usize =  7;


pub const SQUARENAME: [&str; 64] = [
    "a1","b1","c1","d1","e1","f1","g1","h1",
    "a2","b2","c2","d2","e2","f2","g2","h2",
    "a3","b3","c3","d3","e3","f3","g3","h3",
    "a4","b4","c4","d4","e4","f4","g4","h4",
    "a5","b5","c5","d5","e5","f5","g5","h5",
    "a6","b6","c6","d6","e6","f6","g6","h6",
    "a7","b7","c7","d7","e7","f7","g7","h7",
    "a8","b8","c8","d8","e8","f8","g8","h8"
];


pub const FILES: [i32; 64] = [
    1, 2, 3, 4, 5, 6, 7, 8,
    1, 2, 3, 4, 5, 6, 7, 8,
    1, 2, 3, 4, 5, 6, 7, 8,
    1, 2, 3, 4, 5, 6, 7, 8,
    1, 2, 3, 4, 5, 6, 7, 8,
    1, 2, 3, 4, 5, 6, 7, 8,
    1, 2, 3, 4, 5, 6, 7, 8,
    1, 2, 3, 4, 5, 6, 7, 8
];

pub const RANKS: [i32; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6,
    7, 7, 7, 7, 7, 7, 7, 7,
    8, 8, 8, 8, 8, 8, 8, 8
];

pub const WHITE_MOVE: u8  = 0;
pub const BLACK_MOVE: u8  = 1;


// Piece identifiers, 4 bits each.
// Usefull bitwise properties of this numbering scheme:
// white = 0..., black = 1..., sliding = .1.., nonsliding = .0..
// rank/file sliding pieces = .11., diagonally sliding pieces = .1.1
// pawns and kings (without color bits), are < 3
// major pieces (without color bits set), are > 5
// minor and major pieces (without color bits set), are > 2
pub const EMPTY: u8 = 0;                //  0000
pub const WHITE_PAWN: u8 = 1;           //  0001
pub const WHITE_KING: u8 = 2;           //  0010
pub const WHITE_KNIGHT: u8 = 3;         //  0011
pub const WHITE_BISHOP: u8 =  5;        //  0101
pub const WHITE_ROOK: u8 = 6;           //  0110
pub const WHITE_QUEEN: u8 = 7;          //  0111
pub const BLACK_PAWN: u8 = 9;           //  1001
pub const BLACK_KING: u8 = 10;          //  1010
pub const BLACK_KNIGHT: u8 = 11;        //  1011
pub const BLACK_BISHOP: u8 = 13;        //  1101
pub const BLACK_ROOK: u8 = 14;          //  1110
pub const BLACK_QUEEN: u8 = 15;         //  1111

pub const PIECENAMES: [&str; 16] = ["  ","P ","K ","N ","  ","B ","R ","Q ",
                                "  ","P*","K*","N*","  ","B*","R*","Q*"];
pub const REAL_PIECENAMES: [&str; 16] = ["*","P","K","N","*","B","R","Q",
                                "*","p","k","n","*","b","r","q"];
pub const PIECECHARS: [&str; 16] = [" "," ","K","N"," ","B","R","Q"," "," ","K","N"," ","B","R","Q"];

pub static mut BITSET: [BitMap; 64] = [0_u64; 64];
pub static mut BOARDINDEX: [[i32; 9]; 9] = [[0_i32; 9]; 9]; // index 0 is not used, only 1..8.

// used in Eugene Nalimov's bitScanReverse
pub static mut MS1BTABLE: [i32; 256] = [0_i32; 256];

// ATTACK ARRAYS
// Attack tables:
pub static mut WHITE_PAWN_ATTACKS: [BitMap; 64] = [0; 64];
pub static mut WHITE_PAWN_MOVES: [BitMap; 64] = [0; 64];
pub static mut WHITE_PAWN_DOUBLE_MOVES: [BitMap; 64] = [0; 64];
pub static mut BLACK_PAWN_ATTACKS: [BitMap;64] = [0; 64];
pub static mut BLACK_PAWN_MOVES: [BitMap; 64] = [0; 64];
pub static mut BLACK_PAWN_DOUBLE_MOVES: [BitMap; 64] = [0; 64];
pub static mut KNIGHT_ATTACKS: [BitMap; 64] = [0; 64];
pub static mut KING_ATTACKS: [BitMap; 64] = [0; 64];
pub static mut RANK_ATTACKS: [[BitMap; 64];64] = [[0; 64];64];      // 32KB
pub static mut FILE_ATTACKS: [[BitMap; 64];64] = [[0; 64];64];      // 32KB
pub static mut DIAGA8H1_ATTACKS: [[BitMap; 64];64] = [[0; 64];64];  // 32KB
pub static mut DIAGA1H8_ATTACKS: [[BitMap; 64];64] = [[0; 64];64];  // 32KB

// Move generator shift for ranks:
pub const RANKSHIFT: [i32; 64] = [
        1,  1,  1,  1,  1,  1,  1,  1,
        9,  9,  9,  9,  9,  9,  9,  9,
       17, 17, 17, 17, 17, 17, 17, 17,
       25, 25, 25, 25, 25, 25, 25, 25,
       33, 33, 33, 33, 33, 33, 33, 33,
       41, 41, 41, 41, 41, 41, 41, 41,
       49, 49, 49, 49, 49, 49, 49, 49,
       57, 57, 57, 57, 57, 57, 57, 57
];

// Move generator magic multiplication numbers for files:
pub const _FILEMAGICS: [BitMap; 8] = [
       0x8040201008040200,
       0x4020100804020100,
       0x2010080402010080,
       0x1008040201008040,
       0x0804020100804020,
       0x0402010080402010,
       0x0201008040201008,
       0x0100804020100804
];

// Move generator magic multiplication numbers for diagonals:
pub const _DIAGA8H1MAGICS: [BitMap; 15] = [
       0x0,
       0x0,
       0x0101010101010100,
       0x0101010101010100,
       0x0101010101010100,
       0x0101010101010100,
       0x0101010101010100,
       0x0101010101010100,
       0x0080808080808080,
       0x0040404040404040,
       0x0020202020202020,
       0x0010101010101010,
       0x0008080808080808,
       0x0,
       0x0
];

// Move generator magic multiplication numbers for diagonals:
pub const _DIAGA1H8MAGICS: [BitMap; 15] = [
       0x0,
       0x0,
       0x0101010101010100,
       0x0101010101010100,
       0x0101010101010100,
       0x0101010101010100,
       0x0101010101010100,
       0x0101010101010100,
       0x8080808080808000,
       0x4040404040400000,
       0x2020202020000000,
       0x1010101000000000,
       0x0808080000000000,
       0x0,
       0x0
];

// Move generator 6-bit masking and magic multiplication numbers:
pub static mut RANKMASK: [BitMap; 64] = [0; 64];
pub static mut FILEMASK: [BitMap; 64] = [0; 64];
pub static mut FILEMAGIC: [BitMap; 64] = [0; 64];
pub static mut DIAGA8H1MASK: [BitMap; 64] = [0; 64];
pub static mut DIAGA8H1MAGIC: [BitMap; 64] = [0; 64];
pub static mut DIAGA1H8MASK: [BitMap; 64] = [0; 64];
pub static mut DIAGA1H8MAGIC: [BitMap; 64] = [0; 64];

// We use one generalized sliding attacks array: [8 squares][64 states]
// the unsigned char (=8 bits) contains the attacks for a rank, file or diagonal
//unsigned char GEN_SLIDING_ATTACKS[8][64];
pub static mut GEN_SLIDING_ATTACKS: [[u8; 64]; 8] = [[0; 64]; 8];



//     ===========================================================================
//     The 4 castling moves can be predefined: see function in movim.rs
//     ===========================================================================
    /*
    move.clear();
    move.setCapt(EMPTY);
    move.setPiec(WHITE_KING);
    move.setProm(WHITE_KING);
    move.setFrom(E1);
    move.setTosq(G1);
    WHITE_OO_CASTL = move.moveInt;
    move.setTosq(C1);
    WHITE_OOO_CASTL = move.moveInt;

    move.setPiec(BLACK_KING);
    move.setProm(BLACK_KING);
    move.setFrom(E8);
    move.setTosq(G8);
    BLACK_OO_CASTL = move.moveInt;
    move.setTosq(C8);
    BLACK_OOO_CASTL = move.moveInt;
    */



// Used for castling:
pub const CANCASTLEOO: u8 = 1;
pub const CANCASTLEOOO: u8 = 2;
// MASCARAS DE ENROQUE EG = E1-G1, etc...
pub static mut MASK_EG: [BitMap; 2] = [0; 2];
pub static mut MASK_FG: [BitMap; 2] = [0; 2];
pub static mut MASK_BD: [BitMap; 2] = [0; 2];
pub static mut MASK_CE: [BitMap; 2] = [0; 2];
// internal mov in 32 bit 
pub const WHITE_OOO_CASTL: i32 =  2105476;
pub const BLACK_OOO_CASTL: i32 = 10530492;
pub const WHITE_OO_CASTL: i32 =  2105732; 
pub const BLACK_OO_CASTL: i32 = 10530748;

pub static mut WHITE_SQUARES: BitMap = 0;
pub static mut BLACK_SQUARES: BitMap = 0;