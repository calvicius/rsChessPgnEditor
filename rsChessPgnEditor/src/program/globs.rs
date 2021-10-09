use std::collections::HashMap;


pub const DEFAULT_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const EMPTY_GAME: &str = r#"[Event "None"]
[Site " "]
[Date " "]
[Round " "]
[White " "]
[Black " "]
[Result " "]
[ECO " "]
[WhiteElo " "]
[BlackElo " "]
[PlyCount " "]
[EventDate " "]
"#;

#[derive(Clone)]
pub struct GameInfo {
    pub white : String, //pgnData['White']
	pub elow : String,	//pgnData['WhiteElo']
	pub black : String,	//pgnData['Black']
	pub elob : String,	//pgnData['BlackElo']
	pub res : String,	//settings.resToEnum(pgnData['Result'])
	pub event : String,	//pgnData['Event']
	pub site : String,	//pgnData['Site']
	pub round : String,	//pgnData['Round']
	pub date : String,	//pgnData['Date']
    pub fen : String,
    pub eco : String,
    pub tags: Vec<String>,  // other tags
}

impl GameInfo {
    pub fn new () -> Self {
        GameInfo {
            white : "".to_string(),
            elow : "".to_string(),
            black : "".to_string(),
            elob : "".to_string(),
            res : "".to_string(),
            event : "".to_string(),
            site : "".to_string(),
            round : "".to_string(),
            date : "".to_string(),
            fen: DEFAULT_POSITION.to_string(),
            eco: "000".to_string(),
            tags : Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct DbEntry {
    pub id :u64,			// solo habrá una partida_pgn el numero de registro de una futura BD
	pub game_info : GameInfo,
	pub pgn :String,
}

impl DbEntry {
    pub fn new() -> Self {
        DbEntry {
            id : 1,			// solo habrá una partida_pgn
            game_info : GameInfo::new(),
            pgn : String::from(EMPTY_GAME),
        }
    }
}

#[derive(Clone)]
pub struct PgnGame {
    pub white : String,
	pub elow : String,
	pub black : String,
	pub elob : String,
	pub res : String,
	pub event : String,
	pub site : String,
	pub round : String,
	pub date : String,
    pub fen : String,
    pub eco : String,
    pub moves: String,
    pub tags: Vec<String>,
}

impl PgnGame {
    pub fn new () -> Self {
        PgnGame {
            white : "".to_string(),
            elow : "".to_string(),
            black : "".to_string(),
            elob : "".to_string(),
            res : "".to_string(),
            event : "".to_string(),
            site : "".to_string(),
            round : "".to_string(),
            date : "".to_string(),
            fen : DEFAULT_POSITION.to_string(),
            eco: "000".to_string(),
            moves : "".to_string(),
            tags : Vec::new(),
        }
    }
}


// los nodos de la partida
#[derive(Clone, PartialEq, Eq)]
pub struct Node {
    pub children: Vec<String>,
    pub branch_level: i32,
    pub fen: String,
    pub san: String,
    pub parent_indx: String,
    pub nag: String,
    pub start_comment: String,
    pub comment: String,
}

impl Node {
    pub fn new() -> Self {
        Node {
            children: Vec::new(),
            branch_level: 0,
            fen: String::from(""),
            san: String::from(""),
            parent_indx: String::from(""),
            nag: String::from(""),
            start_comment: String::from(""),
            comment: String::from(""),
        }
    }
}

#[derive(Clone)]
pub struct ListNodes {
    pub nodes: HashMap<String, Node>,
}

impl ListNodes {
    pub fn new() -> Self {
        ListNodes {
            nodes: HashMap::new(),
        }
    }
    
    pub fn node_exists (&mut self, idx:String) -> bool {
        if let Some(_node) = self.nodes.get_mut(&idx) {
            return true;
        }
        else {
            return false;
        }
    }

    /* --------------------------- */

    pub fn init_node (&mut self, idx: String) {
        let nodo = Node::new();
        let _ = self.nodes.insert(idx, nodo);
    }

    pub fn init_branch_level(&mut self, idx: String, value: i32) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.branch_level = value;
        }
        else {
            println!("error en init_branch_level {} -- {}", idx, value);
        }
    }

    pub fn init_children(&mut self, idx: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.children = vec![];
        }
        else {
            println!("error en init_children {} --", idx);
        }
    }

    pub fn init_fen(&mut self, idx: String, fen: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.fen = fen;
        }
        else {
            println!("error en init_fen {} -- {}", idx, fen);
        }
    }

    /* -------------------------- */

    pub fn set_parent_indx(&mut self, idx: String, parent: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.parent_indx = parent;
        }
        else {
            println!("error en set_parent_indx {} -- {}", idx, parent);
        }
    }

    pub fn set_branch_level(&mut self, idx: String, level: i32) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.branch_level = level;
        }
        else {
            println!("error en set_branch_level {} -- {}", idx, level);
        }
    }

    pub fn set_nag(&mut self, idx: String, nag: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.nag = nag;
        }
        else {
            println!("error en set_nag {} -- {}", idx, nag);
        }
    }

    pub fn set_start_comment(&mut self, idx: String, comment: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.start_comment = comment;
        }
        else {
            println!("error en set_start_comment {} -- {}", idx, comment);
        }
    }

    pub fn set_comment(&mut self, idx: String, comment: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.comment = comment;
        }
        else {
            println!("error en set_comment {} -- {}", idx, comment);
        }
    }

    pub fn set_san(&mut self, idx: String, san: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.san = san;
        }
        else {
            println!("error en set_san {} -- {}", idx, san);
        }
    }

    pub fn set_fen(&mut self, idx: String, fen: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.fen = fen;
        }
        else {
            println!("error en set_fen {} -- {}", idx, fen);
        }
    }


    pub fn push_children(&mut self, idx: String, child_idx: String) {
        if let Some(node) = self.nodes.get_mut(&idx) {
            node.children.push(child_idx);
        }
        else {
            println!("error en push_children {} -- {}", idx, child_idx);
        }
    }

    /* -------------------------- */

    pub fn get_fen(&mut self, idx: String) -> String {
        let mut fen = "".to_string();
        if let Some(node) = self.nodes.get_mut(&idx) {
            fen = node.fen.clone();
        }
        fen
    }

    pub fn get_parent_indx(&mut self, idx: String, _parent_idx: String) -> String{
        let mut parent = "".to_string();
        if let Some(node) = self.nodes.get_mut(&idx) {
            parent = node.parent_indx.clone();
        }
        parent
    }

    pub fn get_children_length(&mut self, idx: String) -> i32 {
        let mut length: i32 = 0;
        if let Some(node) = self.nodes.get_mut(&idx) {
            length = node.children.len() as i32;
        }
        length
    }

    pub fn get_start_comment(&mut self, idx: String) -> String {
        let mut comment = "".to_string();
        if let Some(node) = self.nodes.get_mut(&idx) {
            comment = node.start_comment.clone();
        }
        comment
    }

}

#[derive(Clone)]
pub struct BranchIndexNodes {
    pub branch_index: HashMap<i32, String>,
}

impl BranchIndexNodes {
    pub fn new() -> Self {
        BranchIndexNodes {
            branch_index: HashMap::new(),
        }
    }

    pub fn set_indx_value(&mut self, idx: i32, value: String) {
        self.branch_index.insert(idx, value);
    }

}


