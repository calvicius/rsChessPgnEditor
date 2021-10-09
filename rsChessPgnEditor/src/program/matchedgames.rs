use regex::Regex;
use std::time;
use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;

use gtk::*;
use gtk::prelude::*;
//use gdk_pixbuf::Pixbuf;
use gdk::prelude::GdkContextExt;
use cairo::{SolidPattern};

use super::fensearch;
use super::pgn;
use super::wchess;
use super::globs;
use super::pieces;
use super::tags_textbuffer as ttbuf;


static mut VEC_GAMES: Option<Vec<String>> = None;
static mut CANCELLED: bool = false;
static mut INDEX_GAMES: Option<Vec<i32>> = None;


pub const CSS_PANED_MATCHED: &'static str = "
#panedgame > separator { 

    min-width: 0px; 
    min-height: 0px;
    border-style: none;
    background-size: 0px 0px;
    background-color: #F2F2F2;
}
#panedgame > separator.wide { 
    min-width: 0px; 
    min-height: 0px; 
    background-color: #F2F2F2;
}

#panelmatched {
    background-color: rgba(242,242,242,0.90);
    border: solid 0px;
    margin: 2px;
}

#panelmatched > separator {
    background-color: rgba(242,242,242,0.90);
    margin: 0px;
    padding: 0px;
    background-size: 0px 0px;
}

#panelright {
    background-color: rgba(242,242,242,0.90);
    border: solid 0px;
    margin: 2px;
}

#panelright > separator {
    background-color: rgba(242,242,242,0.90);
    margin: 0px;
    padding: 0px;
    background-size: 0px 0px;
}

#barmatched { 
    background-color: rgba(44,63,117, 0.99);
    color: #ececec;
    /*padding-right: 25px;*/
    padding-left: 30px;
    
    /*padding: 0.20625rem 0.20625rem 0.20625rem 0.20625rem;*/
}
";


pub fn matched_dialog(s: fensearch::DataPosition, games: Vec<String>) {
    // Cargamos el CSS
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSS_PANED_MATCHED.as_bytes())
        .expect("Failed to load CSS");
    // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
    // que agregamos se puedan aplicar a nuestra ventana.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    unsafe {
        INDEX_GAMES = None;
        CANCELLED = false;
        VEC_GAMES = Some(games);
    }
    
    let dialog = gtk::Dialog::with_buttons(
            Some("Search matched games"),
            None::<&gtk::Window>,   // es el parent
            gtk::DialogFlags::MODAL,
            &[]
        );

    gtk::WidgetExt::set_widget_name(&dialog, "dialmatched");
    dialog.set_position(gtk::WindowPosition::CenterAlways);
    let top_area = dialog.get_content_area(); // -> Box

    let panel_matched = gtk::Paned::new(Orientation::Horizontal);
    gtk::Paned::set_size_request (&panel_matched, 900, 750);
    panel_matched.set_position(500);
    gtk::WidgetExt::set_widget_name(&panel_matched, "panelmatched");

    let vbox_tree = gtk::Box::new(Orientation::Vertical, 0);
    let tree = TreeGamesMatched::new(s);
    vbox_tree.pack_start(&tree.stat_bar, false, true, 0);
    vbox_tree.pack_start(&tree.scrolled_win, true, true, 0);
    panel_matched.add1(&vbox_tree);
    
    let panel_right = gtk::Paned::new(Orientation::Vertical);
    gtk::Paned::set_size_request (&panel_right, 400, 750);
    panel_right.set_position(400);
    gtk::WidgetExt::set_widget_name(&panel_right, "panelright");

    let frm = gtk::AspectFrame::new(None, 0.5, 0.1, 5.0, true);
    let board_area = BoardGuiMatched::new();
    frm.add(&board_area.area_tablero);
    panel_right.pack1(&frm, false, false);
    
    let vbox_text = gtk::Box::new(Orientation::Vertical, 0);
    let text_game = NotationViewMatched::new();
    vbox_text.pack_start(&text_game.scrolled, true, true, 5);
    panel_right.add2(&vbox_text);

    let vbox_game = gtk::Box::new(Orientation::Vertical, 0);
    vbox_game.pack_start(&panel_right, false, false, 0);
    panel_matched.add2(&vbox_game);

    top_area.pack_start(&panel_matched, false, true, 20);

    let stat_bar_matched = gtk::Statusbar::new();
    gtk::WidgetExt::set_widget_name(&stat_bar_matched, "barmatched");
    // Returns a new context identifier, given a description of the actual context.
    let id1 = stat_bar_matched.get_context_id("Matchedbar");
    stat_bar_matched.push(id1, " ");
    top_area.pack_start(&stat_bar_matched, true, true, 10);

    //llevamos una partida vacia a pantalla
    // así inicializamos la variable global NODOS
    let game: &str = globs::EMPTY_GAME;
    let mut view = text_game.clone().view;
    init_game(&mut view, game, 0_u64);
    let vw = text_game.view.downgrade();      // -> glib::object::WeakRef<gtk::TextView>
    tree.clone().closures_treegame(vw.clone());
    text_game.view_closures(board_area.area_tablero);

    let dial = dialog.clone();
    dial.show_all();
    while gtk::events_pending() {
        gtk::main_iteration();
    }
    
    dialog.connect_delete_event(move |_, _| {
        let processing: bool;
        unsafe {
            CANCELLED = true;
            processing = PROCESSING_GAME;
        }
        if processing {
            super::utils::alerta("A long game is processing. Await please.");
            Inhibit(true)
        }
        else {
            //unsafe { gtk::prelude::WidgetExtManual::destroy(&dial); }
            dial.close();
            Inhibit(false)
        }
    });
    
    tree.clone().display_data(&stat_bar_matched);
    /*
    let result = dialog.run();
    println!("result es {}", result);
    if result == gtk::ResponseType::Cancel.into() {
        println!("se ha presionado cancel");

    }
    */
    //dialog.close()
}


pub const CSSBARSTATUS: &'static str = "
#bar_tree { 
    background-color: rgba(44,63,117, 0.99);
    color: #ececec;
    /*padding-right: 25px;*/
    padding-left: 130px;
    
    /*padding: 0.20625rem 0.20625rem 0.20625rem 0.20625rem;*/
}
/*
#tree_view row:selected {
    border-color: #400;
    border-top-width: 2px;
    border-bottom-width: 2px;
    background-color: rgba(44,63,117, 0.90);
    color: #000;
}
*/
#tree_view.view:selected {
    /*background: rgba(11,97,11, 0.62);*/
    background-color: rgba(44,63,117, 0.70);
}
#tree_view {
    background-color: rgba(242,242,242,0.99);
}
#tree_view.view header {
    background-color: rgba(242,242,242,0.99);
}
";


#[derive(Clone)]
struct RecRow {
    gamenum: u64,
    white: String,
    black: String,
    result: String,
    eco: String,
}

impl RecRow {
    fn init(gamenum: u64, white: String, black: String, result: String, eco: String) -> RecRow {
        RecRow {
            gamenum,
            white,
            black,
            result,
            eco,
        }
    }
}

#[derive(Clone)]
pub struct TreeGamesMatched {
    pub scrolled_win: gtk::ScrolledWindow,
    pub model_tgames: gtk::ListStore,
    tview: gtk::TreeView,
    pub stat_bar: gtk::Statusbar,
    selection: gtk::TreeSelection,
    dataposition: fensearch::DataPosition,
}


impl TreeGamesMatched {
    pub fn new(s: fensearch::DataPosition) -> Self {
        // Cargamos el CSS
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSSBARSTATUS.as_bytes())
            .expect("Failed to load CSS");
        // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
        // que agregamos se puedan aplicar a nuestra ventana.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        let stat_bar = gtk::Statusbar::new();
        gtk::WidgetExt::set_widget_name(&stat_bar, "bar_tree");
        // Returns a new context identifier, given a description of the actual context.
        let id1 = stat_bar.get_context_id("Statusbar");
        stat_bar.push(id1, "Games matched in PGN file");

        let scrolled_win = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_win.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        gtk::WidgetExt::set_widget_name(&scrolled_win, "scrolltree");

        let model_tgames = gtk::ListStore::new(&[glib::types::Type::U64,
                glib::types::Type::String, 
                glib::types::Type::String,
                glib::types::Type::String,
                glib::types::Type::String,
            ]);
        let field_header: [&str; 5] = ["#", "White", "Black", "Result", "ECO"];
        let field_justification: [f32; 5] = [1.0, 0.0, 0.0, 0.5, 0.0];

        // Creates the view to display the list store
        let tview = gtk::TreeView::with_model(&model_tgames);
        gtk::WidgetExt::set_widget_name(&tview, "tree_view");
        scrolled_win.add(&tview);
        let selection = tview.clone().get_selection();

        let tg = TreeGamesMatched {
            scrolled_win: scrolled_win,
            model_tgames: model_tgames,
            tview: tview,
            stat_bar: stat_bar,
            selection: selection,
            dataposition: s,
        };
        
        // Creamos las columnas
        for col in 0..field_header.len() {
            let cell_renderer = gtk::CellRendererText::new(); 
            cell_renderer.set_property_xalign(field_justification[col]);
            cell_renderer.set_padding(5, 2);
            
            let column = gtk::TreeViewColumn::new();
            column.pack_start(&cell_renderer, true);
            column.add_attribute(&cell_renderer, "text", col as i32);
            column.set_alignment(field_justification[col]); 
            column.set_sort_column_id(col as i32); 
            
            let label = gtk::Label::new(Some(field_header[col]));
            
            // aqui vamos a cambiar el tamaño y apariencia del texto de la etiqueta
            let attr_list = pango::AttrList::new();
            
            //let mut attr = pango::Attribute::new_foreground(2048, 33423, 2048)
            //                    .expect("Couldn't create new foreground");
            let mut attr = pango::Attribute::new_foreground(115*255, 132*255, 181*255)
                                .expect("Couldn't create new foreground");
            attr.set_start_index(0);
            attr_list.insert(attr);

            let mut attr = pango::Attribute::new_scale(1.2)
                                    .expect("Couldn't create new scale");
            attr.set_start_index(0);
            attr_list.insert(attr);

            label.set_attributes(Some(&attr_list));
            
            
            column.set_widget(Some(&label)); 
            label.show();
            // fin del texto de la etiqueta
            
            // Configurar la función autodefinida para mostrar el color de la fila alternativa
            // https://docs.rs/gtk/0.6.0/gtk/trait.TreeViewColumnExt.html#tymethod.set_cell_data_func
            gtk::TreeViewColumnExt::set_cell_data_func(&column, &cell_renderer, 
                                            Some(std::boxed::Box::new(format_color)));
            tg.tview.append_column(&column);
        }
        
        tg
    }


    pub fn closures_treegame (mut self, 
            weak_san_viewer: glib::object::WeakRef<gtk::TextView>) {

        // recover the weakref
        let txtview = match weak_san_viewer.upgrade() {
            Some(txtview) => txtview,
            None => return,
        };
        
        
        self.selection = self.tview.get_selection();
        // https://gtk-rs.org/docs/gtk/trait.TreeSelectionExt.html#tymethod.connect_changed
        self.selection.connect_changed(move |widget| {
            while gtk::events_pending() {
                gtk::main_iteration();
            }
            if let Some((modelo, iter)) = widget.get_selected() {
                let mut txtview_clon = txtview.clone();
                
                let game_str: String;
                let data_idx: Option<Vec<i32>>;
                unsafe {
                    data_idx = INDEX_GAMES.clone();
                }

                if data_idx.is_some() {
                    //obtenemos el numero de partida de la fila seleccionada
                    let game_number = modelo.get_value(&iter, 0);

                    // necesitamos traducir estos valores para ser entendibles
                    let game_number1 = game_number.get::<u64>().unwrap().unwrap();

                    let data_opt: Option<Vec<String>>;
                    unsafe {
                        data_opt = VEC_GAMES.clone();
                    }

                    let data_vec = data_opt.unwrap();
                    game_str = data_vec[game_number1 as usize].clone();
                    
                    unsafe {
                        ITERS_FONDO_TAG = None;
                        CASILLA_ORIGEN = Some("999".to_string());   //casilla ficticia
                        //CABECERA = None;
                        NODOS = None;
                        FEN_ACTUAL = None;
                        NODO_ACTIVO = None;
                    }
                    
                    
                    reset_buffer(&txtview_clon);
                    init_game(&mut txtview_clon, &game_str, game_number1 as u64);
                    
                }
            }
            
        });
        
    }


    pub fn display_data(self, stat_bar: &gtk::Statusbar) {
        
        // poblamos el arbol
        let data_opt: Option<Vec<String>>;
        unsafe {
            data_opt = VEC_GAMES.clone();
        }
        
        if data_opt.is_some() {
            let mut fen = self.dataposition.search_str.clone();
            let mut board = wchess::new_board();
            
            self.model_tgames.clear();  // if is present another pgn file
            while gtk::events_pending() {
                gtk::main_iteration();
            }
            let raw_data = data_opt.unwrap();
            // Temporalmente "congela / freeze" la actualización.
            //self.tview.freeze_child_notify(); 
            
            for i in 0..raw_data.len() {
                let cancelled: bool;
                unsafe {
                    cancelled = CANCELLED.clone();
                }
                if cancelled { break; }
                
                let mut game_num: i32 = 0;

                if self.dataposition.pos_type == fensearch::PositionType::StandardFen {
                    // a ficticious starting point
                    // we shall work with the array wchess -> board -> square
                    fen.push_str(" w - - 0 1");
                    wchess::set_fen(&mut board, fen.as_str());
                    let arr_to_search = wchess::get_board_array(&mut board);

                    let (tx, rx) = mpsc::channel();
                    let mut game = raw_data[i].clone();
                    thread::spawn(move || {
                        let data_game = pgn::parse_pgn_moves(&mut game);
                        let gamenum = pgn::search_pattern_exact_fen (data_game.moves, data_game.fen, i, arr_to_search);
                        tx.send(gamenum).unwrap();
                    });
                    game_num = rx.recv().unwrap();
                }
                else if self.dataposition.pos_type == fensearch::PositionType::PiecesFen {
                    // a ficticious starting point
                    // we shall work with the array wchess -> board -> square
                    fen.push_str(" w - - 0 1");
                    wchess::set_fen(&mut board, fen.as_str());
                    let arr_to_search = wchess::get_board_array(&mut board);

                    let (tx, rx) = mpsc::channel();
                    let mut game = raw_data[i].clone();
                    thread::spawn(move || {
                        let data_game = pgn::parse_pgn_moves(&mut game);
                        let gamenum = pgn::search_pattern_position_fen (data_game.moves, data_game.fen, i, arr_to_search);
                        tx.send(gamenum).unwrap();
                    });
                    game_num = rx.recv().unwrap();
                }
                else if self.dataposition.pos_type == fensearch::PositionType::PiecesMaterial {
                    let (tx, rx) = mpsc::channel();
                        
                    let mut game = raw_data[i].clone();
                    let cond = self.dataposition.search_str.clone(); //condition.clone();
                    thread::spawn(move || {
                        let data_game = pgn::parse_pgn_moves(&mut game);
                        let gamenum = pgn::search_pattern_material (data_game.moves, data_game.fen, i, cond);
                        tx.send(gamenum).unwrap();
                    });
                    game_num = rx.recv().unwrap();
                }
                else {
                    //
                }

                let msg = format!("Examined {} games", i);
                let id1 = stat_bar.get_context_id("Matchedbar");
                stat_bar.push(id1, &msg);
                while gtk::events_pending() {
                    gtk::main_iteration();
                }
                let five_millis = time::Duration::from_millis(5);
                thread::sleep(five_millis);

                if game_num < 0 {
                    continue;
                }
                
                unsafe {
                    if INDEX_GAMES.is_none() {
                        INDEX_GAMES = Some(vec![game_num]);
                    }
                    else {
                        let mut indexes = INDEX_GAMES.clone().unwrap();
                        indexes.push(game_num);
                        INDEX_GAMES = Some(indexes);
                    }
                }
                
                let mut elem = RecRow::init(0_u64, "".to_string(), "".to_string(), "*".to_string(), "000".to_string());
                elem.gamenum = (i as u64).clone();
                
                // reemplaza saltos de linea por espacios
                let game_vec: Vec<&str> = raw_data[i].split("\n").collect();
                let re = Regex::new(r"\r?\n|\r").unwrap();
                for tag in game_vec {
                    let tag_stripped = re.replace_all(tag, " ");
                    let inner_re = Regex::new(r"^\s*\[[^%].*?\]");
                    let match_header = inner_re.unwrap().captures(&tag_stripped);
                    // the header
                    if match_header.is_some() {
                        let tag_vec: Vec<&str> = tag_stripped.split("\"").collect();
                        if tag_stripped.contains("[White ") {
                            let tmp = tag_vec[1].to_string();
                            if tmp.len() > 15 {
                                elem.white = tmp[0..15].to_string();
                            }
                            else { elem.white = tmp; }
                        }
                        else if tag_stripped.contains("[Black ") {
                            let tmp = tag_vec[1].to_string();
                            if tmp.len() > 15 {
                                elem.black = tmp[0..15].to_string();
                            }
                            else { elem.black = tmp; }
                        }
                        else if tag_stripped.contains("Result") {
                            elem.result = tag_vec[1].to_string();
                        }
                        else if tag_stripped.contains("ECO") {
                            elem.eco = tag_vec[1].to_string();
                        }
                    }
                }
                self.model_tgames.insert_with_values(None,
                    &[0, 1, 2, 3, 4], 
                    &[&elem.gamenum, &elem.white, &elem.black, &elem.result, &elem.eco]);
                    
                if i%4 == 0 {
                    while gtk::events_pending() {
                        gtk::main_iteration();
                    }
                }
                
            }
            //Reactiva la actualización
            //self.tview.thaw_child_notify(); // nota 3
        }
    }


}


fn format_color (_col: &TreeViewColumn,
        cell: &gtk::CellRenderer,
        model: &gtk::TreeModel,
        iter: &gtk::TreeIter
        ) {
    
    let path = model.get_path(iter); // -> Option<TreePath>
    match path {
        Some(path) => {
            let indices = gtk::TreePath::get_indices(&path);    // -> Vec<i32>
            let row_num = indices[0];
            let row_color: &str;
            if row_num%2 == 1 {
                row_color = "#F2FBEF";
            }
            else {
                row_color = "#f2f2f2";
            }
            cell.set_property_cell_background(Some(row_color));
        },
        None => {},
    }
}


const FILAS: i32 = 8;
const COLUMNAS: i32 = 8;
const COLOR1: (f64, f64, f64) = (115.0 / 255.0, 132.0 / 255.0, 181.0 / 255.0);
const COLOR2: (f64, f64, f64) = (239.0 / 255.0, 239.0 / 255.0, 239.0 / 255.0);
const DIR_PIECES: &str = "./piezas/Merida96/";

static mut DIM_SQUARE: f64 = 20.0;
static mut CASILLA_ORIGEN: Option<String> = None;
static mut TURNO: u8 = 0;
static mut FEN_ACTUAL: Option<String> = None;

const Y_EJE: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const X_EJE: [i32; 8] = [1,2,3,4,5,6,7,8];

#[derive(Clone, PartialEq)]
pub struct BoardGuiMatched {
    pub area_tablero : gtk::DrawingArea,
    pub listapiezas : HashMap<String,  gdk_pixbuf::Pixbuf>
}


impl BoardGuiMatched {
    pub fn new() -> Self {
        let area_tablero = gtk::DrawingArea::new();
        let lista_piezas = pieces::crea_lista_piezas(DIR_PIECES);
        
        let list_piec = lista_piezas.clone();

        area_tablero.connect_draw ( move |widget, ctx| {
            let mut color = COLOR2;
            let dim_square: f64;
            unsafe {
                DIM_SQUARE = (widget.get_allocated_width() / 8) as f64;
                dim_square = DIM_SQUARE;
            }
            
            // el padding
            let leftover_space = widget.get_allocated_width() as f64 - 
                    dim_square * 8.0;
            let padding = leftover_space / 2.0;
            cairo::Context::translate(ctx, padding as f64, padding as f64);

            // creamos un tablero interno legible
            let current_fen: String;
            unsafe {
                match FEN_ACTUAL.clone() {
                    Some(vfen) => current_fen = vfen,
                    None => {
                        FEN_ACTUAL = Some(globs::DEFAULT_POSITION.to_string());
                        current_fen = globs::DEFAULT_POSITION.to_string();
                    },
                };
            }
            
            let mut tablero = wchess::new_board();
            let _fen_valida = wchess::set_fen(&mut tablero, &current_fen );
            unsafe { 
                TURNO = tablero.next_move; 
            }
            let grafico = wchess::graphical_board(&mut tablero);
            let tablero_interno = procesa_notacion(grafico);
            
            // el tablero
            for r in 0..FILAS {
                if color == COLOR2 {
                    color = COLOR1;
                } else {
                    color = COLOR2; 
                }
                for c in 0..COLUMNAS {
                    let x1 = c as f64 * dim_square;
                    let y1 = (7-r) as f64 * dim_square;
                    ctx.set_source_rgb(color.0, color.1, color.2);
                    ctx.rectangle(x1, y1, dim_square, dim_square);
                    ctx.fill();
                    if color == COLOR2 {
                        color = COLOR1;
                    } else {
                        color = COLOR2; 
                    }
                }
            }

            // Dibujamos las coordenadas
            let tx = dim_square;
            let height = tx * 8.0;
            let width = tx * 8.0;
            let padding_coord = dim_square / 60.0;
            
            let mut solido = SolidPattern::from_rgb(30.0/255.0, 30.0/255.0, 30.0/255.0); // (red: f64, green: f64, blue: f64) -> SolidPattern
            //let dark_square_pattern = cairo::Pattern::SolidPattern(solido);
            let dark_square_pattern = solido;
            solido = SolidPattern::from_rgb(239.0/255.0, 239.0/255.0, 239.0/255.0);
            //let light_square_pattern = cairo::Pattern::SolidPattern(solido);
            let light_square_pattern = solido;
            
            let fuente = format!("{}{}", "Sans ", (10.0 * dim_square / 80.0) as i32);
            let layout = pangocairo::functions::create_layout(ctx).expect("error layout draw coordinates"); // -> Option<Layout>
            let mut desc = pango::FontDescription::from_string(&fuente);  // -> FontDescription
            pango::FontDescription::set_weight(&mut desc, pango::Weight::Semibold);
            layout.set_font_description(Some(&desc));

            // Column names
            let mut light = true;
            for j in 0..8 {
                cairo::Context::save(ctx);
                let light_dark;
                {
                    if light {
                        light_dark = light_square_pattern.clone();
                    }
                    else {
                        light_dark = dark_square_pattern.clone();
                    }
                }
                cairo::Context::set_source(ctx, &light_dark);
                let coord;
                {
                    let _h: u8 = 104;  // ascii values
                    let a: u8 = 97;
                    coord = ((a + j) as char).to_string();
                }
                layout.set_text(&coord);
                let (_pix_ancho, pix_alto) = layout.get_pixel_size();
                cairo::Context::translate(ctx, (j as f64 * tx) + padding_coord +2.0, height as f64 - pix_alto as f64 - padding_coord); // (&self, tx: f64, ty: f64)
                pangocairo::functions::show_layout(ctx, &layout);
                cairo::Context::restore(ctx);
                light = !light;
            }
            
            // Rank numbers
            light = true;
            for j in 0..8 {
                cairo::Context::save(ctx);
                let light_dark;
                {
                    if light {
                        light_dark = light_square_pattern.clone();
                    }
                    else {
                        light_dark = dark_square_pattern.clone();
                    }
                }
                cairo::Context::set_source(ctx, &light_dark);
                let coord;
                {
                    let _uno: u8 = 49;  // ascii values
                    let ocho: u8 = 56;
                    coord = ((ocho - j) as char).to_string();
                }
                layout.set_text(&coord);
                let (pix_ancho, _pix_alto) = layout.get_pixel_size();
                cairo::Context::translate(ctx, width as f64 - pix_ancho as f64-2.0, (j as f64 * tx) + padding_coord);
                pangocairo::functions::show_layout(ctx, &layout);
                cairo::Context::restore(ctx);
                light = !light;
            }

            // las piezas
            for (xycoord, valor) in &tablero_interno {
                let (x, y) = num_notacion(xycoord);
                let x0 = (y as f64 * dim_square) + 
                    (dim_square/16.0);
                let y0 = ((7-x) as f64 * dim_square) + 
                    (dim_square/16.0);
                
                let pieza = list_piec.get(valor)
                    .expect("error al obtener la pieza");
                let mut minimum_dim_square: i32 = 1;
                if (dim_square * 0.90) as i32 >= 1 {
                    minimum_dim_square = (dim_square * 0.90) as i32;
                }
                let pixbuf = pieza.scale_simple (
                    minimum_dim_square,
                    minimum_dim_square,
                    gdk_pixbuf::InterpType::Bilinear
                    ).expect("error al escalar pixbuf");
                let _sr1 = ctx.set_source_pixbuf(&pixbuf, x0, y0);
                
                ctx.paint();
            }

            Inhibit(false)
        });

        
        BoardGuiMatched {
            area_tablero: area_tablero,
            listapiezas: lista_piezas,
        }
    }
}

  

/*
 funciones para crear un tablero interno del tablero grafico
 ===========================================================
*/

fn procesa_notacion(arr_piezas: Vec<&str>) -> HashMap<String, String> {

    let mut tablero: HashMap<String, String> = HashMap::new();
    let mut grafico: Vec<Vec<&str>> = Vec::new();
    let mut temporal: Vec<&str> = Vec::new();
    let sitio_piezas = arr_piezas.clone();

    // ahora hacemos un array bidimensional
    for i in 0..8 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 8..16 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 16..24 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 24..32 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 32..40 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 40..48 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 48..56 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 56..64 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);

    for fila in 0..grafico.len() {
        for col in 0..grafico[fila].len() {
            let alfabeto = &grafico[fila][col];
            if *alfabeto == "*".to_string() {   //empty square
                continue;
            }
            
            let xycoord = alfa_notacion((7-fila, col));
            if xycoord != "None" {
                tablero.insert(xycoord, alfabeto.to_string());
            }
        }
    }
    
    tablero
}


// Necesitamos una manera de convertir las coordenadas x e y de una pieza 
// a su notación equivalente alfabética, por ejemplo, A1, D5, E3, etc.
fn alfa_notacion (xycoord: (usize, usize)) -> String {

    if !esta_en_tablero(xycoord) {
        return "None".to_string();
    }
    format!("{}{}",Y_EJE[xycoord.1], X_EJE[xycoord.0])
}

// la definición de un método para comprobar si una determinada
// coordenada está en el tablero
fn esta_en_tablero(coord: (usize, usize)) -> bool {
    if coord.1 > 7 || coord.0 > 7 {
        return false;
    }
    else { return true; }
}

// Necesitamos convertir una notacion a1, a8, etc a coordenadas x,y
// definimos un método que toma una coordenada x, y como una tupla y 
// devuelve su notación numérica equivalente, de la siguiente manera:
fn num_notacion(xycoord: &str) -> (usize, usize) {
    let car = xycoord.chars().nth(0).unwrap();
    let num_car = xycoord.chars().nth(1).unwrap();
    let col = Y_EJE.iter().position(|&x| x == car)
        .expect("error al obtener el num de col."); // Option<usize>
    let fila = (num_car.to_string()).parse::<usize>().unwrap() - 1;

    (fila, col)
}

/*
Fin de las funciones del tablero interno
*/ 




/*
El textview
*/

pub static mut ITERS_FONDO_TAG: Option<(gtk::TextIter, gtk::TextIter)> = None;
pub static mut NODOS: Option<globs::ListNodes> = None;
pub static mut NODO_ACTIVO: Option<String> = None;
//pub static mut CABECERA: Option<globs::GameInfo> = None;

const CSS_TEXTVIEW: &'static str = "
#san_text, text{
    background-color: rgba(242,242,242,0.99);
    font-size: 16px;
    font-family: Segoe UI;
    padding: 15px;
}
#scrolled {
    padding-right: 5px;
}
#scrolled slider { 
    min-width: 0.7rem; 
    min-height: 0.7rem;
}
";

#[derive(Clone)]
pub struct NotationViewMatched {
    pub view: gtk::TextView,
    pub scrolled: gtk::ScrolledWindow,
}


impl NotationViewMatched {
    pub fn new() -> Self {
        // Cargamos el CSS sucede aquí.
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSS_TEXTVIEW.as_bytes())
            .expect("Failed to load CSS");
        // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
        // que agregamos se puedan aplicar a nuestra ventana.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        // Cree un nuevo búfer y una nueva vista para mostrar el búfer.
        let mut view = gtk::TextView::new();
        let buffer = view.get_buffer().expect("error");
        // inicializamos/vaciamos el buffer;
        buffer.set_text("");
        
        view.set_wrap_mode(gtk::WrapMode::WordChar);
        view.set_cursor_visible(false);
        view.set_editable(false);
        view.set_left_margin(5);
        view.set_right_margin(25);
        view.set_wrap_mode(gtk::WrapMode::Word);

        // Set up a scroll window
        let scrolled_win = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_win.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        gtk::WidgetExt::set_widget_name(&scrolled_win, "scrolled");
        scrolled_win.add(&view);
   
        //view.grab_focus();  // poner el foco despues de empaquetarlo dentro de otro widget
        gtk::WidgetExt::set_widget_name(&view, "san_text");

        // creamos los tags de cada jugada
        ttbuf::tags_branchlevel_matched(&mut view);
        //ttbuf::tags_mvnr(&mut view);
        ttbuf::tags_move(&mut view);

        NotationViewMatched {
            view: view,
            scrolled: scrolled_win,
        }
    }


    pub fn view_closures(self, bgui: gtk::DrawingArea) {
        unsafe {
            if NODOS.is_none() { return; }
        }

        self.view.connect_motion_notify_event ( move |widget, event| {
            mueve_raton (widget, event);
            Inhibit(false)
        });

        self.view.connect_event_after ( move |widget, event| {
            let ex: f64;
            let ey: f64;
            
            let ev = event.get_event_type();
            if ev == gdk::EventType::ButtonPress {
                let boton = event.get_button().expect("error");
                if boton != 1 {
                    return;
                }
                let pos = event.get_coords().expect("error");
                ex = pos.0;
                ey = pos.1;
            }
            else {
                return;
            }
            
            let buffer = widget.get_buffer().expect("error");
            // no debemos seguir un enlace si el usuario ha seleccionado algo
            let hay_inicio_fin_iter = buffer.get_selection_bounds();
            match hay_inicio_fin_iter {
              Some(iteradores) => {
                if iteradores.0.get_offset () != iteradores.1.get_offset () {
                  return;
                }
              },
              None => {},
            }
            
            let (x, y) = widget.window_to_buffer_coords (gtk::TextWindowType::Widget, ex as i32, ey as i32);
            if let Some(iter) = widget.get_iter_at_location(x, y) {
              continua_si_link(&iter, widget, &bgui.clone());
            }
        });
    }
}


fn mueve_raton (view: &gtk::TextView,
        event: &gdk::EventMotion) {

    let (ex, ey) = gdk::EventMotion::get_position(event);
    let (x, y) = view.window_to_buffer_coords (gtk::TextWindowType::Widget, ex as i32, ey as i32);
    poner_cursor_si_apropiado(&view, x as i32, y as i32);
}


fn poner_cursor_si_apropiado (view: &gtk::TextView,
        x: i32,
        y: i32) {
    
    let mut tag_string = "".to_string();

    if let Some(text_iter) = view.get_iter_at_location (x, y) {
        let vec_tags = text_iter.get_tags();

        for tag in vec_tags {
            let gstr = tag.get_property_name().expect("error");
            tag_string = format! ("{}", gstr.as_str());
        }
        if tag_string.len() > 0 {
            let display = gdk::Display::get_default()
                .expect("error en display");
            let gcursor = gdk::Cursor::new_for_display(&display, 
                gdk::CursorType::Hand2);
            let gwindow = gtk::TextViewExt::get_window(view, 
                gtk::TextWindowType::Text)
                    .expect("error en gwindow");
            gdk::WindowExt::set_cursor(&gwindow, Some(&gcursor));
        }
        else {
            //
        }
        
    }
}


fn continua_si_link (iter: &gtk::TextIter,
        view: &gtk::TextView,
        board_area: &gtk::DrawingArea) {
    
    let buf = view.get_buffer().unwrap();
    let nodes: globs::ListNodes;
    unsafe {
        nodes = NODOS.clone().unwrap();
    }
    let mut iter1 = iter;
    let tags = gtk::TextIter::get_tags (iter);
    
    for tag in &tags {
        while gtk::events_pending() {
            gtk::main_iteration();
        }
        let gstr = tag.get_property_name().expect("error");

        let re = Regex::new(r"^\d+z");
        if re.unwrap().is_match(&gstr.to_string()) {
            unsafe {
                let iters = ITERS_FONDO_TAG.clone();
                match iters {
                    Some(iters) => {
                        buf.remove_tag_by_name("selected", &iters.0, &iters.1);
                        },
                    None => {},
                };
            }

            // update board_area
            unsafe {
                FEN_ACTUAL = Some(nodes.nodes[&gstr.to_string()].fen.clone());
                NODO_ACTIVO = Some(gstr.to_string());
            }
            board_area.queue_draw();

            let start_tag_iter = gtk::TextIter::backward_search(&mut iter1,
                " ",
                gtk::TextSearchFlags::VISIBLE_ONLY,
                None).unwrap();
            
            let end_tag_iter: (gtk::TextIter, gtk::TextIter);
            let end_tag_iter1 = gtk::TextIter::forward_search(&mut iter1,
                " ",
                gtk::TextSearchFlags::VISIBLE_ONLY,
                None);
            if end_tag_iter1.is_none() {    // there isn't space after last move
                end_tag_iter = (buf.get_end_iter(), buf.get_end_iter());
            }
            else {
                end_tag_iter = end_tag_iter1.unwrap();
            }

            buf.apply_tag_by_name("selected", &start_tag_iter.1, &end_tag_iter.0);
            unsafe {
                ITERS_FONDO_TAG = Some((start_tag_iter.1, end_tag_iter.clone().0));
            }
        }
    }
}



fn reset_buffer(view: &gtk::TextView) {
    let txtbuf = gtk::TextBuffer::new::<gtk::TextTagTable>(None::<&gtk::TextTagTable>);
    view.set_buffer(Some(&txtbuf));
    // creamos los tags estáticos
    ttbuf::tags_branchlevel_matched(&view);
    //ttbuf::tags_mvnr(&view);
    ttbuf::tags_move(&view);
}

//use std::time::{Duration, Instant};
static mut PROCESSING_GAME: bool = false;
fn init_game(view: &mut gtk::TextView, game: &str, game_nr: u64) {
    load_gif(&view);
    view.show_all();
    
    unsafe { PROCESSING_GAME = true; }
    let mut db_entry = globs::DbEntry::new();
    //let start = Instant::now();
    // set an empty game for future DB
    db_entry.id = game_nr;    // this game number will be changed with db record number
    db_entry.pgn = game.to_string();

    read_nodes_from_file(&mut db_entry);
    while gtk::events_pending() {
        gtk::main_iteration();
    }
    
    draw_notation(view);
    unsafe { PROCESSING_GAME = false }
    
    //let duration = start.elapsed();
    //println!("matched games Time elapsed in expensive_function() is: {:?}", duration);
}


fn load_gif(view: &gtk::TextView) {

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.set_homogeneous(false);
    let gif_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let img = gtk::Image::from_file("./icons/Loading-bar.gif");
    gif_box.set_homogeneous(true);
    gif_box.pack_start(&img, true, true, 80);

    vbox.pack_start(&gif_box, true, true, 50);

    let buffer = view.get_buffer().expect("error");

    let marca = buffer.get_insert().expect("no se ha podido crear la marca");
    // ahora obtenemos el nombre de la marca gtk::TextMarkExt::get_name(&self) -> Option<GString>
    let gnombre = marca.get_name().expect("error al obtener el nombre de la marca");
    let nombre = gnombre.as_str();
    
    let cursor_pos = buffer.get_mark(&nombre)
            .expect("error al obtener el cursor_pos"); 
    let mut iter = buffer.get_iter_at_mark(&cursor_pos); 
    
    let anchor = buffer.create_child_anchor(&mut iter)
            .expect("error al obtener el anchor"); // -> Option<TextChildAnchor>
    //si usamos la img que hemos añadido al Box da un error al hacer gtk un get_parent()
    view.add_child_at_anchor(&vbox, &anchor);    // nota 7
    img.show();
    view.show_all();
}


fn draw_notation(view: &gtk::TextView) {
    let mut nodes: globs::ListNodes;
    unsafe {
        nodes = NODOS.clone().unwrap();
    }
    
    pgn::traverse_nodes(view, &mut nodes);
    while gtk::events_pending() {
        gtk::main_iteration();
    }
}



fn read_nodes_from_file(entry: &mut globs::DbEntry) {   //-> globs::ListNodes {

    let mut entry_clon = entry.clone();

    //sync thread
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let pgngame = pgn::parse_pgn_data(&mut entry_clon.pgn);
        tx.send(pgngame).unwrap();
    });
    let pgn_data: globs::PgnGame = rx.recv().unwrap();
    let nodes: globs::ListNodes = pgn::pgn_moves_to_nodes(pgn_data.moves, pgn_data.fen.clone());

    unsafe {
        NODOS = Some(nodes.clone());
        FEN_ACTUAL = Some(pgn_data.fen.clone());
    }

    //return nodes;
}
