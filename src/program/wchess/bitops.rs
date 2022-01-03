#[allow(overflowing_literals)]

use std::convert::TryInto;

use super::globs;


pub fn bit_cnt(mut bitmap: u64) -> i32 {

// MIT HAKMEM algorithm, see http://graphics.stanford.edu/~seander/bithacks.html

       const  M1: u64 = 0x5555555555555555;  // 1 zero,  1 one ...
       const  M2: u64 = 0x3333333333333333;  // 2 zeros,  2 ones ...
       const  M4: u64 = 0x0f0f0f0f0f0f0f0f;  // 4 zeros,  4 ones ...
       const  M8: u64 = 0x00ff00ff00ff00ff;  // 8 zeros,  8 ones ...
       const M16: u64 = 0x0000ffff0000ffff;  // 16 zeros, 16 ones ...
       const M32: u64 = 0x00000000ffffffff;  // 32 zeros, 32 ones

    bitmap = (bitmap & M1 ) + ((bitmap >>  1) & M1 );   //put count of each  2 bits into those  2 bits
    bitmap = (bitmap & M2 ) + ((bitmap >>  2) & M2 );   //put count of each  4 bits into those  4 bits
    bitmap = (bitmap & M4 ) + ((bitmap >>  4) & M4 );   //put count of each  8 bits into those  8 bits
    bitmap = (bitmap & M8 ) + ((bitmap >>  8) & M8 );   //put count of each 16 bits into those 16 bits
    bitmap = (bitmap & M16) + ((bitmap >> 16) & M16);   //put count of each 32 bits into those 32 bits
    bitmap = (bitmap & M32) + ((bitmap >> 32) & M32);   //put count of each 64 bits into those 64 bits
    return bitmap as i32;
}


pub fn last_one(mut bitmap: u64) -> u32 {
       // this is Eugene Nalimov's bitScanReverse
       // use firstOne if you can, it is faster than lastOne.
       // don't use this if bitmap = 0

       let mut result: i32 = 0;
       if bitmap > 0xFFFFFFFF {
              bitmap >>= 32;
              result = 32;
       }
       if bitmap > 0xFFFF {
              bitmap >>= 16;
              result += 16;
       }
       if bitmap > 0xFF {
              bitmap >>= 8;
              result += 8;
       }

       let result1: i32;
       unsafe {
              result1 = result + globs::MS1BTABLE[bitmap as usize];
       }
       result1.try_into().unwrap()
}


pub fn first_one(bitmap: u64) -> u32 {
       // De Bruijn Multiplication, 
       // https://www.chessprogramming.org/BitScan#De_Bruijn_Multiplication
       // don't use this if bitmap = 0

       const INDEX64: [u32; 64] = [
       63,  0, 58,  1, 59, 47, 53,  2,
       60, 39, 48, 27, 54, 33, 42,  3,
       61, 51, 37, 40, 49, 18, 28, 20,
       55, 30, 34, 11, 43, 14, 22,  4,
       62, 57, 46, 52, 38, 26, 32, 41,
       50, 36, 17, 19, 29, 10, 13, 21,
       56, 45, 25, 31, 35, 16,  9, 12,
       44, 24, 15,  8, 23,  7,  6,  5  ];

       const DEBRUIJN64: u64 = (0x07EDD5E59A4E28C2) as u64;

       // here you would get a warming: "unary minus operator applied to unsigned type",
       // that's intended and OK so I'll disable it

       return INDEX64[(((bitmap as i128 & -(bitmap as i128)) * DEBRUIJN64 as i128) as u64 >> 58) as usize];
       
}