//use std::time::{Duration, Instant};
use std::time;
use std::thread;
use std::process;

use gtk::*;
//use glib::Cast;

use super::globs;
use super::utils;
//use super::pgn as ph;
use super::xboard::{NODO_ACTIVO, NODOS,
    strip_variation_handler, promote_variation_handler};
use super::tags_textbuffer as ttbuf;
use super::board_gui::draw_notation;
use super::nagcomments;
use super::toolbarengine::Engine;
use super::pgn as ph;
use super::treegames;
use super::fensearch;
use super::matchedgames;
use super::xboard;
use super::bookreader;


const BTNS_WHITE: &str = "./icons/btnstoolbar_white/";

pub struct BarGame {
    pub tb : gtk::Toolbar,
    btn_open_pgn: gtk::ToolButton,
    btn_save_pgn: gtk::ToolButton,
    btn_cut:      gtk::ToolButton,
    btn_del:      gtk::ToolButton,
    btn_prom:     gtk::ToolButton,
    btn_close:    gtk::ToolButton,
    btn_nag:      gtk::ToolButton,
    btn_open_bin: gtk::ToolButton,
    btn_search:   gtk::ToolButton,
}

pub const CSSBAR: &'static str = "
#bargame { 
    background-color: rgba(50,50,50, 0.92);
    padding: 0.20625rem 0.20625rem 0.20625rem 0.20625rem; }/*3 3 3 3*/

";


impl BarGame {
    pub fn new() -> Self {
        // Cargamos el CSS
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSSBAR.as_bytes())
            .expect("Failed to load CSS");
        // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
        // que agregamos se puedan aplicar a nuestra ventana.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        let toolbar = gtk::Toolbar::new();
        gtk::WidgetExt::set_widget_name(&toolbar, "bargame");
        let separador = gtk::SeparatorToolItem::new();
        toolbar.insert(&separador, -1);

        let size_btn: i32 = 20;

        // button close window
        let btn_path = format!("{}{}", BTNS_WHITE, "window-close.png");
        let btn_str = btn_path.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn, size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_close = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_close, "closewin");
        toolbar.insert(&btn_close, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_close, Some("Close window"));

        let separador = gtk::SeparatorToolItem::new();
        toolbar.insert(&separador, -1);

        // button open pgn file
        let btn_path = format!("{}{}", BTNS_WHITE, "folder-open.png");
        let btn_str = btn_path.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn, size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_open_pgn = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_open_pgn, "openpgn");
        toolbar.insert(&btn_open_pgn, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_open_pgn, Some("Open PGN file"));

        // button save as pgn
        let btn_path = format!("{}{}", BTNS_WHITE, "save.png");
        let btn_str = btn_path.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn, size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_save_pgn = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_save_pgn, "savepgn");
        toolbar.insert(&btn_save_pgn, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_save_pgn, Some("Save game as PGN"));

        let separador = gtk::SeparatorToolItem::new();
        toolbar.insert(&separador, -1);

        // button cut variation from some active move
        let btn_path = format!("{}{}", BTNS_WHITE, "cut.png");
        let btn_str = btn_path.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_cut = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_cut, "cutgame");
        toolbar.insert(&btn_cut, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_cut, Some("Cut variation from active move"));

        // button delete variation where is active move
        let btn_path = format!("{}{}", BTNS_WHITE, "trash.png");
        let btn_str = btn_path.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_del = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_del, "delgame");
        toolbar.insert(&btn_del, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_del, Some("Delete variation where is active move"));

        // button promote variation
        let btn_path = format!("{}{}", BTNS_WHITE, "angle-double-up.png");
        let btn_str = btn_path.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_prom = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_prom, "promvar");
        toolbar.insert(&btn_prom, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_prom, Some("Promote variation"));

        let separador = gtk::SeparatorToolItem::new();
        toolbar.insert(&separador, -1);

        // button edit NAGs and Comments
        let btn_path = format!("{}{}", BTNS_WHITE, "edit.png");
        let btn_str = btn_path.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_nag = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_nag, "editnag");
        toolbar.insert(&btn_nag, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_nag, Some("Edit NAGs and Comments"));

        // button open book.bin file
        let btn_path = format!("{}{}", BTNS_WHITE, "folder-open.png");
        let btn_str = btn_path.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn, size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_open_bin = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_open_bin, "openpgn");
        toolbar.insert(&btn_open_bin, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_open_bin, Some("Open book.bin opening filee"));

        // button search position
        let btn_search = format!("{}{}", BTNS_WHITE, "search.png");
        let btn_str = btn_search.as_str();
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn, size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        let btn_search = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_search, "searchpos");
        for _i in 0..50 {
            let separador = gtk::SeparatorToolItem::new();
            toolbar.insert(&separador, -1);
        }
        toolbar.insert(&btn_search, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_search, Some("Search Positions"));
        

        BarGame {
            tb: toolbar,
            btn_open_pgn: btn_open_pgn,
            btn_save_pgn: btn_save_pgn,
            btn_cut:      btn_cut,
            btn_del:      btn_del,
            btn_prom:     btn_prom,
            btn_close:    btn_close,
            btn_nag:      btn_nag,
            btn_open_bin: btn_open_bin,
            btn_search:   btn_search,
        }
    }


    pub fn btns_bargame_closures(self, weak_area: glib::object::WeakRef<gtk::DrawingArea>, 
                weak_san_viewer: glib::object::WeakRef<gtk::TextView>,
                header: super::header::Form,
                tree_games: treegames::TreeGames,
                engine: &Engine) {

        unsafe {
            if NODO_ACTIVO.is_none() {
                NODO_ACTIVO = Some("1z".to_string());
            }
        }

        // recover the weakref
        let txtview = match weak_san_viewer.upgrade() {
            Some(txtview) => txtview,
            None => return,
        };
        // recover the weakref
        let board_area = match weak_area.upgrade() {
            Some(board_area) => board_area,
            None => return,
        };

        let eng1 = engine.clone();
        self.btn_close.connect_clicked ( move |_| {
            //eng1.set_stop();
            //eng1.set_quit();
            //let one_secs = time::Duration::from_millis(1000);
            //thread::sleep(one_secs);
            //eng1.motor.force_exit();
            if gtk::events_pending() {
                eng1.set_quit();
                eng1.motor.force_exit();
                //utils::alerta("Hay eventos pendientes.\n¿Motor activo?.\n¿Fichero PGN todavia procesando?");
                let one_secs = time::Duration::from_millis(100);
                thread::sleep(one_secs);
                process::exit(0);
            }
            else {
                eng1.set_quit();
                eng1.motor.force_exit();
                main_quit();
            }
        });

        // open book.bin 
        self.btn_open_bin.connect_clicked ( move |_| {
            let fen: String;
            unsafe {
                match &xboard::FEN_ACTUAL {
                    Some(v) => fen = v.clone(),
                    None => fen = "error".to_string(),
                }
            }
            if fen != "error" {
                let path_book = "./books/book.bin";
                bookreader::init_search( path_book, &fen);
            }
        });


        let area_clon5 = board_area.clone();
        let tree_games_clon = tree_games.clone();
        self.btn_open_pgn.connect_clicked ( move |_| {
            unsafe {
                if treegames::PROCESSING_FILE {
                    utils::alerta("At this moment is being processed some large pgn file");
                    return;
                } 
            }
            //let header_clon5 = header.clone();
            //let view_clon5 = weak_san_viewer.clone();
            let clon1 = tree_games_clon.clone();
            
            let mut vec_games_str: Vec<String> = Vec::new();
            let file_opt = utils::pgn_select();
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
            
            match file_opt {
                Some(file) => {
                    ph::read_games_from_file(file, &mut vec_games_str);
                    unsafe {
                        treegames::VEC_GAMES = Some(vec_games_str);
                        clon1.model_tgames.clear();
                    }
                },
                None => { return; },
            }


            clon1.clone().model_tgames.clear();
            /*
                to start the new list of games with an empty one in san-textview
            */
            // recover the weakref
            let mut txtview = match weak_san_viewer.upgrade() {
                Some(txtview) => txtview,
                None => return,
            };
            let mut header_clon5 = header.clone();
            super::notationview::reset_buffer(&txtview);
            let game_str = globs::EMPTY_GAME.to_string();
            super::mainwin::init_game(&mut header_clon5, &mut txtview, &game_str, 0_u64);
            /*
                end of empty game
            */
            clon1.clone().display_data();
            //clon1.closures_treegame (
            //    view_clon5,
            //    header_clon5);
            area_clon5.queue_draw();
        });


        let area_clon6 = board_area.clone();
        let view_clon6 = txtview.clone();
        self.btn_cut.connect_clicked ( move |_| {

            let mut nodes: globs::ListNodes;
            let mut cur_node: String;
            unsafe {
                cur_node = NODO_ACTIVO.clone().unwrap();
            }

            strip_variation_handler(cur_node);
            
            unsafe {
                nodes = NODOS.clone().unwrap();
                cur_node = NODO_ACTIVO.clone().unwrap();
            }

            // we need to create new textbuffer and fixed texttags
            let txtbuf = gtk::TextBuffer::new::<gtk::TextTagTable>(None::<&gtk::TextTagTable>);
            view_clon6.set_buffer(Some(&txtbuf));
            
            // creamos los tags estáticos
            ttbuf::tags_branchlevel(&view_clon6);
            //ttbuf::tags_mvnr(&view_clon6);
            ttbuf::tags_move(&view_clon6);
            //ttbuf::tags_nag(&view_clon6);
            //ttbuf::tags_comment(&view_clon6);

            draw_notation(&view_clon6, &mut nodes);

            // now we allocate over the first un-cutted move
            super::buttonsboard::btns_delete_variation(nodes, &view_clon6, &area_clon6, cur_node);
        });

        let area_clon7 = board_area.clone();
        let view_clon7 = txtview.clone();
        self.btn_del.connect_clicked ( move |_| {
            
            let mut nodes: globs::ListNodes;
            let mut cur_node: String;
            unsafe {
                cur_node = NODO_ACTIVO.clone().unwrap();
                nodes = NODOS.clone().unwrap();
            }

            // primero necesitamos ponernos en el primer nodo de la variante
            // y luego procedemos igual con strip_variation_handler
            // we are into main line
            let mut first_node_indx: String;    // = "".to_string();
            if &cur_node[cur_node.len()-1..] == "z" && !cur_node.contains("n") {
                cur_node = "1z".to_string();
            }
            // maybe we are in a variation. eg: 72z1n1z
            while cur_node.len() > 2 && cur_node.contains("n") { 
                if cur_node.ends_with("n") {    // ya estábamos al inicio de la variante a borrar
                    break;
                }
                first_node_indx = nodes.nodes[&cur_node].parent_indx.clone();
                // puede ser que en lugar de 1z encontremos 81z por ejemplo
                // por tanto...
                if &first_node_indx[first_node_indx.len()-1..] == "n" { 
                    cur_node = first_node_indx.clone();
                    break; 
                }
                if first_node_indx.len() > 2 {
                    let is_num: char = first_node_indx[first_node_indx.len()-3..first_node_indx.len()-2]
                                .chars().next().expect("string is empty");
                    if is_num.is_ascii_digit() {
                        first_node_indx = nodes.nodes[&first_node_indx].parent_indx.clone();
                        cur_node = first_node_indx.clone();
                        continue;
                    }
                }
                cur_node = first_node_indx.clone();
            }

            if cur_node == "1z".to_string() {
                // the first visible move
                cur_node = "2z".to_string();
            }
            
            super::buttonsboard::btns_select_node(nodes, &view_clon7, &area_clon7, cur_node.clone());

            // ahora borramos la variante
            strip_variation_handler(cur_node);
            
            unsafe {
                nodes = NODOS.clone().unwrap();
                cur_node = NODO_ACTIVO.clone().unwrap();
            }

            // we need to create new textbuffer and fixed texttags
            let txtbuf = gtk::TextBuffer::new::<gtk::TextTagTable>(None::<&gtk::TextTagTable>);
            view_clon7.set_buffer(Some(&txtbuf));
            
            // creamos los tags estáticos
            ttbuf::tags_branchlevel(&view_clon7);
            //ttbuf::tags_mvnr(&view_clon7);
            ttbuf::tags_move(&view_clon7);
            //ttbuf::tags_nag(&view_clon7);
            //ttbuf::tags_comment(&view_clon7);

            draw_notation(&view_clon7, &mut nodes);

            // now we allocate over the first un-cutted move
            super::buttonsboard::btns_delete_variation(nodes, &view_clon7, &area_clon7, cur_node.clone());
        });

        let area_clon8 = board_area.clone();
        let view_clon8 = txtview.clone();
        self.btn_prom.connect_clicked ( move |_| {
            
            let mut nodes: globs::ListNodes;
            let cur_node: String;

            promote_variation_handler();

            unsafe {
                nodes = NODOS.clone().unwrap();
                cur_node = NODO_ACTIVO.clone().unwrap();
            }

            // we need to create new textbuffer and fixed texttags
            let txtbuf = gtk::TextBuffer::new::<gtk::TextTagTable>(None::<&gtk::TextTagTable>);
            view_clon8.set_buffer(Some(&txtbuf));
            
            // creamos los tags estáticos
            ttbuf::tags_branchlevel(&view_clon8);
            //ttbuf::tags_mvnr(&view_clon8);
            ttbuf::tags_move(&view_clon8);
            //ttbuf::tags_nag(&view_clon6);
            //ttbuf::tags_comment(&view_clon6);

            draw_notation(&view_clon8, &mut nodes);

            // now we allocate over the first un-cutted move
            super::buttonsboard::btns_delete_variation(nodes, &view_clon8, &area_clon8, cur_node);
        });


        self.btn_save_pgn.connect_clicked ( move |_| {
            
            utils::chooser_savepgn ();
            
        });


        let area_clon1 = board_area.clone();
        let view_clon1 = txtview.clone();
        self.btn_nag.connect_clicked ( move |_| {

            nagcomments::crea_menu_nag(&view_clon1, &area_clon1);

        });


        self.btn_search.connect_clicked ( move |_| {
            let vec_games: Vec<String>;
            unsafe {
                match super::treegames::VEC_GAMES.clone() {
                    Some(v) => {
                        vec_games = v.clone();
                    },
                    None => {
                        utils::alerta("A PGN file must be active to perform a search on it.");
                        return;
                    },
                }
            }
            
            let search_str = fensearch::create_fen();

            match search_str {
                Some(s) => {
                    /*
                    lbl.set_text(s.search_str.as_str());
                    while gtk::events_pending() {
                        gtk::main_iteration();
                    }
                    */
                    let games: &str = include_str!("../../pgns/chessok_com.pgn");
                    //let mut matched_games: Vec<i32> = Vec::new();
                    if s.pos_type == fensearch::PositionType::StandardFen {
                        //let mut vec_games: Vec<String> = Vec::new();
                        ph::read_games_string(games.to_string(), &mut vec_games.clone());
                        matchedgames::matched_dialog(s.clone(), vec_games.clone());
    
                    }
    
                    if s.pos_type == fensearch::PositionType::PiecesFen {
                        /*
                        let mut fen = s.search_str.clone();
                        let mut board = wchess::new_board();
                        // a ficticious starting point
                        // we shall work with the array wchess -> board -> square
                        // see line 498 of wchess::board
                        
                        fen.push_str(" w - - 0 1");
                        wchess::set_fen(&mut board, fen.as_str());
                        let arr_to_search = wchess::get_board_array(&mut board);
                        */
                        // create the vector of games
                        //let mut vec_games: Vec<String> = Vec::new();
                        ph::read_games_string(games.to_string(), &mut vec_games.clone());
                        matchedgames::matched_dialog(s.clone(), vec_games.clone());
                        
                    }
    
                    if s.pos_type == fensearch::PositionType::PiecesMaterial {
                        
                        //let mut vec_games: Vec<String> = Vec::new();
                        ph::read_games_string(games.to_string(), &mut vec_games.clone());
                        matchedgames::matched_dialog(s.clone(), vec_games.clone());
                        
                    }
                },
                None => {utils::alerta("No search pattern found"); },
            };
        });
    }
}

