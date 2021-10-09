use std::collections::HashMap;
//use std::thread;

use gtk::*;
//use gdk;
//use glib::Cast;

use super::globs::{DbEntry, GameInfo, ListNodes, Node};
use super::pgn as ph;
use super::wchess;
use super::utils;
use super::notationview::{select_node_exists, select_node_noexists, reset_buffer};
use super::levels as lh;
//use super::tags_textbuffer as ttbuf;


// Variables globales del taablero
pub const Y_EJE: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const X_EJE: [i32; 8] = [1,2,3,4,5,6,7,8];

pub static mut CABECERA: Option<GameInfo> = None;
pub static mut NODOS: Option<ListNodes> = None;
pub static mut FEN_ACTUAL: Option<String> = None;
pub static mut FLIPPED: bool = false;
pub static mut NODO_ACTIVO: Option<String> = None;



	
fn res_to_enum(res: String) -> String {
    
    match res.as_str() {
        "1-0" => return "1".to_string(),
        "1/2-1/2" => return "2".to_string(),
        "0-1" => return String::from("3"),
        "*" => return String::from("4"),
        _ => return String::from("4"),
    }
}

//static mut PGNDATA: Option<PgnGame> = None;
pub fn read_nodes_from_file(entry: &mut DbEntry) -> ListNodes {
    /*
    let mut entry_clon = entry.clone();
    let child = thread::spawn(move || {
        
        let pgngame = ph::parse_pgn_data(&mut entry_clon.pgn);
        unsafe { PGNDATA = Some(pgngame) }
    });
    let _ = child.join();
    let pgn_data: PgnGame;
    */
    let pgn_data = ph::parse_pgn_data(&mut entry.pgn);
    unsafe {
        
        //PGNDATA = Some(pgn_data);
        FEN_ACTUAL = Some(pgn_data.fen.clone());
    }

    entry.game_info.white = pgn_data.white;
    entry.game_info.elow = pgn_data.elow;
    entry.game_info.black = pgn_data.black;
    entry.game_info.elob = pgn_data.elob;
    entry.game_info.res = res_to_enum(pgn_data.res);
    entry.game_info.event = pgn_data.event;
    entry.game_info.site = pgn_data.site;
    entry.game_info.round = pgn_data.round;
    entry.game_info.date = pgn_data.date;
    entry.game_info.fen = pgn_data.fen.clone();
    entry.game_info.tags = pgn_data.tags;
    entry.game_info.eco = pgn_data.eco;
    
    //entry.game_info = game_info;
    //entry.id = 1;		// siempre numero de partida =1 o num. reg. of bd
    let nodes: ListNodes = ph::pgn_moves_to_nodes(pgn_data.moves, pgn_data.fen);

    unsafe {
        CABECERA = Some(entry.clone().game_info);
        NODOS = Some(nodes.clone());
    }

    return nodes;
}


pub fn xb_make_move (area:&gtk::DrawingArea,
                    view: &gtk::TextView,
                    from: String, to: String ) {
    
    let mut b = wchess::new_board();
    let current_fen: String;
    unsafe {
        current_fen = FEN_ACTUAL.clone().unwrap();
    } 
    wchess::set_fen(&mut b, &current_fen);

    let piece: String = wchess::get_piece_at_alphasquare(&mut b, &from);

    if piece == "None".to_string() {
        let no_piece_at = format!("No hay pieza en la casilla \no no es el turno de este color");
        utils::alerta(&no_piece_at);
        return;
    }

    let movim: wchess::legalmoves::LegalMove;
    if (piece == "P".to_string() || piece == "p".to_string()) &&
                (&to[1..] == "8" || &to[1..] == "1") {
        // TODO : some popup to select promoted piece
        // now is only Q
        //let prom = "q";
        let prom = utils::popup_promotion();
        let uci_string = format!("{}{}{}", from, to, prom);
        movim = wchess::move_uci(&mut b, &uci_string);
        if movim.movim.move_int == 0 {
            let error_move = format!("La jugada {} es erronea  ", uci_string);
            utils::alerta(&error_move);
            return;
        }
    }
    else {
        let uci_string = format!("{}{}", from, to);
        movim = wchess::move_uci(&mut b, &uci_string);
        if movim.movim.move_int == 0 {
            let error_move = format!("La jugada {} es erronea  ", uci_string);
            utils::alerta(&error_move);
            return;
        }
    }
    
    let vfen = wchess::get_fen(&mut b);
    let mut nodes: ListNodes;
    let node_indx: String;
    unsafe {
        nodes = NODOS.clone().unwrap();
        if NODO_ACTIVO.is_some() {
            node_indx = NODO_ACTIVO.clone().unwrap();
        }
        else {
            NODO_ACTIVO = Some("1z".to_string());
            node_indx = "1z".to_string();
        }
        FEN_ACTUAL = Some(vfen.clone());
    }
    
    // update board_area
    area.queue_draw();
    while gtk::events_pending() {
        gtk::main_iteration();
    }

    let node_idx = node_indx.clone();
    
    let clon2 = node_idx.clone();
    let children = nodes.nodes[&node_idx.clone()].children.clone();

    let nods = nodes.clone();
    for i in 0..children.len() {
        //let nods = nodes.clone();
        
        if nods.nodes[&children[i]].fen == vfen.clone() {
            
            // avanzamos en textview para posicionarnos en la jugada que coincide
            select_node_exists (nodes.clone(), view, &area, children[i].clone());
            return;
        }
    }
    
    // inserta un movimiento nuevo
    let cur_node = nodes.nodes[&node_idx.clone()].clone();

    let new_node_indx = lh::get_next_sibling_indx(clon2.clone(), children.len() as i32);

    nodes.push_children(node_idx.clone(), new_node_indx.clone());
    let children1 = nodes.nodes[&node_idx.clone()].children.clone();

    nodes.init_node(new_node_indx.clone());
    nodes.set_fen(new_node_indx.clone(), vfen.clone());
    nodes.set_san(new_node_indx.clone(), movim.clone().san);
    nodes.init_children(new_node_indx.clone());
    nodes.set_parent_indx(new_node_indx.clone(), node_idx.clone());

    if children1.len() == 1 {
        nodes.set_branch_level(new_node_indx.clone(), cur_node.branch_level);
    } else {
        nodes.set_branch_level(new_node_indx.clone(), cur_node.branch_level+1);
    }
    
    reset_buffer(&view);

    super::board_gui::draw_notation(view, &mut nodes.clone());
    
    // avanzamos en textview para posicionarnos en la jugada que coincide
    select_node_noexists (nodes.clone(), view, &area, new_node_indx.clone());

}


/************************************
 * STRIP VARIATION
 */

pub fn strip_variation_handler (nodefrom: String) {

        let (new_nodes, new_node_indx) = strip_variation(nodefrom);

        unsafe {
            if new_nodes.nodes.len() > 1 {
                NODOS = Some(new_nodes);
                NODO_ACTIVO = Some(new_node_indx);
            }
        }
    
}



pub fn strip_variation(nodefrom: String)  -> (ListNodes, String) {
    let selected_node;  // = nodefrom.clone();
    let mut old_nodes: ListNodes;
    unsafe {
        old_nodes = NODOS.clone().unwrap();
        selected_node = NODO_ACTIVO.clone().unwrap();
    }

    if selected_node == lh::root_node() {
        return (old_nodes, nodefrom.clone());
    }

    let mut new_nodeslist = ListNodes::new();
    let mut new_nodes = new_nodeslist.clone().nodes;
    add_other_nodes(lh::root_node(), selected_node.clone(), &mut new_nodes, &mut old_nodes);

    // borrar el nodo de los children de su padre
    let parent_indx = old_nodes.nodes[&selected_node.clone()].parent_indx.clone();
    let siblings = old_nodes.nodes[&parent_indx].children.clone();
    let mut new_siblings: Vec<String> = Vec::new();
    let length = siblings.len()-1;

    for i in 0..length {
        new_siblings.push(lh::get_next_sibling_indx(parent_indx.clone(), i as i32));
    }

    if let Some(x) = new_nodes.get_mut(&parent_indx.clone()) {
        x.children = new_siblings.clone();
    }

    // renombramos/recolocamos los hermanos
    for i in  (lh::get_child_indx(selected_node) + 1) as usize .. siblings.len() {
        //
        let branch_level_delta: i32;
        if i == 1 {
            branch_level_delta = -1;
        }
        else {
            branch_level_delta = 0;
        }
        let _cur_node = reallocate_sibling(&mut new_nodes,
                    &mut old_nodes,
                    lh::get_next_sibling_indx(parent_indx.clone(), i as i32),
                    lh::get_next_sibling_indx(parent_indx.clone(), i as i32-1),
                    branch_level_delta,
                    parent_indx.clone());
    }

    new_nodeslist.nodes = new_nodes;
    (new_nodeslist, parent_indx)
}



fn reallocate_sibling(new_nodes: &mut HashMap<String, Node>, 
        old_nodes: &mut ListNodes,
        old_indx: String, new_indx: String, 
        branch_level_delta: i32, parent_indx: String) {

    new_nodes.remove(&old_indx);

    let mut newnode = Node::new();

    newnode.branch_level = old_nodes.nodes[&old_indx.clone()].branch_level.clone();
    newnode.branch_level += branch_level_delta.clone();

    newnode.fen           = old_nodes.get_fen(old_indx.clone());
    newnode.san           = old_nodes.nodes[&old_indx.clone()].san.clone();
    newnode.parent_indx   = parent_indx.clone();
    newnode.nag           = old_nodes.nodes[&old_indx.clone()].nag.clone();
    newnode.start_comment = old_nodes.get_start_comment(old_indx.clone());
    newnode.comment       = old_nodes.nodes[&old_indx.clone()].comment.clone();

    for j in 0..old_nodes.get_children_length(old_indx.clone()) as usize {
        newnode.children.push(old_nodes.nodes[&old_indx].children[j].clone());
    }

    new_nodes.insert(new_indx.clone(), newnode);

    let children = old_nodes.nodes[&old_indx].children.clone();
    let mut new_children: Vec<String> = Vec::new();

    for i in 0..children.len() {
        let new_child_indx = lh::get_next_sibling_indx(new_indx.clone(), i as i32);
        new_children.push(new_child_indx.clone());
        reallocate_sibling(new_nodes, old_nodes, children[i].clone(), 
                new_child_indx, branch_level_delta, new_indx.clone());
    }

    if let Some(x) = new_nodes.get_mut(&new_indx.clone()) {
        x.children = new_children.clone();
    }
    
}



fn add_other_nodes(node_indx: String, 
    selected_node: String,
    new_nodes: &mut HashMap<String, Node>,
    old_nodes: &mut ListNodes) {

    let mut newnode = Node::new();
    if node_indx != selected_node {
        for j in 0..old_nodes.get_children_length(node_indx.clone()) as usize {
            newnode.children.push(old_nodes.nodes[&node_indx].children[j].clone());
        }
        newnode.branch_level = old_nodes.nodes[&node_indx.clone()].branch_level.clone();
        newnode.fen          = old_nodes.get_fen(node_indx.clone());
        newnode.san          = old_nodes.nodes[&node_indx.clone()].san.clone();
        newnode.parent_indx  = old_nodes.nodes[&node_indx.clone()].parent_indx.clone();
        newnode.nag          = old_nodes.nodes[&node_indx.clone()].nag.clone();
        newnode.start_comment= old_nodes.get_start_comment(node_indx.clone());
        newnode.comment      = old_nodes.nodes[&node_indx.clone()].comment.clone();

        new_nodes.insert(node_indx.clone(), newnode);

        let children = old_nodes.nodes[&node_indx].children.clone();
        for i in 0..children.len() {
            add_other_nodes(children[i].clone(), selected_node.clone(), new_nodes, old_nodes);
        }
    }

}



/*************************
 * PROMOTE VARIATION
 *************************/


pub fn promote_variation_handler () {
    let cur_node: String;
    let mut old_nodes: ListNodes;
    unsafe {
        cur_node = NODO_ACTIVO.clone().unwrap();
        old_nodes= NODOS.clone().unwrap();
    }

    if old_nodes.nodes[&cur_node.clone()].branch_level == 0 {
        // ya estamos en la linea principal
        return ;
        
    } else {    
        
        let (new_nodes, new_node_indx) = promote_variation(cur_node.clone(), &mut old_nodes);

        unsafe {
            if new_nodes.nodes.len() > 1 {
                NODOS = Some(new_nodes);
                NODO_ACTIVO = Some(new_node_indx);
            }
        }

        return;
    }
    
}


fn promote_variation (cur_node: String, old_nodes: &mut ListNodes) -> (ListNodes, String) {

    let selected_node = cur_node.clone();
    let branch_node_opt = lh::get_branch_node(selected_node.clone());
    let mut new_cur_indx: String = "".to_string(); 

    if branch_node_opt.is_none() {
        return (old_nodes.clone(), selected_node);
    }

    let branch_node = branch_node_opt.unwrap();
    let mut split_indx = selected_node.clone();

    while old_nodes.nodes[&split_indx].parent_indx != old_nodes.nodes[&branch_node].parent_indx {
        split_indx = old_nodes.nodes[&split_indx.clone()].parent_indx.clone();
    }

    let mut new_nodeslist: ListNodes = ListNodes::new();
    let mut new_nodes = new_nodeslist.nodes;
    
    add_other_nodes_promo(lh::root_node(), 
            branch_node.clone(),
            split_indx.clone(),
            &mut new_nodes,
            old_nodes);

    let mut branch_level_delta: i32 = 1;
    rename_node_promo(branch_node.clone(), 
            split_indx.clone(), 
            old_nodes.nodes[&branch_node.clone()].parent_indx.clone(), 
            branch_level_delta,
            &mut new_nodes,
            old_nodes,
            selected_node.clone(),
            &mut new_cur_indx);

    branch_level_delta = -1;
    
    rename_node_promo(split_indx.clone(), 
            branch_node.clone(), 
            old_nodes.nodes[&branch_node.clone()].parent_indx.clone(), 
            branch_level_delta,
            &mut new_nodes,
            old_nodes,
            selected_node,
            &mut new_cur_indx);

    new_nodeslist.nodes = new_nodes.clone();
    (new_nodeslist, new_cur_indx)
}



fn rename_node_promo(old_indx: String, 
        new_indx: String, 
        new_parent_indx:String, 
        branch_level_delta: i32,
        new_nodes: &mut HashMap<String, Node>,
        old_nodes: &mut ListNodes,
        selected_node: String,
        new_cur_indx: &mut String) {

    let mut newnode = Node::new();
    
    newnode.branch_level = old_nodes.nodes[&old_indx.clone()].branch_level.clone();
    newnode.branch_level += branch_level_delta.clone();

    newnode.fen           = old_nodes.get_fen(old_indx.clone());
    newnode.san           = old_nodes.nodes[&old_indx.clone()].san.clone();
    newnode.parent_indx   = new_parent_indx.clone();
    newnode.nag           = old_nodes.nodes[&old_indx.clone()].nag.clone();
    newnode.start_comment = old_nodes.get_start_comment(old_indx.clone());
    newnode.comment       = old_nodes.nodes[&old_indx.clone()].comment.clone();

    for j in 0..old_nodes.get_children_length(old_indx.clone()) as usize {
        newnode.children.push(old_nodes.nodes[&old_indx].children[j].clone());
    }

    new_nodes.insert(new_indx.clone(), newnode);

    if old_indx == selected_node {
        *new_cur_indx = new_indx.clone();
    }

    let mainline_continuation = lh::get_next_mainline_indx(old_indx.clone());
    let children = old_nodes.nodes[&old_indx.clone()].children.clone();
    let mut new_children: Vec<String> = Vec::new();

    for i in 0..children.len() {
        if children[i] == mainline_continuation.clone() {
            new_children.push(lh::get_next_mainline_indx(new_indx.clone()));
            rename_node_promo(mainline_continuation.clone(), 
                lh::get_next_mainline_indx(new_indx.clone()), 
                new_indx.clone(), 
                branch_level_delta,
                new_nodes,
                old_nodes,
                selected_node.clone(),
                new_cur_indx);
        }
        else {
            let new_child_indx = format!("{}{}{}", 
                new_indx.clone(), lh::get_child_indx(children[i].clone()), "n");
            new_children.push(new_child_indx.clone());
            rename_node_promo(children[i].clone(), 
                new_child_indx.clone(), 
                new_indx.clone(), 
                branch_level_delta,
                new_nodes,
                old_nodes,
                selected_node.clone(),
                new_cur_indx);
        }
    }

    if let Some(x) = new_nodes.get_mut(&new_indx.clone()) {
        x.children = new_children.clone();
    }
}



fn add_other_nodes_promo(node_indx: String, 
    branch_node: String,
    split_indx: String,
    new_nodes: &mut HashMap<String, Node>,
    old_nodes: &mut ListNodes) {

    let mut newnode = Node::new();
    if node_indx != branch_node && node_indx != split_indx {
        for j in 0..old_nodes.get_children_length(node_indx.clone()) as usize {
            newnode.children.push(old_nodes.nodes[&node_indx].children[j].clone());
        }
        newnode.branch_level = old_nodes.nodes[&node_indx.clone()].branch_level.clone();
        newnode.fen          = old_nodes.get_fen(node_indx.clone());
        newnode.san          = old_nodes.nodes[&node_indx.clone()].san.clone();
        newnode.parent_indx  = old_nodes.nodes[&node_indx.clone()].parent_indx.clone();
        newnode.nag          = old_nodes.nodes[&node_indx.clone()].nag.clone();
        newnode.start_comment= old_nodes.get_start_comment(node_indx.clone());
        newnode.comment      = old_nodes.nodes[&node_indx.clone()].comment.clone();

        new_nodes.insert(node_indx.clone(), newnode);

        let children = old_nodes.nodes[&node_indx].children.clone();
        for i in 0..children.len()  {
            add_other_nodes_promo(children[i].clone(),
                branch_node.clone(),
                split_indx.clone(),
                new_nodes,
                &mut old_nodes.clone());
        }
    }
}