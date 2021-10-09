use super::globs as gl;


pub fn mov_rankmoves (a: u32, target_bitmap: gl::BitMap, occupiedsq: gl::BitMap) -> u64 {  
    let x: u64;
    unsafe { 
        x = gl::RANK_ATTACKS[a as usize]
            [((occupiedsq & gl::RANKMASK[a as usize]) >> gl::RANKSHIFT[a as usize]) as usize] 
            & target_bitmap;
    }
    return x;
}


pub fn mov_filemoves (a: u32, target_bitmap: gl::BitMap, occupiedsq: gl::BitMap) -> u64 { 
    let x: u64;
    unsafe {
        x = gl::FILE_ATTACKS[a as usize]
            [(((occupiedsq & gl::FILEMASK[a as usize]) as i128 * 
                gl::FILEMAGIC[a as usize] as i128) as u64 >> 57) as usize] 
            & target_bitmap;
    }
    return x;
}


pub fn mov_slide_a8h1_moves(a: u32, target_bitmap: gl::BitMap, occupiedsq: gl::BitMap) -> u64 {
    let x: u64;
    unsafe {
        x = gl::DIAGA8H1_ATTACKS[a as usize]
            [((((occupiedsq & gl::DIAGA8H1MASK[a as usize]) as i128 * 
                gl::DIAGA8H1MAGIC[a as usize] as i128)) as u64 >> 57) as usize] 
            & target_bitmap;
    }
    return x;
}


pub fn mov_slide_a1h8_moves(a: u32, target_bitmap: gl::BitMap, occupiedsq: gl::BitMap) -> u64 {
    let x: u64;
    unsafe {
        x = gl::DIAGA1H8_ATTACKS[a as usize]
            [((((occupiedsq & gl::DIAGA1H8MASK[a as usize]) as i128 * 
                gl::DIAGA1H8MAGIC[a as usize] as i128)) as u64 >> 57) as usize] 
            & target_bitmap;
    }
    x
}


pub fn mov_bishopmoves(a: u32, target_bitmap: gl::BitMap, occupiedsq: gl::BitMap) -> u64 {
    
    let x = mov_slide_a8h1_moves(a, target_bitmap, occupiedsq) | 
            mov_slide_a1h8_moves(a, target_bitmap, occupiedsq);
    
    return x;
}


pub fn mov_rookmoves(a: u32, target_bitmap: gl::BitMap, occupiedsq: gl::BitMap) -> u64 {
    
    let x = mov_rankmoves(a, target_bitmap, occupiedsq) | 
            mov_filemoves(a, target_bitmap, occupiedsq);
    
    x
}


pub fn mov_queenmoves (a: u32, target_bitmap: gl::BitMap, occupiedsq: gl::BitMap) -> u64 {
    
    let x = mov_bishopmoves(a, target_bitmap, occupiedsq) |
            mov_rookmoves(a, target_bitmap, occupiedsq);
  
    x
}


// los valores constantes de la jugada de los enroques
pub fn mov_white_oo_castle() {
    let mut mov = Movim::new();
    mov.clear();
    mov.set_capt(gl::EMPTY as u32);
    mov.set_piec(gl::WHITE_KING as u32);
    mov.set_prom(gl::WHITE_KING as u32);
    mov.set_from(gl::E1 as u32);
    mov.set_to_sq(gl::G1 as u32);
    //gl::WHITE_OO_CASTL = move.moveInt;
    println!("white_00-castle: {} - {:#034b}", mov.move_int, mov.move_int);
}

pub fn mov_black_oo_castle() {
    let mut mov = Movim::new();
    mov.clear();
    mov.set_capt(gl::EMPTY as u32);
    mov.set_piec(gl::BLACK_KING as u32);
    mov.set_prom(gl::BLACK_KING as u32);
    mov.set_from(gl::E8 as u32);
    mov.set_to_sq(gl::G8 as u32);
    //gl::BLACK_OO_CASTL = move.moveInt;
    println!("black_00-castle: {} - {:#034b}", mov.move_int, mov.move_int);
}

pub fn mov_white_ooo_castle() {
    let mut mov = Movim::new();
    mov.clear();
    mov.set_capt(gl::EMPTY as u32);
    mov.set_piec(gl::WHITE_KING as u32);
    mov.set_prom(gl::WHITE_KING as u32);
    mov.set_from(gl::E1 as u32);
    mov.set_to_sq(gl::C1 as u32);
    //gl::WHITE_OOO_CASTL = move.moveInt;
    println!("white_000-castle: {} - {:#034b}", mov.move_int, mov.move_int);
}

pub fn mov_black_ooo_castle() {
    let mut mov = Movim::new();
    mov.clear();
    mov.set_capt(gl::EMPTY as u32);
    mov.set_piec(gl::BLACK_KING as u32);
    mov.set_prom(gl::BLACK_KING as u32);
    mov.set_from(gl::E8 as u32);
    mov.set_to_sq(gl::C8 as u32);
    //gl::BLACK_OO_CASTL = move.moveInt;
    println!("black_000-castle: {} - {:#034b}", mov.move_int, mov.move_int);
}





//  There are at least 3 different ways to store a move in max 32 bits
//  1) using shift & rank in an unsigned int
//  2) using 4 unsigned chars, union-ed with an unsigned int
//  3) using C++ bitfields, union-ed with an unsigned int

//  this is 1) using shift & rank in an unsigned int (32 bit):
#[derive(Clone, Copy, Debug)]
pub struct Movim {
    // from (6 bits)
    // tosq (6 bits)
    // piec (4 bits)
    // capt (4 bits)
    // prom (4 bits)
    pub move_int: u32,
}


impl Movim {
    pub fn new() -> Self {
        let m = Movim {
            move_int: 0,
        };

        m
    }

    pub fn clear(&mut self) {
        self.move_int = 0;
    }

    pub fn set_from(&mut self, from: u32) {   // bits  0.. 5
        self.move_int &= 0xffffffc0; 
        self.move_int |= from & 0x0000003f;
    }

    pub fn set_to_sq(&mut self, tosq: u32) {   // bits  6..11
        self.move_int &= 0xfffff03f; 
        self.move_int |= (tosq & 0x0000003f) << 6;
    }

    pub fn set_piec(&mut self, piec: u32) {   // bits 12..15
        self.move_int &= 0xffff0fff; 
        self.move_int |= (piec & 0x0000000f) << 12;
    }

    pub fn set_capt(&mut self, capt: u32) {   // bits 16..19
        self.move_int &= 0xfff0ffff; 
        self.move_int |= (capt & 0x0000000f) << 16;
    }

    pub fn set_prom(&mut self, prom: u32) {   // bits 20..23
        self.move_int &= 0xff0fffff; 
        self.move_int |= (prom & 0x0000000f) << 20;
    }

    // read move information:
    // first shift right, then mask to get the info

    pub fn get_from(self) -> u32 {   // 6 bits (value 0..63), position  0.. 5
        return self.move_int & 0x0000003f;
    }

    pub fn get_to_sq(self) -> u32 {   // 6 bits (value 0..63), position  6..11
        return (self.move_int >>  6) & 0x0000003f;
    }

    pub fn get_piec(self) -> u32{   // 4 bits (value 0..15), position 12..15
        return (self.move_int >> 12) & 0x0000000f;
    }

    pub fn get_capt(self) -> u32 {   // 4 bits (value 0..15), position 16..19
        return (self.move_int >> 16) & 0x0000000f;
    }

    pub fn get_prom(self) -> u32 {   // 4 bits (value 0..15), position 20..23
           return (self.move_int >> 20) & 0x0000000f;
    }

    // get move in uci and san formats

    pub fn get_uci(self) -> String{
        let mut uci_str: String = "".to_string();

        // castles
        if (self.get_piec() == gl::WHITE_KING as u32) && (self.is_castle_oo()) {
            uci_str.push_str("e1g1");
            return uci_str;
        };
        if (self.get_piec() == gl::WHITE_KING as u32) && (self.is_castle_ooo()) {
            uci_str.push_str("e1c1");
            return uci_str;
        };
        if (self.get_piec() == gl::BLACK_KING as u32) && (self.is_castle_oo()) {
            uci_str.push_str("e8g8");
            return uci_str;
        };
        if (self.get_piec() == gl::BLACK_KING as u32) && (self.is_castle_ooo()) {
            uci_str.push_str("e8c8");
            return uci_str;
        };

        //other type of moves
        // FROM
        // column
        if gl::FILES[self.get_from() as usize] == 1 { uci_str.push_str("a"); }
        if gl::FILES[self.get_from() as usize] == 2 { uci_str.push_str("b"); }
        if gl::FILES[self.get_from() as usize] == 3 { uci_str.push_str("c"); }
        if gl::FILES[self.get_from() as usize] == 4 { uci_str.push_str("d"); }
        if gl::FILES[self.get_from() as usize] == 5 { uci_str.push_str("e"); }
        if gl::FILES[self.get_from() as usize] == 6 { uci_str.push_str("f"); }
        if gl::FILES[self.get_from() as usize] == 7 { uci_str.push_str("g"); }
        if gl::FILES[self.get_from() as usize] == 8 { uci_str.push_str("h"); }
        // rank
        uci_str.push_str( gl::RANKS[self.get_from() as usize].to_string().as_str() );

        // TO
        // column
        if gl::FILES[self.get_to_sq() as usize] == 1 { uci_str.push_str("a"); }
        if gl::FILES[self.get_to_sq() as usize] == 2 { uci_str.push_str("b"); }
        if gl::FILES[self.get_to_sq() as usize] == 3 { uci_str.push_str("c"); }
        if gl::FILES[self.get_to_sq() as usize] == 4 { uci_str.push_str("d"); }
        if gl::FILES[self.get_to_sq() as usize] == 5 { uci_str.push_str("e"); }
        if gl::FILES[self.get_to_sq() as usize] == 6 { uci_str.push_str("f"); }
        if gl::FILES[self.get_to_sq() as usize] == 7 { uci_str.push_str("g"); }
        if gl::FILES[self.get_to_sq() as usize] == 8 { uci_str.push_str("h"); }
        // rank
        uci_str.push_str( gl::RANKS[self.get_to_sq() as usize].to_string().as_str() );

        if self.is_promotion() {
            if (self.get_prom() == gl::WHITE_ROOK as u32)   || (self.get_prom() == gl::BLACK_ROOK as u32) {
                uci_str.push_str("r");
            }
            if (self.get_prom() == gl::WHITE_BISHOP as u32) || (self.get_prom() == gl::BLACK_BISHOP as u32) {
                uci_str.push_str("b");
            }
            if (self.get_prom() == gl::WHITE_KNIGHT as u32) || (self.get_prom() == gl::BLACK_KNIGHT as u32) {
                uci_str.push_str("n");
            }
            // ¿?
            if (self.get_prom() == gl::WHITE_KING as u32)   || (self.get_prom() == gl::BLACK_KING as u32) {
                uci_str.push_str("k");
            }
            if (self.get_prom() == gl::WHITE_QUEEN as u32)  || (self.get_prom() == gl::BLACK_QUEEN as u32) {
                uci_str.push_str("q");
            }
        }
        uci_str
    }



    // boolean checks for some types of moves.
    // first mask, then compare
    // Note that we are using the bit-wise properties of piece identifiers, 
    // so we cannot just change them anymore !

    pub fn is_whitemove(self) -> gl::BOOLTYPE {   // piec is white: bit 15 must be 0
        return (!self.move_int & 0x00008000) == 0x00008000;
    }

    pub fn is_blackmove(self) -> bool {   // piec is black: bit 15 must be 1
        return ( self.move_int & 0x00008000) == 0x00008000;
    }

    pub fn is_capture(self) -> bool {   // capt is nonzero, bits 16 to 19 must be nonzero
        return ( self.move_int & 0x000f0000) != 0x00000000;
    }

    pub fn is_kingcaptured(self) -> bool {   // bits 17 to 19 must be 010
           return ( self.move_int & 0x00070000) == 0x00020000;
    }

    pub fn is_rookmove(self) -> bool {   // bits 13 to 15 must be 110
           return ( self.move_int & 0x00007000) == 0x00006000;
    }

    pub fn is_rookcaptured(self) -> bool {   // bits 17 to 19 must be 110
           return ( self.move_int & 0x00070000) == 0x00060000;
    }

    pub fn is_kingmove(self) -> bool {   // bits 13 to 15 must be 010
           return ( self.move_int & 0x00007000) == 0x00002000;
    }

    pub fn is_pawn_move(self) -> bool {   // bits 13 to 15 must be 001
           return ( self.move_int & 0x00007000) == 0x00001000;
    }

    pub fn is_pawn_doublemove(self) -> bool {   
        // bits 13 to 15 must be 001 &
        //     bits 4 to 6 must be 001 (from rank 2) & bits 10 to 12 must be 011 (to rank 4)
        // OR: bits 4 to 6 must be 110 (from rank 7) & bits 10 to 12 must be 100 (to rank 5)
    
        return 
            (( self.move_int & 0x00007000) == 0x00001000) && 
            (
                (
                    (( self.move_int & 0x00000038) == 0x00000008) && 
                    ((( self.move_int & 0x00000e00) == 0x00000600))
                ) || 
                (
                    (( self.move_int & 0x00000038) == 0x00000030) && 
                    ((( self.move_int & 0x00000e00) == 0x00000800))
                )
            
            );
    }

    pub fn is_enpassant(self) -> bool {   // prom is a pawn, bits 21 to 23 must be 001
           return ( self.move_int & 0x00700000) == 0x00100000;
    }

    pub fn is_promotion(self) -> bool {   // prom (with color bit removed), .xxx > 2 (not king or pawn)
           return ( self.move_int & 0x00700000) >  0x00200000;
    }

    pub fn is_castle(self) -> bool {   // prom is a king, bits 21 to 23 must be 010
           return ( self.move_int & 0x00700000) == 0x00200000;
    }

    pub fn is_castle_oo(self) -> bool {   // prom is a king and tosq is on the g-file
           return ( self.move_int & 0x007001c0) == 0x00200180;
    }

    pub fn is_castle_ooo(self) -> bool {   // prom is a king and tosq is on the c-file
           return ( self.move_int & 0x007001c0) == 0x00200080;
    }
}