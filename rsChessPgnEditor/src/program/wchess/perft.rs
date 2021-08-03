use std::time::{Instant};

use super::board;



pub fn perft_test(depth: i32, board: &mut board::Board) {

    let mut time: u128;
        let now = Instant::now();
        
        {
            println!("depth\ttime (MiliSecs.)\t\t\tnodes");
            println!("-----\t----------------\t\t\t-----");
            
            for i in 1..depth+1 {
                
                let nodes = mini_max(board, i, 0);
                time = now.elapsed().as_millis();
                println!("{}\t{}\t\t\t\t\t{}", i, time, nodes);
                
            }
            
        }
        
        time = now.elapsed().as_millis();    // it throws u128
        println!("Time used: {}", time);
}


fn mini_max (board: &mut board::Board, depth: i32, ply: usize) -> u64 {
    let mut nodes: u64 = 0;

    if depth == 0 { return 1; }
    
    // generate moves from this position
    board.move_buf_len[ply+1] = board.movegen(board.move_buf_len[ply]) as i32;

    
    // loop over moves:
    for i in board.move_buf_len[ply]..board.move_buf_len[ply+1] {

        board.make_move(board.move_buffer[i as usize]);
        {
            if !board.is_other_king_attacked() {
                nodes += mini_max(board, depth-1, ply+1);
            }
        }
        
        board.unmake_move(board.move_buffer[i as usize]);
    }

    nodes
}