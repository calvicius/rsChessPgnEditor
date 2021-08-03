use super::globs as gl;


pub fn data_init() {

    let mut charbitset: [u8; 8] = [0; 8];
    //let mut state6bit: u8; 
    let mut state8bit: u8;
    let mut attack8bit: u8;
    let mut slide: i32;
//     ===========================================================================
//     BITSET has only one bit set:
//     ===========================================================================
    
    unsafe {
        gl::BITSET[0] = 0x1;
        for i in 1..64 {
            gl::BITSET[i] = gl::BITSET[i-1] << 1;
        }
    }
    

//     ===========================================================================
//     BOARDINDEX is used to translate [file][rank] to [square],
//     Note that file is from 1..8 and rank from 1..8 (not starting from 0)
//     ===========================================================================
    
    unsafe {
        for rank in 0..9 {
            for file in 0..9 {
                gl::BOARDINDEX[file][rank] = (rank as i32-1) * 8 + file as i32 - 1;
            }
        }
    }
    

//     ===========================================================================
//     Initialize MS1BTABLE, used in lastOne (see bitops.rs)
//     ===========================================================================
    unsafe {
        for i in 0..256 {
            
            if i > 127 {
                gl::MS1BTABLE[i] = 7;
            }
            else if i > 63 {
                gl::MS1BTABLE[i] = 6;
            }
            else if i > 31 {
                gl::MS1BTABLE[i] = 5;
            }
            else if i > 15 {
                gl::MS1BTABLE[i] = 4;
            }
            else if i > 7 {
                gl::MS1BTABLE[i] = 3;
            }
            else if i > 3 {
                gl::MS1BTABLE[i] = 2;
            }
            else if i > 1 {
                gl::MS1BTABLE[i] = 1;
            }
            else {
                gl::MS1BTABLE[i] = 0;
            }
            /*
            MS1BTABLE[i] = (
                (i > 127) ? 7 :
                (i >  63) ? 6 :
                (i >  31) ? 5 :
                (i >  15) ? 4 :
                (i >   7) ? 3 :
                (i >   3) ? 2 :
                (i >   1) ? 1 : 0 );
            */
        }
    }


//     ===========================================================================
//     Initialize rank, file and diagonal 6-bit masking bitmaps, to get the
//     occupancy state, used in the movegenerator (see movegen.rs)
//     ===========================================================================

    for square in 0..64 {
        unsafe {
            gl::RANKMASK[square] = 0x0;
            gl::FILEMASK[square] = 0x0;
            gl::DIAGA8H1MASK[square] = 0x0;
            gl::DIAGA1H8MASK[square] = 0x0;
            gl::FILEMAGIC[square] = 0x0;
            gl::DIAGA8H1MAGIC[square] = 0x0;
            gl::DIAGA1H8MAGIC[square] = 0x0;
        }
    }

    for file in 1..9 {
        for rank in 1..9 {
          unsafe{
            // ===========================================================================
            //  initialize 6-bit rank mask, used in the movegenerator (see movegen.rs)
            // ===========================================================================
            gl::RANKMASK[gl::BOARDINDEX[file][rank] as usize]  = 
                gl::BITSET[gl::BOARDINDEX[2][rank] as usize] | 
                gl::BITSET[gl::BOARDINDEX[3][rank] as usize] | 
                gl::BITSET[gl::BOARDINDEX[4][rank] as usize] ;
            gl::RANKMASK[gl::BOARDINDEX[file][rank] as usize] |= 
                gl::BITSET[gl::BOARDINDEX[5][rank] as usize] | 
                gl::BITSET[gl::BOARDINDEX[6][rank] as usize] | 
                gl::BITSET[gl::BOARDINDEX[7][rank] as usize] ;

            // ===========================================================================
            //   initialize 6-bit file mask, used in the movegenerator (see movegen)
            // ===========================================================================
            gl::FILEMASK[gl::BOARDINDEX[file][rank] as usize]  = 
                gl::BITSET[gl::BOARDINDEX[file][2] as usize] | 
                gl::BITSET[gl::BOARDINDEX[file][3] as usize] | 
                gl::BITSET[gl::BOARDINDEX[file][4] as usize] ;
            gl::FILEMASK[gl::BOARDINDEX[file][rank] as usize] |= 
                gl::BITSET[gl::BOARDINDEX[file][5] as usize] | 
                gl::BITSET[gl::BOARDINDEX[file][6] as usize] | 
                gl::BITSET[gl::BOARDINDEX[file][7] as usize] ;

            // ===========================================================================
            // Initialize diagonal magic multiplication numbers, used in the movegenerator (see movegen)
            // ===========================================================================
            let diaga8h1: i32 = (file + rank) as i32; // from 2 to 16, longest diagonal = 9
            gl::DIAGA8H1MAGIC[gl::BOARDINDEX[file][rank] as usize] = 
                    gl::_DIAGA8H1MAGICS[(diaga8h1 - 2) as usize];

            // ===========================================================================
            //  Initialize 6-bit diagonal mask, used in the movegenerator (see movegen)
            // ===========================================================================
            gl::DIAGA8H1MASK[gl::BOARDINDEX[file][rank] as usize] = 0x0;
            if diaga8h1 < 10 {  // lower half, diagonals 2 to 9
                //for (square = 2 ; square < diaga8h1-1 ; square ++)
                for square in 2..diaga8h1-1 {
                    gl::DIAGA8H1MASK[gl::BOARDINDEX[file][rank] as usize] |= 
                        gl::BITSET[gl::BOARDINDEX[square as usize][(diaga8h1-square as i32) as usize] as usize];
                }
            }
            else  {     // upper half, diagonals 10 to 16
                //for (square = 2 ; square < 17 - diaga8h1 ; square ++)
                for square in 2..(17 - diaga8h1) {
                    gl::DIAGA8H1MASK[gl::BOARDINDEX[file][rank] as usize] |= 
                        gl::BITSET[gl::BOARDINDEX[(diaga8h1+square-9) as usize][(9-square) as usize] as usize];
                }
            }

            // ===========================================================================
            // Initialize diagonal magic multiplication numbers, used in the movegenerator (see movegen)
            // ===========================================================================
            let diaga1h8: i32 = file as i32 - rank as i32; // from -7 to +7, longest diagonal = 0
            gl::DIAGA1H8MAGIC[gl::BOARDINDEX[file][rank] as usize] = 
                    gl::_DIAGA1H8MAGICS[(diaga1h8+7) as usize];

            // ===========================================================================
            // Initialize 6-bit diagonal mask, used in the movegenerator (see movegen)
            // ===========================================================================
            gl::DIAGA1H8MASK[gl::BOARDINDEX[file][rank] as usize] = 0x0;
            if diaga1h8 > -1 { // lower half, diagonals 0 to 7
                //for (square = 2 ; square < 8 - diaga1h8 ; square ++)
                for square in 2..(8 - diaga1h8) {
                    gl::DIAGA1H8MASK[gl::BOARDINDEX[file][rank] as usize] |= 
                        gl::BITSET[gl::BOARDINDEX[(diaga1h8 + square) as usize][square as usize] as usize];
                }
            }
            else {
                //for (square = 2 ; square < 8 + diaga1h8 ; square ++)
                for square in 2..(8 + diaga1h8) {
                    gl::DIAGA1H8MASK[gl::BOARDINDEX[file][rank] as usize] |= 
                        gl::BITSET[gl::BOARDINDEX[square as usize][(square - diaga1h8) as usize] as usize];
                }
            }

            // ===========================================================================
            // Initialize file magic multiplication numbers, used in the movegenerator (see movegen.ccp)
            // ===========================================================================
            gl::FILEMAGIC[gl::BOARDINDEX[file][rank] as usize] = gl::_FILEMAGICS[file-1];
            
          } // end unsafe
        }
    }

//     ===========================================================================
//     Now initialize the GEN_SLIDING_ATTACKS array, used to generate the sliding
//     attack bitboards.
//     unsigned char GEN_SLIDING_ATTACKS[8 squares][64 states] holds the attacks
//     for any file, rank or diagonal - it is going to be usefull when generating the
//     RANK_ATTACKS[64][64], FILE_ATTACKS[64][64], DIAGA8H1_ATTACKS[64][64] and
//     DIAGA1H8_ATTACKS[64][64] arrays
//     ===========================================================================

    // initialize CHARBITSET, this array is equivalant to BITSET for bitboards:
    // 8 chars, each with only 1 bit set.
    charbitset[0] = 1;
    //for (square = 1; square <= 7; square++)
    for square in 1..=7 {
            charbitset[square] = charbitset[square-1] << 1;
    }

    // loop over rank, file or diagonal squares:
    //for (square = 0; square <= 7; square++)
    for square in 0..=7 {
           // loop of occupancy states
           // state6Bit represents the 64 possible occupancy states of a rank,
           // except the 2 end-bits, because they don't matter for calculating attacks
           //for (state6Bit = 0; state6Bit < 64; state6Bit++)
           for state6bit in 0..64 {
                state8bit = state6bit << 1; // create an 8-bit occupancy state
                attack8bit = 0;
                if square < 7 {
                    attack8bit |= charbitset[square + 1];
                }
                slide = square as i32 + 2;
                while slide <= 7 { // slide in '+' direction
                    if ((!state8bit) & (charbitset[(slide - 1) as usize])) != 0 {
                            attack8bit |= charbitset[slide as usize];
                    }
                    else { break; }
                    slide += 1;
                }
                if square > 0 {
                    attack8bit |= charbitset[(square - 1) as usize];
                }
                slide = square as i32 - 2;
                while slide >= 0 { // slide in '-' direction
                    if ((!state8bit) & (charbitset[(slide + 1) as usize])) != 0 {
                            attack8bit |= charbitset[slide as usize];
                    }
                    else { break; }
                    slide -= 1;
                }
                unsafe {
                gl::GEN_SLIDING_ATTACKS[square as usize][state6bit as usize] = attack8bit;
                }
           }
    }

//     ===========================================================================
//     Initialize all attack bitmaps, used in the movegenerator (see movegen.ccp)
//     ===========================================================================

    for square in 0..64 {
      unsafe {
        gl::KNIGHT_ATTACKS[square] = 0x0;
        gl::KING_ATTACKS[square] = 0x0;
        gl::WHITE_PAWN_ATTACKS[square] = 0x0;
        gl::WHITE_PAWN_MOVES[square] = 0x0;
        gl::WHITE_PAWN_DOUBLE_MOVES[square] = 0x0;
        gl::BLACK_PAWN_ATTACKS[square] = 0x0;
        gl::BLACK_PAWN_MOVES[square] = 0x0;
        gl::BLACK_PAWN_DOUBLE_MOVES[square] = 0x0;
        for state in 0..64 {
            gl::RANK_ATTACKS[square][state] = 0x0;
            gl::FILE_ATTACKS[square][state] = 0x0;
            gl::DIAGA8H1_ATTACKS[square][state] = 0x0;
            gl::DIAGA1H8_ATTACKS[square][state] = 0x0;
        }
      } //end unsafe
    }

    // WHITE_PAWN_ATTACKS
    for square in 0..64 {
        let file = gl::FILES[square]; 
        let rank = gl::RANKS[square];
        let mut afile = file - 1; 
        let mut arank = rank + 1;
      unsafe {
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::WHITE_PAWN_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 1; 
        arank = rank + 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
                gl::WHITE_PAWN_ATTACKS[square] |= 
                    gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
      }   // end unsafe
    }

    // WHITE_PAWN_MOVES
    for square in 0..64 {
        let file = gl::FILES[square]; 
        let rank = gl::RANKS[square];
        let mut afile = file; 
        let mut arank = rank + 1;
      unsafe {
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::WHITE_PAWN_MOVES[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        if rank == 2 {
            afile = file; 
            arank = rank + 2;
            if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
                gl::WHITE_PAWN_DOUBLE_MOVES[square] |= 
                    gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
            }
        }
      }  //end unsafe
    }

    // BLACK_PAWN_ATTACKS
    for square in 0..64 {
        let file = gl::FILES[square]; 
        let rank = gl::RANKS[square];
        let mut afile = file - 1; 
        let mut arank = rank - 1;
      unsafe {
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::BLACK_PAWN_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 1; 
        arank = rank - 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::BLACK_PAWN_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
      } // end unsafe
    }

    // BLACK_PAWN_MOVES
    for square in 0..64 {
        let file = gl::FILES[square]; 
        let rank = gl::RANKS[square];
        let mut afile = file; 
        let mut arank = rank - 1;
      unsafe {
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::BLACK_PAWN_MOVES[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        if rank == 7 {
            afile = file; arank = rank - 2;
            if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
                gl::BLACK_PAWN_DOUBLE_MOVES[square] |= 
                    gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
            }
        }
      }  // end unsafe
    }

    // KNIGHT attacks;
    for square in 0..64 {
        let file = gl::FILES[square];
        let rank = gl::RANKS[square];
        let mut afile = file - 2; 
        let mut arank = rank + 1;
      unsafe {
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KNIGHT_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file - 1; 
        arank = rank + 2;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KNIGHT_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 1; 
        arank = rank + 2;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KNIGHT_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 2; 
        arank = rank + 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KNIGHT_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 2; 
        arank = rank - 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KNIGHT_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 1; 
        arank = rank - 2;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KNIGHT_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file - 1; 
        arank = rank - 2;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KNIGHT_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file - 2; 
        arank = rank - 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KNIGHT_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
      } // end unsafe
    }

    // KING attacks;
    for square in 0..64 {
        let file = gl::FILES[square]; 
        let rank = gl::RANKS[square];
        let mut afile = file - 1; 
        let mut arank = rank;
      unsafe {
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KING_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file - 1; 
        arank = rank + 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KING_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file; 
        arank = rank + 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KING_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 1; 
        arank = rank + 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KING_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 1; 
        arank = rank;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KING_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file + 1; 
        arank = rank - 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KING_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file; 
        arank = rank - 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KING_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
        afile = file - 1; 
        arank = rank - 1;
        if (afile >= 1) & (afile <= 8) & (arank >= 1) & (arank <= 8) {
            gl::KING_ATTACKS[square] |= 
                gl::BITSET[gl::BOARDINDEX[afile as usize][arank as usize] as usize];
        }
      } //end unsafe
    }

    //  RANK attacks (ROOKS and QUEENS):
    //  use           unsigned char GEN_SLIDING_ATTACKS[8 squares] [64 states]
    //  to initialize BitMap        RANK_ATTACKS       [64 squares][64 states]
    //
    for square in 0..64 {
        for state6bit in 0..64 {
          unsafe {
            gl::RANK_ATTACKS[square][state6bit] = 0;
            gl::RANK_ATTACKS[square][state6bit] |=
                ((gl::GEN_SLIDING_ATTACKS[(gl::FILES[square]-1) as usize][state6bit]) as gl::BitMap) << 
                    (gl::RANKSHIFT[square] - 1);
          }  // end unsafe
        }
    }

    //  FILE attacks (ROOKS and QUEENS):
    //  use           unsigned char GEN_SLIDING_ATTACKS[8 squares] [64 states]
    //  to initialize BitMap        FILE_ATTACKS       [64 squares][64 states]
    //
    //  Occupancy transformation is as follows:
    //
    //   occupancy state bits of the file:               occupancy state bits in GEN_SLIDING_ATTACKS:
    //
    //        . . . . . . . . MSB                           LSB         MSB
    //        . . . . . A . .                    =>         A B C D E F . .
    //        . . . . . B . .
    //        . . . . . C . .
    //        . . . . . D . .
    //        . . . . . E . .
    //        . . . . . F . .
    //    LSB . . . . . . . .
    //
    //  The reverse transformation is as follows:
    //
    //   attack bits in GEN_SLIDING_ATTACKS:             attack bits in the file:
    //
    //        LSB         MSB                               . . . . . m . . MSB
    //        m n o p q r s t                    =>         . . . . . n . .
    //                                                      . . . . . o . .
    //                                                      . . . . . p . .
    //                                                      . . . . . q . .
    //                                                      . . . . . r . .
    //                                                      . . . . . s . .
    //                                                 LSB  . . . . . t . .
    //
    for square in 0..64 {
        for state6bit in 0..64 {
          unsafe {
            gl::FILE_ATTACKS[square][state6bit] = 0x0;
            // check to see if attackbit'-th  bit is set in GEN_SLIDING_ATTACKS, 
            // for this combination of square/occupancy state
            for attackbit in 0..8 { // from LSB to MSB
                // conversion from 64 board squares to the 8 corresponding 
                // positions in the GEN_SLIDING_ATTACKS array: "8-RANKS[square]"
                if gl::GEN_SLIDING_ATTACKS[(8-gl::RANKS[square]) as usize][state6bit] & 
                        charbitset[attackbit] != 0 {
                    // the bit is set, so we need to update FILE_ATTACKS accordingly:
                    // conversion of square/attackbit to the corresponding 64 board FILE: FILES[square]
                    // conversion of square/attackbit to the corresponding 64 board RANK: 8-attackbit
                    let file = gl::FILES[square];
                    let rank = 8 - attackbit;
                    gl::FILE_ATTACKS[square][state6bit] |=  
                        gl::BITSET[gl::BOARDINDEX[file as usize][rank as usize] as usize];
                }
            }
          } // end unsafe
        }
    }

    //  DIAGA8H1_ATTACKS attacks (BISHOPS and QUEENS):
    for square in 0..64 {
        for state6bit in 0..64 {
          unsafe {
            gl::DIAGA8H1_ATTACKS[square][state6bit] = 0x0;
            for attackbit in 0..8 { // from LSB to MSB
                let file: i32;
                let rank: i32;
                // conversion from 64 board squares to the 8 corresponding positions in 
                // the GEN_SLIDING_ATTACKS array: MIN((8-RANKS[square]),(FILES[square]-1))
                //if (GEN_SLIDING_ATTACKS
                //        [(8-RANKS[square]) < (FILES[square]-1) ? (8-RANKS[square]) : (gl::FILES[square]-1)]
                //        [state6Bit] & CHARBITSET[attackbit])
                let idx: usize;
                if (8-gl::RANKS[square]) < (gl::FILES[square]-1) {
                    idx = (8-gl::RANKS[square]) as usize ;
                }
                else {
                    idx = (gl::FILES[square]-1) as usize;
                }
                if gl::GEN_SLIDING_ATTACKS[idx][state6bit] & charbitset[attackbit] != 0 {
                    // the bit is set, so we need to update FILE_ATTACKS accordingly:
                    // conversion of square/attackbit to the corresponding 64 board file and rank:
                    let diaga8h1 = gl::FILES[square] + gl::RANKS[square]; // from 2 to 16, longest diagonal = 9
                    if diaga8h1 < 10 {
                        file = (attackbit + 1) as i32;
                        rank = diaga8h1 - file;
                    }
                    else
                    {
                        rank = 8 - attackbit as i32;
                        file = diaga8h1 - rank;
                    }
                    if (file > 0) && (file < 9) && (rank > 0) && (rank < 9) {
                        gl::DIAGA8H1_ATTACKS[square][state6bit] |=  
                            gl::BITSET[gl::BOARDINDEX[file as usize][rank as usize] as usize];
                    }
                }
            }
          }  // end unsafe
        }
    }

    //  DIAGA1H8_ATTACKS attacks (BISHOPS and QUEENS):
    for square in 0..64 {
        for state6bit in 0..64 {
          unsafe {
            gl::DIAGA1H8_ATTACKS[square][state6bit] = 0x0;
            for attackbit in 0..8 { // from LSB to MSB
                let file: i32;
                let rank: i32;
                // conversion from 64 board squares to the 8 corresponding positions in 
                // the GEN_SLIDING_ATTACKS array: MIN((8-RANKS[square]),(FILES[square]-1))
                //if (GEN_SLIDING_ATTACKS
                //[(RANKS[square]-1) < (FILES[square]-1) ? (RANKS[square]-1) : (FILES[square]-1)]
                //[state6Bit] & CHARBITSET[attackbit])
                let idx: usize;
                if (gl::RANKS[square]-1) < (gl::FILES[square]-1) {
                    idx = (gl::RANKS[square]-1) as usize ;
                }
                else {
                    idx = (gl::FILES[square]-1) as usize;
                }
                if gl::GEN_SLIDING_ATTACKS[idx][state6bit] & charbitset[attackbit] != 0{
                    // the bit is set, so we need to update FILE_ATTACKS accordingly:
                    // conversion of square/attackbit to the corresponding 64 board file and rank:
                    let diaga1h8 = gl::FILES[square] - gl::RANKS[square]; // from -7 to 7, longest diagonal = 0
                    if diaga1h8 < 0 {
                        file = (attackbit + 1) as i32;
                        rank = file - diaga1h8;
                    }
                    else
                    {
                        rank = (attackbit + 1) as i32;
                        file = diaga1h8 + rank;
                    }
                    if (file > 0) && (file < 9) && (rank > 0) && (rank < 9) {
                        gl::DIAGA1H8_ATTACKS[square][state6bit] |=  
                            gl::BITSET[gl::BOARDINDEX[file as usize][rank as usize] as usize];
                    }
                }
            }
          } // end unsafe
        }
    }

//     ===========================================================================
//     Masks for castling, index 0 is for white, 1 is for black
//     ===========================================================================
    unsafe {
        gl::MASK_EG[0] = gl::BITSET[gl::E1] | gl::BITSET[gl::F1] | gl::BITSET[gl::G1];
        gl::MASK_EG[1] = gl::BITSET[gl::E8] | gl::BITSET[gl::F8] | gl::BITSET[gl::G8];

        gl::MASK_FG[0] = gl::BITSET[gl::F1] | gl::BITSET[gl::G1];
        gl::MASK_FG[1] = gl::BITSET[gl::F8] | gl::BITSET[gl::G8];

        gl::MASK_BD[0] = gl::BITSET[gl::B1] | gl::BITSET[gl::C1] | gl::BITSET[gl::D1];
        gl::MASK_BD[1] = gl::BITSET[gl::B8] | gl::BITSET[gl::C8] | gl::BITSET[gl::D8];

        gl::MASK_CE[0] = gl::BITSET[gl::C1] | gl::BITSET[gl::D1] | gl::BITSET[gl::E1];
        gl::MASK_CE[1] = gl::BITSET[gl::C8] | gl::BITSET[gl::D8] | gl::BITSET[gl::E8];
    }

//     ===========================================================================
//     Initialize other bitmaps
//     ===========================================================================
        unsafe {
            gl::BLACK_SQUARES = 0;
            for i in 0..64 {
                if ((i as i32 + gl::RANKS[i]) % 2) != 0 {
                    gl::BLACK_SQUARES ^= gl::BITSET[i];
                }
            }
            
            gl::WHITE_SQUARES = !gl::BLACK_SQUARES;
        }

}
