use regex::Regex;
use std::fs;

use gtk::*;

use super::wchess;
use super::globs;
use super::levels as lh;
use super::tags_textbuffer as ttbuf;
use super::utils;


const GTK_INDENT_LEVEL: i32 = 5;

pub const NAG_SYMBOLS: [&str; 251] = [
"", 
"!", "?", "\u{203C}", "\u{2047}", "\u{2049}", "\u{2048}", "\u{25A1}", "", "", "=",      // nag [1] to [10]
"", "", "\u{221E}", "\u{2A72}", "\u{2A71}", "\u{00B1}", "\u{2213}", "+-", "-+", "",     // nag [11] to [20]
"", "\u{2A00}", "\u{2A00}", "", "", "", "", "", "", "",                                 // nag [21] to [30]
"", "\u{27F3}", "\u{27F3}", "", "", "\u{2192}", "\u{2192}", "", "", "\u{2191}",         // nag [31] to [40]
"\u{2191}", "", "", "=/\u{221E}", "=/\u{221E}", "=/\u{221E}", "=/\u{221E}", "", "", "", // nag [41] to [50]
"", "", "", "", "", "", "", "", "", "",                                                 // nag [51] to [60]
"", "", "", "", "", "", "", "", "", "",                                                 // nag [61] to [70]
"", "", "", "", "", "", "", "", "", "",                                                 // nag [71] to [80]
"", "", "", "", "", "", "", "", "", "",                                                 // nag [81] to [90]
"", "", "", "", "", "", "", "", "", "",                                                 // nag [91] to [100]
"", "", "", "", "", "", "", "", "", "",                                                 // nag [101] to [110]
"", "", "", "", "", "", "", "", "", "",                                                 // nag [111] to [120]
"", "", "", "", "", "", "", "", "", "",                                                 // nag [121] to [130]
"", "\u{21C6}", "\u{21C6}", "", "", "\u{2A01}", "\u{2A01}", "\u{2A01}", "\u{2A01}", "\u{2206}",  // nag [131] to [140]
"\u{25BD}", "\u{2313}", "\u{2264}", "", "RR", "N", "", "", "", "",                  // Non standard nag [141] to [150]
"", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [151] to [160]
"", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [161] to [170]
"", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [171] to [180]
"", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [181] to [190]
"", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [191] to [200]
"", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [201] to [210]
"", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [211] to [220]
"", "", "", "", "", "", "", "", "", "",                                             // Non standard nag [221] to [230]
"", "", "", "", "", "", "", "\u{25CB}", "\u{21D4}", "\u{21D7}",                     // Non standard nag [231] to [240]
"", "\u{27EB}", "\u{27EA}", "\u{2715}", "\u{22A5}", "", "", "", "", "",             // Non standard nag [241] to [250]
];

struct ClosedParentesis {
    num: i32,
    //_is_last: bool,
}


pub fn read_games_from_file (filestr: String, games: &mut Vec<String>) {
    
    let data = fs::read_to_string(&filestr).expect("Unable to read file");
    let re = Regex::new(r"(\r\n\r\n|\r\r|\n\n)").unwrap();
    let intermedio: Vec<String> = re.split(&data).map(|s| s.to_string()).collect();
    if intermedio.len()%2 !=0 {
        utils::alerta("PGN file looks malformed.\nMaybe an extra newline symbol at the end.");
        //hack
        games.push(globs::EMPTY_GAME.to_string());
        games.push(globs::EMPTY_GAME.to_string());
        
    }
    else {
        let mut i = 0_usize;
        let mut final_vec: Vec<String> = Vec::new(); 
        // añadimos primero una partida vacia index[0]
        final_vec.push(globs::EMPTY_GAME.to_string());
        while i < intermedio.len() {
            let mut partida: String = "".to_string();
            partida = format!("{}{}\n\n", partida, intermedio[i]);
            i+=1;
            partida = format!("{}{}", partida, intermedio[i]);
            final_vec.push(partida);
            i+=1;
        }
        *games = final_vec.clone();
    }
}


pub fn parse_pgn_data(pgn: &mut String) -> globs::PgnGame {

    let mut jugadas: String = "".to_string();
    let mut pgn_data = globs::PgnGame::new();

    // reemplaza saltos de linea por espacios
    let game_vec: Vec<&str> = pgn.split("\n").collect();
    let re = Regex::new(r"\r?\n|\r").unwrap();
    for tag in game_vec {
        let tag_stripped = re.replace_all(tag, " ");

        let inner_re = Regex::new(r"^\s*\[[^%].*?\]");
        let match_header = inner_re.unwrap().captures(&tag_stripped);
        // the header
        if match_header.is_some() {
            let tag_vec: Vec<&str> = tag_stripped.split("\"").collect();
            // first the mandatory seven tags-roster
            if tag_stripped.contains("Site") {
                pgn_data.site = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("Event") {
                pgn_data.event = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("Date") {
                pgn_data.date = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("Round") {
                pgn_data.round = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("[White ") {
                pgn_data.white = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("[Black ") {
                pgn_data.black = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("Result") {
                pgn_data.res = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("WhiteElo") {
                pgn_data.elow = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("WhiteElo") {
                pgn_data.elob= tag_vec[1].to_string();
            }
            else if tag_stripped.contains("FEN") {
                pgn_data.fen = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("ECO") {
                pgn_data.eco = tag_vec[1].to_string();
            }
            else {
                pgn_data.tags.push(tag.to_string());
            }
        }
        // the moves
        else if tag_stripped != " ".to_string() && tag_stripped.len() > 0 {
            jugadas.push_str(&tag_stripped);
        }
    }

    pgn_data.moves = normalize_pgn_moves(&mut jugadas);
  
    pgn_data
}


fn normalize_pgn_moves(pgn: &mut String) -> String {

    let mut moves: String = "".to_string();
    // quitamos posibles "[...]" dentro de los movimientos
    let mut re = Regex::new(r"\[[^%].*?\]");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&pgn, "").to_string();
    }

    // Separar los NAGs por espacios
    re = Regex::new(r"\$");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " $").to_string();
    }

    // Separar comentarios con espacios
    re = Regex::new(r"\{");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " { ").to_string();
    }
    re = Regex::new(r"\}");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " } ").to_string();
    }

    // Separar variantes con espacios
    re = Regex::new(r"\(");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " ( ").to_string();
    }
    re = Regex::new(r"\)");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " ) ").to_string();
    }

    re = Regex::new(r"(?P<num>\d+)(\.+)");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, "$num. ").to_string();
    }

    re = Regex::new(r"\s\s+");
    if re.is_ok() {
        moves = re.unwrap().replace_all(&moves, " ").to_string();
    }

    // Borrar posibles espacios al inicio
    moves = moves.trim().to_string();

    moves
}


pub fn pgn_moves_to_nodes(pgn_moves: String, start_fen: String) -> globs::ListNodes {

    let mut game = wchess::new_board();

    let moves: Vec<&str> = pgn_moves.split(" ").collect();
    // inicializa
    let mut nodes = globs::ListNodes::new();
    let mut parent_indx = lh::root_node();
    let mut cur_indx = lh::get_next_mainline_indx(lh::root_node());
    let mut comment_indx = lh::get_next_mainline_indx(lh::root_node());
    let mut branch_level = 0;
    let mut branch_indx  = globs::BranchIndexNodes::new();
    
    // inicializa nodo raiz
    nodes.init_node (lh::root_node());

    //cambiamos el valor de un elemento del Node
    nodes.init_branch_level(lh::root_node(), 0);    // init to 0
    nodes.init_children(lh::root_node());   // init to vec![]
    
    if start_fen.len() > 1 {
        nodes.init_fen(lh::root_node(), start_fen);
    }
    else {
        nodes.init_fen(lh::root_node(), globs::DEFAULT_POSITION.to_string());
    }

    let _fen_valida = wchess::set_fen(&mut game, &nodes.nodes[&lh::root_node()].fen.clone() );

    let mut starts_with_comment = true;
    let mut i = 0;


    // iteramos sobre los datos
    let mut len = moves.len();
    // a little hack
    if len == 1 && moves[0] == "" { len = 0; }
    while i < len {
        
        if !nodes.node_exists(cur_indx.clone()) {
            nodes.init_node(cur_indx.clone());
            nodes.init_children(cur_indx.clone());
        }

        nodes.set_parent_indx(cur_indx.clone(), parent_indx.clone());
        nodes.set_branch_level(cur_indx.clone(), branch_level);

        // fin de partida
        if moves[i] == "*" ||
                moves[i] == "1-0" ||
                moves[i] == "0-1" ||
                moves[i] == "1/2-1/2" {
            starts_with_comment = false;
            i += 1;
            continue;
        }

        
        // NAGs
        let re = Regex::new(r"^\$");
        if re.is_ok() {
            if re.unwrap().is_match(moves[i]) {
                starts_with_comment = false;
                nodes.set_nag(comment_indx.clone(), moves[i].to_string());

                i += 1;
                continue;
            }
        }

        
        // numero de movim. o descripcion de movimiento
        let re = Regex::new(r"^[1-9]");
        let re1 = Regex::new(r"^\.+$");
        if re.is_ok() && re1.is_ok() {
            if re.unwrap().is_match(moves[i]) ||
                    re1.unwrap().is_match(moves[i]) {
                starts_with_comment = false;

                i += 1;
                continue;
            }
        }

        
        // comentarios
        if moves[i] == "{" {
            i += 1;
            let mut comment = "".to_string();
            while moves[i] != "}" {
                comment.push_str(" ");
                comment.push_str(moves[i]);
                i += 1;
            }

            if starts_with_comment {    // == true 
                nodes.set_start_comment(comment_indx.clone(), comment.trim().to_string());
            } else {
                nodes.set_comment(comment_indx.clone(), comment.trim().to_string());
            }

            starts_with_comment = false;
            i += 1;
            continue;
        }

        
        // SAN
        if "abcdefghRBNQKO0".contains(&moves[i].to_string()[0..1])  {
            starts_with_comment = false;
            
            nodes.push_children(parent_indx.clone(), cur_indx.clone());
            branch_indx.set_indx_value(branch_level+1, cur_indx.clone());
            nodes.set_san(cur_indx.clone(), moves[i].to_string());
            comment_indx = cur_indx.clone();

            // calcula la FEN
            let _fen_valida = wchess::set_fen(&mut game, &nodes.nodes[&parent_indx].fen );

            // TODO: contemplar si al enrocar se da jaque = O-O-O+
            // enroque. es un movimiento no convencional
            if moves[i] == "0-0" || moves[i] == "0-0+" ||
                    moves[i] == "O-O" || moves[i] == "O-O+" {
                let mov = wchess::move_san(&mut game, "O-O");
                //TODO : when error stop parsing smoothly
                if mov.movim.move_int == 0 {
                    panic!("error Ended game or invalid move");
                }
                
            } else if moves[i] == "0-0-0" || moves[i] == "0-0-0+" ||
                    moves[i] == "O-O-O" || moves[i] == "O-O-O+" {
                let mov = wchess::move_san(&mut game, "O-O-O");
                if mov.movim.move_int == 0 {
                    panic!("error Ended game or invalid move");
                }
            } else {
                let mov = wchess::move_san(&mut game, moves[i]);
                if mov.movim.move_int == 0 {
                    panic!("error Ended game or invalid move");
                }
            }
            //println!("occurrence white knights {} ", game.square.iter().filter(|&n| *n == 3).count());
            nodes.set_fen(cur_indx.clone(), wchess::get_fen(&mut game));

            // preparamos la iteracion siguiente
            parent_indx = cur_indx.clone();
            cur_indx = lh::get_next_mainline_indx(cur_indx.clone()); // superfluous nodes may be created, delete them later

            i += 1;
            continue;
        }

        
        // inicio de variante
        if moves[i] == "(" {
            // we can suposse that game may be long; so...
            // in this way we do not block the screen while processsing 
            // a very, very, long game (1.2 secs.)
            
            while gtk::events_pending() {
                gtk::main_iteration();
            }
            
            starts_with_comment = true;
            branch_level += 1;
            
            let indice = branch_indx.branch_index[&branch_level].to_string();
            parent_indx = nodes.get_parent_indx(indice, parent_indx);

            let num_siblings = nodes.get_children_length(parent_indx.clone());

            cur_indx = lh::get_next_sibling_indx(parent_indx.clone(), num_siblings);
            comment_indx = cur_indx.clone();

            i += 1;
            continue;
        }

        // fin de variante
        if moves[i] == ")" {
            starts_with_comment = true;
            
            parent_indx = branch_indx.branch_index[&branch_level].to_string();
            cur_indx = lh::get_next_mainline_indx(parent_indx.clone());
            comment_indx = cur_indx.clone();
            branch_level -= 1;

            i += 1;
            continue;
        }
    }

    // start_comment del primer nodo es el comentario del nodo raíz
    let first_node_indx = lh::get_next_mainline_indx(lh::root_node());
    let first_comment = nodes.get_start_comment(first_node_indx.clone());
    if first_comment.len() > 0 {
        nodes.set_comment(lh::root_node(), first_comment.clone());
        nodes.set_start_comment(first_node_indx.clone(), "".to_string());
    }

    // borrar nodos superfluos
    let keys: Vec<_> = nodes.nodes.keys().cloned().collect();
    for k in 0..keys.len() {
        if keys[k] == lh::root_node() {
            continue;
        }
        let n = nodes.nodes[&keys[k]].clone();
        if n.san.len() < 1 {
            nodes.nodes.remove(&keys[k]);
        }
    }

    return nodes;
}


pub fn traverse_nodes(view: &gtk::TextView, nodes: &mut globs::ListNodes) {
    let buff = view.get_buffer().expect("error");
    buff.set_text("");
    // comienza con el nodo raiz
    let seen_nodes: Vec<String> = vec![];
    //let mut vec_nodes: Vec<String> = Vec::new();
    parse_node(nodes, lh::root_node(), seen_nodes, view);
}


    
fn parse_node(nodes: &mut globs::ListNodes, 
        indx: String, 
        mut seen_nodes: Vec<String>,
        view: &gtk::TextView) {

    let index = seen_nodes.iter().position(|r| *r == indx.clone());
    
    if index.is_none() {
        nodes_to_buffer(view, nodes, indx.clone());
        seen_nodes.push(indx.clone());
    }

    let mainline_indx = lh::get_next_mainline_indx(indx.clone());
    let index = seen_nodes.iter().position(|r| *r == mainline_indx);

    if nodes.node_exists(mainline_indx.clone()) && index.is_none() {
        nodes_to_buffer(view, nodes, mainline_indx.clone());
        seen_nodes.push(mainline_indx.clone());
    }

    let mut i = 1;
    loop {
        let child_indx = lh::get_next_sibling_indx(indx.clone(), i);
        
        if nodes.node_exists(child_indx.clone()) {
            parse_node(nodes, child_indx.clone(), seen_nodes.clone(), view);
        } 
        else {
            break
        }

        i += 1;
    }

    if nodes.node_exists(mainline_indx.clone()) {
        parse_node(nodes, mainline_indx, seen_nodes.clone(), view);
    }

}


fn convert_nag2symbol(nag: String) -> String{
    let indx_nag = nag[1..].to_string();

    // the nag may contains more than 1 nag ($3$123)
    if indx_nag.contains("$") {
        let nags: Vec<&str> = indx_nag.split("$").collect();
        let mut final_nag: String = "".to_string();
        for nag in nags {
            let idx = nag.parse::<usize>().unwrap();
            final_nag = format!("{}{}", final_nag, NAG_SYMBOLS[idx]);
        }
        final_nag = format!(" {}", final_nag);
        return final_nag;
    }
    else {
        let idx = indx_nag.parse::<usize>().unwrap();
        let final_nag = format!(" {}", NAG_SYMBOLS[idx]);
        return final_nag;
    }
}



fn num_of_closed_parentheses_after_node (
            nodes: &mut globs::ListNodes, 
            node_indx: String, 
            indent_level: i32) -> ClosedParentesis {

    if nodes.nodes[&node_indx.clone()].children.len() > 0 {
        // node no es el nodo final
        return ClosedParentesis {num: 0};
    }

    
    let child_indx = lh::get_child_indx(node_indx.clone());
    let branch_indx = nodes.nodes[&node_indx].parent_indx.clone();

    
    if child_indx == 0 && nodes.nodes[&branch_indx].children.len() > 1 {
        // el nodo pertenece a la linea principal y tiene hermanos
        return ClosedParentesis {num: 0};
    }
    
    let num_closed = 1;
    let branch_level = nodes.nodes[&node_indx].branch_level.clone();
    let branch_san = nodes.nodes[&branch_indx].san.clone();
    
    
    // Comprueba si el nodo es el ultimo hijo de la ramificacion actual
    fn test_last_child (
            nodes: &mut globs::ListNodes,
            mut child_indx: i32,
            mut branch_indx: String,
            mut branch_san: String,
            mut num_closed: i32,
            mut branch_level: i32,
            indent_level: i32,
            ) -> ClosedParentesis {
        while nodes.nodes[&branch_indx.clone()].branch_level.clone() != branch_level-1 
                && branch_level != 0 {
            
            child_indx = lh::get_child_indx(branch_indx.clone());
            branch_indx = nodes.nodes[&branch_indx.clone()].parent_indx.clone();
            branch_san = nodes.nodes[&branch_indx.clone()].san.clone();
        }

       
        if lh::is_absolute_mainline(branch_indx.clone()) &&
            !nodes.node_exists(lh::get_next_mainline_indx(lh::get_next_mainline_indx(branch_indx.clone()))) {
            // no hay continuación despues de la linea principal
            return ClosedParentesis {num: num_closed};
        }

        let num_branches = nodes.nodes[&branch_indx.clone()].children.len();

        if nodes.node_exists(lh::get_next_mainline_indx(lh::get_next_mainline_indx(branch_indx.clone()))) {
            // la linea principal continúa despues del nodo rama
            return ClosedParentesis {num: num_closed};

            
        } 
        else {
            if num_branches as i32 == child_indx + 1 && branch_level > indent_level {
                // el nodo termina la ramificacion previa.
                num_closed += 1;
                branch_level -= 1;
                
                return test_last_child (
                    nodes,
                    child_indx,
                    branch_indx,
                    branch_san,
                    num_closed,
                    branch_level,
                    indent_level,
                    );
            } 
            else if branch_level == indent_level &&
                       num_branches as i32 == child_indx + 1 {

                return ClosedParentesis {num: num_closed};
                
            } else {
                return ClosedParentesis {num: num_closed};
            }
        }
    };

    return test_last_child (
        nodes,
        child_indx,
        branch_indx,
        branch_san,
        num_closed,
        branch_level,
        indent_level,
        );
}


pub fn nodes_to_buffer(view: &gtk::TextView, nodes: &mut globs::ListNodes, node_indx: String) {

    let buf = view.get_buffer().expect("error al obtener el buffer");

    let parent_indx = nodes.nodes[&node_indx].parent_indx.clone();
    let mut grand_parent = "".to_string();
    if nodes.node_exists(parent_indx.clone()) {
        grand_parent = nodes.get_parent_indx(parent_indx.clone(), parent_indx.clone());
    }

    let children = nodes.nodes[&node_indx.clone()].children.clone();
    let branch_level = nodes.nodes[&node_indx].branch_level;
    let fen = nodes.nodes[&node_indx.clone()].fen.clone();
    let fen_parts: Vec<&str> = fen.split(" ").collect();
    let mv_nr = fen_parts[5].to_string();

    let side_to_move: String = fen_parts[1].to_string();
    let mut display_mv_nr = false;
    let mut is_branch_point = false;
    if nodes.node_exists(parent_indx.clone()) &&
            nodes.nodes[&parent_indx].branch_level == branch_level &&
            nodes.nodes[&parent_indx].children.len() > 1 {
        is_branch_point = true;
    }
                        
    // nodo raiz
    if node_indx == lh::root_node() {
        let mut start_iter = buf.get_end_iter();
        buf.insert(&mut start_iter, "\u{200B}");
        let end_iter = buf.get_end_iter();
        
        ttbuf::create_tag_node(view, lh::root_node());
        buf.apply_tag_by_name(lh::root_node().as_str(), &start_iter, &end_iter);
        buf.apply_tag_by_name("branchlevel0", &start_iter, &end_iter);
    }
    
    // continuacion despues del comentario
    if nodes.node_exists(parent_indx.clone()) &&
            nodes.nodes[&parent_indx.clone()].comment.len() > 0 {
        display_mv_nr = true;
    }

    // comienzo de variante
    if nodes.node_exists(parent_indx.clone()) &&
            nodes.nodes[&parent_indx.clone()].branch_level != nodes.nodes[&node_indx.clone()].branch_level {

        let level = get_tag_branchlevel(branch_level.clone());
        if branch_level < GTK_INDENT_LEVEL {
            
            let mut start_iter = buf.get_end_iter();
            buf.insert(&mut start_iter, " \n");
            let mut start_iter = buf.get_end_iter();
            let marca1 = buf.create_mark(Some("varstart"), &start_iter, true)
                        .expect("error al crear marca");
            buf.insert(&mut start_iter, "");
            let end_iter = buf.get_end_iter();
            let marca_2 = buf.create_mark(Some("varend"), &end_iter, true)
                        .expect("error al crear marca");
            let start_iter = buf.get_iter_at_mark(&marca1);
            let end_iter = buf.get_iter_at_mark(&marca_2);

            buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);
        }
        else {
            
            let mut start_iter = buf.get_end_iter();
            buf.insert(&mut start_iter, "( ");
        }
        display_mv_nr = true;
    }

    // continuación de linea principal despues de variante
    if nodes.node_exists(parent_indx.clone()) && nodes.node_exists(grand_parent.clone()) &&
            nodes.nodes[&grand_parent.clone()].branch_level == nodes.nodes[&node_indx.clone()].branch_level &&
            nodes.nodes[&grand_parent.clone()].children.len() > 1 {

        display_mv_nr = true;
        if branch_level < GTK_INDENT_LEVEL -1 {
            let mut start_iter = buf.get_end_iter();
            buf.insert(&mut start_iter, " \n");
        }
        else {
            let mut start_iter = buf.get_end_iter();
            buf.insert(&mut start_iter, " ");
        }
    }

    if nodes.nodes[&node_indx.clone()].start_comment.len() > 0 {
        let level = get_tag_branchlevel(branch_level.clone());

        let mut start_iter = buf.get_end_iter();
        buf.insert(&mut start_iter, &nodes.nodes[&node_indx.clone()].start_comment);
        let end_iter = buf.get_end_iter();

        buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);

        display_mv_nr = true;
    }

    if node_indx != lh::root_node() && side_to_move == "b".to_string() {
        display_mv_nr = true;
    }

    
    
    // numero de jugada
    if display_mv_nr || node_indx == lh::get_next_mainline_indx(lh::root_node()) {
        if side_to_move == "b".to_string() {

            let nr = format!("{}.", mv_nr.parse::<i32>().unwrap()/2);
            let mut start_iter = buf.get_end_iter();
            let marca1 = buf.create_mark(Some("marca1"), &start_iter, true)
                        .expect("error al crear marca");
            buf.insert(&mut start_iter, &nr);
            let end_iter = buf.get_end_iter();
            let marca2 = buf.create_mark(Some("marca2"), &end_iter, true)
                        .expect("error al crear marca");

            let start_iter = buf.get_iter_at_mark(&marca1);
            let end_iter = buf.get_iter_at_mark(&marca2);

            let level = get_tag_branchlevel(branch_level.clone());
            buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);
            
        } 
        else {
            let tmp = mv_nr.parse::<i32>().unwrap();
            let nr = format!("{}...", (tmp-1)/2);

            let mut start_iter = buf.get_end_iter();
            let marca1 = buf.create_mark(Some("marca1"), &start_iter, true)
                        .expect("error al crear marca");
            buf.insert(&mut start_iter, &nr);
            let end_iter = buf.get_end_iter();
            let marca2 = buf.create_mark(Some("marca2"), &end_iter, true)
                        .expect("error al crear marca");

            let start_iter = buf.get_iter_at_mark(&marca1);
            let end_iter = buf.get_iter_at_mark(&marca2);

            let level = get_tag_branchlevel(branch_level.clone());
            buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);
        }
        
        let mut start_iter = buf.get_end_iter();
        let marca1 = buf.create_mark(Some("marca1"), &start_iter, true)
                        .expect("error al crear marca");
        buf.insert(&mut start_iter, " ");
        let end_iter = buf.get_end_iter();
        let marca2 = buf.create_mark(Some("marca2"), &end_iter, true)
                        .expect("error al crear marca");
        
        let start_iter = buf.get_iter_at_mark(&marca1);
        let end_iter = buf.get_iter_at_mark(&marca2);

        let level = get_tag_branchlevel(branch_level.clone());
        buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);
    }

    if nodes.nodes[&node_indx.clone()].san.len() > 0 {
        
        ttbuf::create_tag_node(view, node_indx.clone());
        let mut start_iter = buf.get_end_iter();
        let marca1 = buf.create_mark(Some("marca1"), &start_iter, true)
                        .expect("error al crear marca");

        buf.insert(&mut start_iter, &nodes.nodes[&node_indx.clone()].san);

        let end_iter = buf.get_end_iter();
        let marca2 = buf.create_mark(Some("marca2"), &end_iter, true)
                        .expect("error al crear marca");

        let start_iter = buf.get_iter_at_mark(&marca1);
        let end_iter = buf.get_iter_at_mark(&marca2);
        buf.apply_tag_by_name(node_indx.clone().as_str(), &start_iter, &end_iter);
        
        let level = get_tag_branchlevel(branch_level.clone());
        buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);

        let nag = nodes.nodes[&node_indx.clone()].nag.clone();
        if nag.len() > 0 {
            
            let mut start_iter = buf.get_end_iter();
            let marca1 = buf.create_mark(Some("marca1"), &start_iter, true)
                        .expect("error al crear marca");
            buf.insert(&mut start_iter, &convert_nag2symbol(nag));
            let end_iter = buf.get_end_iter();
            let marca2 = buf.create_mark(Some("marca2"), &end_iter, true)
                        .expect("error al crear marca");
            let start_iter = buf.get_iter_at_mark(&marca1);
            let end_iter = buf.get_iter_at_mark(&marca2);

            let level = get_tag_branchlevel(branch_level.clone());
            buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);
        }

        let mut spc = " \u{200B}";
        if children.len() == 0 && !is_branch_point {
            spc = "";
        }
        
        let mut start_iter = buf.get_end_iter();
        let marca1 = buf.create_mark(Some("marca1"), &start_iter, true)
                    .expect("error al crear marca");
        buf.insert(&mut start_iter, spc);
        let end_iter = buf.get_end_iter();
        let marca2 = buf.create_mark(Some("marca2"), &end_iter, true)
                    .expect("error al crear marca");
        
        let start_iter = buf.get_iter_at_mark(&marca1);
        let end_iter = buf.get_iter_at_mark(&marca2);
        let level = get_tag_branchlevel(branch_level.clone());
        buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);
    }

    if nodes.nodes[&node_indx.clone()].comment.len() > 0 {
        
        let mut start_iter = buf.get_end_iter();
        let marca1 = buf.create_mark(Some("marca1"), &start_iter, true)
                        .expect("error al crear marca");
        let comm = format!(" {} ", nodes.nodes[&node_indx.clone()].comment);
        buf.insert(&mut start_iter, &comm);
        let end_iter = buf.get_end_iter();
        let marca2 = buf.create_mark(Some("marca2"), &end_iter, true)
                        .expect("error al crear marca");
        
        let start_iter = buf.get_iter_at_mark(&marca1);
        let end_iter = buf.get_iter_at_mark(&marca2);

        let level = get_tag_branchlevel(branch_level.clone());
        buf.apply_tag_by_name(level.as_str(), &start_iter, &end_iter);
    }

    // fin de variante
    if children.len() == 0 {

        if branch_level >= GTK_INDENT_LEVEL {
            let num_closed = num_of_closed_parentheses_after_node(
                            nodes, node_indx.clone(), GTK_INDENT_LEVEL);
            
            for _ in 0..num_closed.num {
                let mut start_iter = buf.get_end_iter();
                buf.insert(&mut start_iter, " )");
            }
        }
    }
}


fn get_tag_branchlevel(depth: i32) -> String {
    
    if depth >= GTK_INDENT_LEVEL {
        return "branchlevel5".to_string();
    }
    else {
        return format!("branchlevel{}", depth);
    }
}


pub fn create_pgn (mut nodeslist: globs::ListNodes, game_info: globs::GameInfo) -> String {

	let nodes = nodeslist.nodes.clone();
    let mut pgn_stream = "".to_string();
    let start_fen = nodes[&lh::root_node()].fen.clone();
    let pgn: String;
    let res: String;

    match game_info.res.as_str() {
        "1" => res = "1-0".to_string(),
        "2" => res = "1/2-1/2".to_string(),
        "3" => res = "0-1".to_string(),
        "4" => res = "*".to_string(),
        _   => res = " ".to_string(),
    };

    pgn_stream = format!("{}[Event \"{}\"]\n", pgn_stream, game_info.event);
    pgn_stream = format!("{}[Site \"{}\"]\n", pgn_stream, game_info.site );
    pgn_stream = format!("{}[Date \"{}\"]\n", pgn_stream, game_info.date);
    pgn_stream = format!("{}[Round \"{}\"]\n", pgn_stream, game_info.round);
    pgn_stream = format!("{}[White \"{}\"]\n", pgn_stream, game_info.white);
    pgn_stream = format!("{}[Black \"{}\"]\n", pgn_stream, game_info.black);
    pgn_stream = format!("{}[Result \"{}\"]\n", pgn_stream, res);
    pgn_stream = format!("{}[WhiteElo \"{}\"]\n", pgn_stream, game_info.elow);
    pgn_stream = format!("{}[BlackElo \"{}\"]\n", pgn_stream, game_info.elob);
    pgn_stream = format!("{}[FEN \"{}\"]\n", pgn_stream, start_fen);
    pgn_stream = format!("{}[ECO \"{}\"]\n", pgn_stream, game_info.eco);
    pgn_stream = format!("{}\n", pgn_stream);

    let moves = traverse_nodes_plain(&mut nodeslist);
    pgn = format!("{} {}\n", moves, res);
    pgn_stream = format!("{}{}", pgn_stream, pgn);

    pgn_stream
}


fn traverse_nodes_plain (nodes: &mut globs::ListNodes) -> String {
    // comienza con el nodo raiz
    let seen_nodes: Vec<String> = vec![];
    let mut vec_nodes: Vec<String> = Vec::new();
    parse_node_plain(nodes, lh::root_node(), &mut vec_nodes, seen_nodes);
    let mut pgn_moves: String = "".to_string();
    
    for n in vec_nodes {
        let tmp = nodes_to_pgn(nodes, n.clone());
        pgn_moves = format!("{}{}", pgn_moves, tmp);
    }
    pgn_moves
}

fn parse_node_plain(nodes: &mut globs::ListNodes, 
        indx: String, 
        notation: &mut Vec<String>, 
        mut seen_nodes: Vec<String>) {

    let index = seen_nodes.iter().position(|r| *r == indx.clone()); // -> Option<>

    if index.is_none() {
        notation.push(indx.clone());
        seen_nodes.push(indx.clone());
    }

    let mainline_indx = lh::get_next_mainline_indx(indx.clone());
    let index = seen_nodes.iter().position(|r| *r == mainline_indx);

    if nodes.node_exists(mainline_indx.clone()) && index.is_none() {
        notation.push(mainline_indx.clone());
        seen_nodes.push(mainline_indx.clone());
    }

    let mut i = 1;
    loop {
        let child_indx = lh::get_next_sibling_indx(indx.clone(), i);
        
        if nodes.node_exists(child_indx.clone()) {
            parse_node_plain(nodes, child_indx.clone(), notation, seen_nodes.clone());
        } 
        else {
            break
        }

        i += 1;
    }

    if nodes.node_exists(mainline_indx.clone()) {
        parse_node_plain(nodes, mainline_indx, notation, seen_nodes.clone());
    }

}


fn nodes_to_pgn(nodeslist: &mut globs::ListNodes, node_indx: String) -> String {
    let mut nodes = nodeslist.nodes.clone();
    let parent_indx = nodes[&node_indx].parent_indx.clone();
    let grand_parent_opt: Option<String>;

    if let Some(_node) = nodes.get_mut(&parent_indx) {   // node exists?
        grand_parent_opt = Some(nodes[&parent_indx].parent_indx.clone());
    }
    else {
        grand_parent_opt = None;
    }

    let children = nodes[&node_indx].children.clone();
    let branch_level = nodes[&node_indx].branch_level;
    let fen = nodes[&node_indx].fen.clone();
    let fen_parts: Vec<&str> = fen.split(" ").collect();
    let mv_nr = fen_parts[5].to_string();
    let side_to_move: String = fen_parts[1].to_string();

    let mv_desc: String;
    let tmp = mv_nr.parse::<i32>().unwrap();
    if side_to_move == "b" {
        mv_desc = format!("{}{} ", tmp/2, ".");
    }
    else {
        mv_desc = format!("{}{} ", (tmp-1)/2, "...");
    }
    let mut display_mvnr = false;

    let mut rv = "".to_string();


    // continuation after comment
    let parent_exists: bool;
    if let Some(_node) = nodes.get_mut(&parent_indx) {   // node exists?
        parent_exists = true;
    }
    else {
        parent_exists = false;
    }
    
    if parent_exists && nodes[&parent_indx].comment.len() > 0 {

        display_mvnr = true;
    }
    
    // beginning of variation
    if parent_exists &&
        nodes[&parent_indx].branch_level != nodes[&node_indx].branch_level {
        
        rv += "(";
        display_mvnr = true;
    }

    // mainline continuation after variation
    if grand_parent_opt.is_some() {
        let grandpa = grand_parent_opt.clone().unwrap();
        if grandpa.len() > 0 {
            if parent_exists &&     //grand_parent_opt.is_some() && 
                nodes[&grandpa].branch_level == nodes[&node_indx].branch_level &&
                nodes[&grandpa].children.len() > 1 {

                display_mvnr = true;
            }
        }
    }

    if nodes[&node_indx].start_comment.len() > 0 {
        rv = format!("{}{}{}{}", rv, " {" , nodes[&node_indx].start_comment, "} ");
        display_mvnr = true;
    }

    if node_indx != lh::root_node() && side_to_move == "b" {
        display_mvnr = true;
    }

    
    // move number
    if display_mvnr == true || node_indx == lh::get_next_mainline_indx(lh::root_node()) {
        rv += mv_desc.as_str();
    }

    
    if nodes[&node_indx].san.len() > 0 {

        let spacing: &str;
        
        if children.len() == 0 {
            if lh::is_absolute_mainline(node_indx.clone()) {
                spacing = " ";
            }
            else { spacing = ""; }
        } 
        else {
            spacing = " ";
        }
        
        let nag = nodes[&node_indx].nag.clone();
        if nag.len() > 0 {
            rv = format!("{}{} {}{}", rv, nodes[&node_indx].san, nag, spacing);
        } else {
            rv = format!("{}{}{}", rv, nodes[&node_indx].san, spacing);
        }
        
    }

    // comments
    if nodes[&node_indx].comment.len() > 0 {
        rv = format!("{}{}{}{}", rv, " {", nodes[&node_indx].comment, "} ");
    }
    
    // end of variation
    if children.len() == 0 {
        
        if branch_level != 0 {

            let num_closed = num_of_closed_parentheses_after_node(nodeslist, node_indx, 0);
            for _ in 0..num_closed.num {
                rv += ")";
            }
            rv += " ";

        } else {
            // end of game
        }
    }

    return rv;
}


// added for searching positions

pub fn read_games_string (filestr: String, games: &mut Vec<String>) {
    
    let data = filestr;
    let re = Regex::new(r"(\r\n\r\n|\r\r|\n\n)").unwrap();
    let intermedio: Vec<String> = re.split(&data).map(|s| s.to_string()).collect();
    let mut length = intermedio.len();
    if intermedio.len()%2 !=0 {
        // the end of file contains two \n
        length -=1;
    }
    else {
        let mut i = 0_usize;
        let mut final_vec: Vec<String> = Vec::new(); 
        // añadimos primero una partida vacia index[0]
        //final_vec.push(globs::EMPTY_GAME.to_string());
        while i < length {
            let mut partida: String = "".to_string();
            partida = format!("{}{}\n\n", partida, intermedio[i]);
            i+=1;
            partida = format!("{}{}", partida, intermedio[i]);
            final_vec.push(partida);
            i+=1;
        }
        *games = final_vec.clone();
    }
}


pub fn parse_pgn_moves(pgn: &mut String) -> globs::PgnGame {

    let mut jugadas: String = "".to_string();
    let mut pgn_data = globs::PgnGame::new();

    // reemplaza saltos de linea por espacios
    let game_vec: Vec<&str> = pgn.split("\n").collect();
    let re = Regex::new(r"\r?\n|\r").unwrap();
    for tag in game_vec {
        let tag_stripped = re.replace_all(tag, " ");

        let inner_re = Regex::new(r"^\s*\[[^%].*?\]");
        let match_header = inner_re.unwrap().captures(&tag_stripped);
        // the header
        if match_header.is_some() {
            
            let tag_vec: Vec<&str> = tag_stripped.split("\"").collect();
            // first the mandatory seven tags-roster
            if tag_stripped.contains("Site") {
                pgn_data.site = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("Event") {
                pgn_data.event = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("Date") {
                pgn_data.date = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("Round") {
                pgn_data.round = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("[White ") {
                pgn_data.white = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("[Black ") {
                pgn_data.black = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("Result") {
                pgn_data.res = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("WhiteElo") {
                pgn_data.elow = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("WhiteElo") {
                pgn_data.elob= tag_vec[1].to_string();
            }
            else if tag_stripped.contains("FEN") {
                pgn_data.fen = tag_vec[1].to_string();
            }
            else if tag_stripped.contains("ECO") {
                pgn_data.eco = tag_vec[1].to_string();
            }
            else {
                pgn_data.tags.push(tag.to_string());
            }
            
        }
        // the moves
        else if tag_stripped != " ".to_string() && tag_stripped.len() > 0 {
            jugadas.push_str(&tag_stripped);
        }
    }

    pgn_data.moves = normalize_pgn_moves(&mut jugadas);
  
    pgn_data
}


pub fn search_pattern_exact_fen (pgn_moves: String, start_fen: String,
        game_num: usize, search_array: [i32; 64]) -> i32 {


    let moves: Vec<&str> = pgn_moves.split(" ").collect();
    let fen: String;

    if start_fen.len() > 1 {
        fen = start_fen;
        }
        else {
        fen = globs::DEFAULT_POSITION.to_string();
    }

    /*
    code of pieces in wchess are:

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
    */

    let w_p = search_array.iter().filter(|&n| *n == 1).count();
    let w_n = search_array.iter().filter(|&n| *n == 3).count();
    let w_b = search_array.iter().filter(|&n| *n == 5).count();
    let w_r = search_array.iter().filter(|&n| *n == 6).count();
    let w_q = search_array.iter().filter(|&n| *n == 7).count();

    let b_p = search_array.iter().filter(|&n| *n == 9).count();
    let b_n = search_array.iter().filter(|&n| *n == 11).count();
    let b_b = search_array.iter().filter(|&n| *n == 13).count();
    let b_r = search_array.iter().filter(|&n| *n == 14).count();
    let b_q = search_array.iter().filter(|&n| *n == 15).count();

    let mut game = wchess::new_board();
    let _fen_valida = wchess::set_fen( &mut game, &fen );

    let mut i = 0;
    let mut num_parentesis = 0;
    // iteramos sobre los datos
    let mut len = moves.len();
    // a little hack
    if len == 1 && moves[0] == "" { len = 0; }

    while i < len {

    // fin de partida
    if moves[i] == "*" ||
            moves[i] == "1-0" ||
            moves[i] == "0-1" ||
            moves[i] == "1/2-1/2" {
        i += 1;
        continue;
    }

    // NAGs
    let re = Regex::new(r"^\$");
    if re.is_ok() {
        if re.unwrap().is_match(moves[i]) {
            i += 1;
            continue;
        }
    }

    // numero de movim. o descripcion de movimiento
    let re = Regex::new(r"^[1-9]");
    let re1 = Regex::new(r"^\.+$");
    if re.is_ok() && re1.is_ok() {
        if re.unwrap().is_match(moves[i]) ||
                re1.unwrap().is_match(moves[i]) {
            
            i += 1;
            continue;
        }
    }

    // comentarios
    if moves[i] == "{" {
        i += 1;
        //let mut comment = "".to_string();
        while moves[i] != "}" {
            //comment.push_str(" ");
            //comment.push_str(moves[i]);
            i += 1;
        }

        i += 1;
        continue;
    }

    // inicio de variante
    // we dont match variations
    if moves[i] == "(" {
        num_parentesis += 1;
        while num_parentesis != 0 {
            
            i += 1;
            if moves[i] == ")" {
                num_parentesis -= 1;
            }
            if moves[i] == "(" {
                num_parentesis += 1;
            }
        }
        i+=1;
        continue;
    }

    // SAN
    if "abcdefghRBNQKO0".contains(&moves[i].to_string()[0..1])  {

        // TODO: contemplar si al enrocar se da jaque = O-O-O+
        // enroque. es un movimiento no convencional
        if moves[i] == "0-0" || moves[i] == "0-0+" ||
                moves[i] == "O-O" || moves[i] == "O-O+" {
            let mov = wchess::move_san(&mut game, "O-O");
            if mov.movim.move_int == 0 {
                return -1;
            }
            
        } else if moves[i] == "0-0-0" || moves[i] == "0-0-0+" ||
                moves[i] == "O-O-O" || moves[i] == "O-O-O+" {
            let mov = wchess::move_san(&mut game, "O-O-O");
            if mov.movim.move_int == 0 {
                return -1;
            }
        } else {
            let mov = wchess::move_san(&mut game, moves[i]);
            if mov.movim.move_int == 0 {
                return -1;
            }
        }
        let game_array = wchess::get_board_array(&mut game);
        if search_array == game_array {
            // found same position
            return game_num as i32;
        }

        // Si el material de la partida, en este momento, es inferior; nunca
        // podrá coincidir con nuestra condición de búsqueda original
        // El tiempo se reduce en un 70% aprox para 4800 partidas
        let gw_p = game_array.iter().filter(|&n| *n == 1).count();
        let gw_n = game_array.iter().filter(|&n| *n == 3).count();
        let gw_b = game_array.iter().filter(|&n| *n == 5).count();
        let gw_r = game_array.iter().filter(|&n| *n == 6).count();
        let gw_q = game_array.iter().filter(|&n| *n == 7).count();

        let gb_p = game_array.iter().filter(|&n| *n == 9).count();
        let gb_n = game_array.iter().filter(|&n| *n == 11).count();
        let gb_b = game_array.iter().filter(|&n| *n == 13).count();
        let gb_r = game_array.iter().filter(|&n| *n == 14).count();
        let gb_q = game_array.iter().filter(|&n| *n == 15).count();

        if gw_p < w_p || gw_n < w_n || gw_b < w_b || gw_r < w_r || gw_q < w_q ||
        gb_p < b_p || gb_n < b_n || gb_b < b_b || gb_r < b_r || gb_q < b_q {

            return -1;
        }


        i += 1;
        continue;
    }

    }

    -1  // negative is match position not found
}


pub fn search_pattern_position_fen (pgn_moves: String, start_fen: String,
        game_num: usize, search_array: [i32; 64]) -> i32 {


    let moves: Vec<&str> = pgn_moves.split(" ").collect();
    let fen: String;

    if start_fen.len() > 1 {
        fen = start_fen;
    }
    else {
        fen = globs::DEFAULT_POSITION.to_string();
    }

    /*
    code of pieces in wchess are:

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
    */

    let w_p = search_array.iter().filter(|&n| *n == 1).count();
    let w_n = search_array.iter().filter(|&n| *n == 3).count();
    let w_b = search_array.iter().filter(|&n| *n == 5).count();
    let w_r = search_array.iter().filter(|&n| *n == 6).count();
    let w_q = search_array.iter().filter(|&n| *n == 7).count();

    let b_p = search_array.iter().filter(|&n| *n == 9).count();
    let b_n = search_array.iter().filter(|&n| *n == 11).count();
    let b_b = search_array.iter().filter(|&n| *n == 13).count();
    let b_r = search_array.iter().filter(|&n| *n == 14).count();
    let b_q = search_array.iter().filter(|&n| *n == 15).count();

    // get where pieces are placed
    struct PiecesIndex {
        idx: usize,
        piece: i32,
    }

    let mut vec_pieces: Vec<PiecesIndex> = Vec::new();

    for i in 0..search_array.len() {
        if search_array[i] != 0 {
            let piece = PiecesIndex { idx: i, piece: search_array[i]};
            vec_pieces.push(piece);
        }
    }

    let mut game = wchess::new_board();
    let _fen_valida = wchess::set_fen( &mut game, &fen );

    let mut i = 0;
    let mut num_parentesis = 0;
    // iteramos sobre los datos
    let mut len = moves.len();
    // a little hack
    if len == 1 && moves[0] == "" { len = 0; }

    while i < len {

    // fin de partida
    if moves[i] == "*" ||
        moves[i] == "1-0" ||
        moves[i] == "0-1" ||
        moves[i] == "1/2-1/2" {
    i += 1;
    continue;
    }

    // NAGs
    let re = Regex::new(r"^\$");
    if re.is_ok() {
        if re.unwrap().is_match(moves[i]) {
            i += 1;
            continue;
        }
    }

    // numero de movim. o descripcion de movimiento
    let re = Regex::new(r"^[1-9]");
    let re1 = Regex::new(r"^\.+$");
    if re.is_ok() && re1.is_ok() {
        if re.unwrap().is_match(moves[i]) ||
                re1.unwrap().is_match(moves[i]) {
            
            i += 1;
            continue;
        }
    }

    // comentarios
    if moves[i] == "{" {
        i += 1;
        while moves[i] != "}" {
            i += 1;
        }

        i += 1;
        continue;
    }

    // inicio de variante
    // we dont match variations
    if moves[i] == "(" {
        num_parentesis += 1;
        while num_parentesis != 0 {
            
            i += 1;
            if moves[i] == ")" {
                num_parentesis -= 1;
            }
            if moves[i] == "(" {
                num_parentesis += 1;
            }
        }
        i+=1;
        continue;
    }

    // SAN
    if "abcdefghRBNQKO0".contains(&moves[i].to_string()[0..1])  {

        if moves[i] == "0-0" {
            let mov = wchess::move_san(&mut game, "O-O");
            if mov.movim.move_int == 0 {
                return -1;
            }
            
        } else if moves[i] == "0-0-0" {
            let mov = wchess::move_san(&mut game, "O-O-O");
            if mov.movim.move_int == 0 {
                return -1;
            }
        } else {
            let mov = wchess::move_san(&mut game, moves[i]);
            if mov.movim.move_int == 0 {
                return -1;
            }
        }
        let game_array = wchess::get_board_array(&mut game);
        //if search_array == game_array {
        //    // found same position
        //    return game_num as i32;
        //}
        let mut found: bool = false;
        for i in 0..vec_pieces.len() {
            let idx = vec_pieces[i].idx;
            if vec_pieces[i].piece == game_array[idx] { found = true; }
            else {
                found = false;
                break;
            }
        }
        if found { return game_num as i32; }

        // Si el material de la partida, en este momento, es inferior; nunca
        // podrá coincidir con nuestra condición de búsqueda original
        // El tiempo se reduce en un 70% aprox para 4800 partidas
        let gw_p = game_array.iter().filter(|&n| *n == 1).count();
        let gw_n = game_array.iter().filter(|&n| *n == 3).count();
        let gw_b = game_array.iter().filter(|&n| *n == 5).count();
        let gw_r = game_array.iter().filter(|&n| *n == 6).count();
        let gw_q = game_array.iter().filter(|&n| *n == 7).count();

        let gb_p = game_array.iter().filter(|&n| *n == 9).count();
        let gb_n = game_array.iter().filter(|&n| *n == 11).count();
        let gb_b = game_array.iter().filter(|&n| *n == 13).count();
        let gb_r = game_array.iter().filter(|&n| *n == 14).count();
        let gb_q = game_array.iter().filter(|&n| *n == 15).count();

        if gw_p < w_p || gw_n < w_n || gw_b < w_b || gw_r < w_r || gw_q < w_q ||
        gb_p < b_p || gb_n < b_n || gb_b < b_b || gb_r < b_r || gb_q < b_q {

            return -1;
        }


        i += 1;
        continue;
        }

    }

    -1  // negative is match position not found
}


pub fn search_pattern_material (pgn_moves: String, start_fen: String,
        game_num: usize, search_string: String) -> i32 {

    let moves: Vec<&str> = pgn_moves.split(" ").collect();
    let fen: String;

    if start_fen.len() > 1 {
        fen = start_fen;
    }
    else {
        fen = globs::DEFAULT_POSITION.to_string();
    }

    /*
    let mut wk: usize = 0;
    let mut bk: usize = 0;
    */
    let mut wp: usize = 0;
    let mut bp: usize = 0;
    let mut wn: usize = 0;
    let mut bn: usize = 0;
    let mut wb: usize = 0;
    let mut bb: usize = 0;
    let mut wr: usize = 0;
    let mut br: usize = 0;
    let mut wq: usize = 0;
    let mut bq: usize = 0;

    let vec_str: Vec<&str> = search_string.split("+").collect();
    for m in vec_str {
        /*
        if m.starts_with("K") {
            wk = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("k") {
            bk = m[1..].parse::<usize>().unwrap();
        }
        */
        if m.starts_with("P") {
            wp = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("p") {
            bp = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("N") {
            wn = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("N") {
            wn = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("n") {
            bn = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("B") {
            wb = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("b") {
            bb = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("R") {
            wr = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("r") {
            br = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("Q") {
            wq = m[1..].parse::<usize>().unwrap();
        }
        if m.starts_with("q") {
            bq = m[1..].parse::<usize>().unwrap();
        }
    }

    let mut game = wchess::new_board();
    let _fen_valida = wchess::set_fen( &mut game, &fen );

    let mut i = 0;
    let mut num_parentesis = 0;
    // iteramos sobre los datos
    let mut len = moves.len();
    // a little hack
    if len == 1 && moves[0] == "" { len = 0; }

    //only search when j > 60 (30 full moves)
    let mut j: i32 = 0;

    while i < len {

        // fin de partida
        if moves[i] == "*" ||
                moves[i] == "1-0" ||
                moves[i] == "0-1" ||
                moves[i] == "1/2-1/2" {
            i += 1;
            continue;
        }

        // NAGs
        let re = Regex::new(r"^\$");
        if re.is_ok() {
            if re.unwrap().is_match(moves[i]) {
                i += 1;
                continue;
            }
        }

        // numero de movim. o descripcion de movimiento
        let re = Regex::new(r"^[1-9]");
        let re1 = Regex::new(r"^\.+$");
        if re.is_ok() && re1.is_ok() {
            if re.unwrap().is_match(moves[i]) ||
                    re1.unwrap().is_match(moves[i]) {
                
                i += 1;
                continue;
            }
        }

        // comentarios
        if moves[i] == "{" {
            i += 1;
            //let mut comment = "".to_string();
            while moves[i] != "}" {
                //comment.push_str(" ");
                //comment.push_str(moves[i]);
                i += 1;
            }

            i += 1;
            continue;
        }

        // inicio de variante
        // we dont match variations
        if moves[i] == "(" {
            num_parentesis += 1;
            while num_parentesis != 0 {
                
                i += 1;
                if moves[i] == ")" {
                    num_parentesis -= 1;
                }
                if moves[i] == "(" {
                    num_parentesis += 1;
                }
            }
            i+=1;
            continue;
        }

        // SAN
        if "abcdefghRBNQKO0".contains(&moves[i].to_string()[0..1])  {

            // TODO: contemplar si al enrocar se da jaque = O-O-O+
            // enroque. es un movimiento no convencional
            if moves[i] == "0-0" || moves[i] == "0-0+" ||
                    moves[i] == "O-O" || moves[i] == "O-O+" {

                let mov = wchess::move_san(&mut game, "O-O");
                if mov.movim.move_int == 0 {
                    return -1;
                }
            } else if moves[i] == "0-0-0" || moves[i] == "0-0-0+" ||
                    moves[i] == "O-O-O" || moves[i] == "O-O-O+" {

                let mov = wchess::move_san(&mut game, "O-O-O");
                if mov.movim.move_int == 0 {
                    return -1;
                }
            } else {

                let mov = wchess::move_san(&mut game, moves[i]);
                if mov.movim.move_int == 0 {
                    return -1;
                }
            }
            // only search from move 30 onwards 
            j += 1;
            if j <= 60 { 
                i += 1;
                continue; 
            }

            let game_array = wchess::get_board_array(&mut game);

            let gw_p = game_array.iter().filter(|&n| *n == 1).count();
            let gw_n = game_array.iter().filter(|&n| *n == 3).count();
            let gw_b = game_array.iter().filter(|&n| *n == 5).count();
            let gw_r = game_array.iter().filter(|&n| *n == 6).count();
            let gw_q = game_array.iter().filter(|&n| *n == 7).count();

            let gb_p = game_array.iter().filter(|&n| *n == 9).count();
            let gb_n = game_array.iter().filter(|&n| *n == 11).count();
            let gb_b = game_array.iter().filter(|&n| *n == 13).count();
            let gb_r = game_array.iter().filter(|&n| *n == 14).count();
            let gb_q = game_array.iter().filter(|&n| *n == 15).count();

            if gw_p == wp && gw_n == wn && gw_b == wb && gw_r == wr && gw_q == wq &&
                    gb_p == bp && gb_n == bn && gb_b == bb && gb_r == br && gb_q == bq {

                return game_num as i32;
            }

            // Si el material de la partida, en este momento, es inferior; nunca
            // podrá coincidir con nuestra condición de búsqueda original
            // El tiempo se reduce en un 70% aprox para 4800 partidas
            if gw_p < wp || gw_n < wn || gw_b < wb || gw_r < wr || gw_q < wq ||
                    gb_p < bp || gb_n < bn || gb_b < bb || gb_r < br || gb_q < bq {

                return -1;
            }


            i += 1;
            continue;

        }
    }

    -1
}