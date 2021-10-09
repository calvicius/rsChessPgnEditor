
use gtk::*;
use gdk::prelude::{ GdkContextExt};
use gtk::prelude::{ WidgetExtManual};


use super::wchess;


#[derive(Clone)]
struct Squares {
    sqcolor: String,
    img_name: String,
}

impl Squares {
    pub fn new(sqcolor: String, img_name: String) -> Self {
        Squares {
            sqcolor,
            img_name,
        }
    }
}


// this is for returning data
#[derive(Clone, Debug, PartialEq)]
pub enum PositionType {
    PiecesFen,
    StandardFen,
    PiecesMaterial
}

#[derive(Clone)]
pub struct DataPosition {
    pub pos_type: PositionType,
    pub search_str: String,
}

impl DataPosition {
    fn data_exact_fen (fenstr: String) -> Self {
        DataPosition {
            pos_type: PositionType::StandardFen,
            search_str: fenstr,
        }
    }

    fn data_material (materialstr: String) -> Self {
        DataPosition {
            pos_type: PositionType::PiecesMaterial,
            search_str: materialstr,
        }
    }

    fn data_position_fen (fenstr: String) -> Self {
        DataPosition {
            pos_type: PositionType::PiecesFen,
            search_str: fenstr,
        }
    }
}


pub const CSS_MATCH_FEN: &'static str = "
#btnstart {
    font-size: 16px;
    transition: all 200ms ease-out;
    min-width:0px;
    min-height:0px;
    padding: 1px 5px;
    background-color: #2C3F75;
    border-color: #2C3F75;
    border-bottom-color: #2C3F75;
    background-image: none;
    outline-color: rgba(44, 63, 117, 0.3);
    color: #ececec;
    box-shadow: none;
}

#btnfen {
    font-size: 16px;
    transition: all 200ms ease-out;
    min-width:0px;
    min-height:0px;
    padding: 1px 5px;
    background-color: #2C3F75;
    border-color: #2C3F75;
    border-bottom-color: #2C3F75;
    background-image: none;
    outline-color: rgba(44, 63, 117, 0.3);
    color: #ececec;
    box-shadow: none;
}

#entryfen {
    min-height: 1.3875rem;
}

#dialfen .dialog-action-area { 
    /*margin: 0.825rem;*/
    padding: 10px;
}

#dialfen .dialog-action-area button {
    background-color: #333333;
    border-color: #333333;
    border-bottom-color: #333333;
    background-image: none;
    outline-color: rgba(44, 63, 117, 0.3);
    color: #ececec;
    box-shadow: none;
}

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

#panelfen {
    background-color: rgba(242,242,242,0.90);
    border: solid 0px;
    margin: 2px;
}

#panelfen > separator {
    background-color: rgba(242,242,242,0.90);
    margin: 0px;
    padding: 0px;
    background-size: 0px 0px;
}
/*
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
*/
";


const IMAGES: &str = "./piezas/piezasgif/";
const DEFAULT_SQUARE_SIZE: i32 = 40;
const BOARD_SIZE: i32 = 8;
static mut SELECTED_PIECE: Option<String> = None;
static mut FEN_BOARD: Option<Vec<Squares>> = None;
/*
const BOARD_FEN_TO_WCHESS_SQUARE: [i32; 64] = [
    56,57,58,59,60,61,62,63,
    48,49,50,51,52,53,54,55,
    40,41,42,43,44,45,46,47,
    32,33,34,35,36,37,38,39,
    24,25,26,27,28,29,30,31,
    16,17,18,19,20,21,22,23,
     8, 9,10,11,12,13,14,15,
     0, 1, 2, 3, 4, 5, 6, 7
];
*/
pub static mut DATA_POSITION: Option<DataPosition> = None;



pub fn create_fen() -> Option<DataPosition> {
    
    let dialog = gtk::Dialog::with_buttons(
                    Some("Construct FEN"),
                    None::<&gtk::Window>,   // es el parent
                    gtk::DialogFlags::MODAL,
                    &[(&"OK", gtk::ResponseType::Ok), (&"Cancel", gtk::ResponseType::Cancel)]
                );
    
    gtk::WidgetExt::set_widget_name(&dialog, "dialfen");
    dialog.set_position(gtk::WindowPosition::CenterAlways);
    let top_area = dialog.get_content_area(); // -> Box
    
    let panel_fen = gtk::Paned::new(Orientation::Horizontal);
    gtk::Paned::set_size_request (&panel_fen, 1100, 450);
    panel_fen.set_position(335);
    gtk::WidgetExt::set_widget_name(&panel_fen, "panelfen");

    let vbox_board = gtk::Box::new(Orientation::Vertical, 0);
    panel_fen.add1(&vbox_board);
    
    let vbox_options = gtk::Box::new(Orientation::Vertical, 0);
    panel_fen.add2(&vbox_options);

    let board_display: gtk::DrawingArea = gtk::DrawingArea::new();
    gtk::WidgetExt::set_tooltip_markup(&board_display, 
        Some("Left click: Puts Piece\nRight click: Removes Piece"));
    gtk::prelude::WidgetExtManual::add_events(&board_display, 
        gdk::EventMask::POINTER_MOTION_MASK | 
        gdk::EventMask::BUTTON_PRESS_MASK | 
        gdk::EventMask::BUTTON_RELEASE_MASK);

    init_board(&vbox_board, &board_display);
    init_options(&vbox_options, &board_display);
    
    top_area.pack_start(&panel_fen, false, true, 20);

    dialog.show_all();
    let result = dialog.run();
    
    
    if result == gtk::ResponseType::Ok.into() {
        let ret: Option<DataPosition>;
        unsafe {
            ret = DATA_POSITION.clone();
        }
        //dialog.close();
        unsafe { WidgetExtManual::destroy(&dialog); }
        return ret;
    }
    else {
        //dialog.close();
        unsafe { WidgetExtManual::destroy(&dialog); }
        return None;
    }

    //dialog.close();

}

/**********************
 * Left side of paned *
***********************/

pub fn init_board(vbox: &gtk::Box, board_display: &gtk::DrawingArea) {
    
    vbox.pack_start(board_display, true, true, 0);
    //let lbl = gtk::Label::new(None);
    //vbox.pack_start(&lbl, false, false, 0);
    
    let pieces_display: gtk::DrawingArea = gtk::DrawingArea::new();
    gtk::prelude::WidgetExtManual::add_events(&pieces_display, 
        gdk::EventMask::POINTER_MOTION_MASK | 
        gdk::EventMask::BUTTON_PRESS_MASK | 
        gdk::EventMask::BUTTON_RELEASE_MASK);
        
    vbox.pack_start(&pieces_display, true, true, 0);

    let brd = create_board();
    unsafe {
        FEN_BOARD = Some(brd.clone());
    }

    let pieces = create_pieces();

    {
        board_display.connect_draw(move |widget, ctx| {
            board_draw_callback(widget, ctx);
            Inhibit(false)
        });

        //let vars2 = vars.clone();
        let pieces2 = pieces.clone();
        pieces_display.connect_draw(move |widget, ctx| {
            let mut pieces1 = pieces2.clone();
            pieces_draw_callback(widget, ctx, &mut pieces1);
            Inhibit(false)
        });
    }

    // **** los closures del mouse click del vector/area pieces ******
    {
        let pieces1 = pieces.clone();
        pieces_display.connect_button_press_event ( move |_widget, event| {
            
            let e = event;
            
            /* Comienza la selección de una pieza */
            let col_tamano = DEFAULT_SQUARE_SIZE as f64;
            let fila_tamano = DEFAULT_SQUARE_SIZE as f64;
            
            if e.get_button() == 1 {
                let (x, y) = event.get_position();
                let col_nr = (x / col_tamano) as usize;
                let row_nr = (y / fila_tamano) as usize;
                let indx = row_nr + col_nr + 7*row_nr;

                let tmp = pieces1[indx].img_name.clone();
                unsafe {
                    SELECTED_PIECE = Some(tmp);
                }
            }

            Inhibit(false)
        });
    }

    // **** los closures del mouse click del vector/area board ******
    {
        board_display.connect_button_press_event ( move |widget, event| {
            
            let e = event;
            
            /* Comienza la colocacion de la pieza seleccionada */
            let col_tamano = DEFAULT_SQUARE_SIZE as f64;
            let fila_tamano = DEFAULT_SQUARE_SIZE as f64;
            
            // puts piece on board
            if e.get_button() == 1 {
                let (x, y) = event.get_position();
                let col_nr:i32 = (x / col_tamano) as i32;
                let row_nr:i32 = (y / fila_tamano) as i32;
                let indx = row_nr + col_nr + 7*row_nr;
                
                let piece: Option<String>;
                unsafe {
                    piece = SELECTED_PIECE.clone();
                }
                if piece.is_some() {
                    unsafe {
                        let mut brd = FEN_BOARD.clone().unwrap();
                        brd[indx as usize].img_name = piece.unwrap();
                        FEN_BOARD = Some(brd.clone());
                        SELECTED_PIECE = None;
                    }
                }
                
            }
            // delete piece from board
            if e.get_button() == 3 {
                let (x, y) = event.get_position();
                let col_nr:i32 = (x / col_tamano) as i32;
                let row_nr:i32 = (y / fila_tamano) as i32;
                let indx = row_nr + col_nr + 7*row_nr;
                unsafe {
                    let mut brd = FEN_BOARD.clone().unwrap();
                    brd[indx as usize].img_name = "sq".to_string();
                    FEN_BOARD = Some(brd.clone());
                    SELECTED_PIECE = None;
                }
            }
            widget.queue_draw();

            Inhibit(false)
        });
    }
}


fn pieces_draw_callback(area: &gtk::DrawingArea, 
        ctx: &cairo::Context, pieces: &mut Vec<Squares>) {

    let dim_square = DEFAULT_SQUARE_SIZE;

    // tamaño del area del tablero
    let width = DEFAULT_SQUARE_SIZE*8;
    let height = DEFAULT_SQUARE_SIZE*2;
    area.set_size_request(width, height);

    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.rectangle(0.0, 0.0, 
        (width+10) as f64, (height+10) as f64);
    ctx.stroke();

    //white pieces
    for i in 0..8 {
        let pixbuf = set_image(pieces[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);
    //black pieces
    for i in 8..16 {
        let pixbuf = set_image(pieces[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
}


fn board_draw_callback(area: &gtk::DrawingArea, 
        ctx: &cairo::Context) {
    
    let dim_square = DEFAULT_SQUARE_SIZE;

    // tamaño del area del tablero
    let width_height = DEFAULT_SQUARE_SIZE*8;
    area.set_size_request(width_height+10, width_height+10);

    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.rectangle(0.0, 0.0, 
        (width_height+10) as f64, (width_height+10) as f64);
    ctx.stroke();

    let brd: Vec<Squares>;
    unsafe {
        brd = FEN_BOARD.clone().unwrap();
    }

    //1st row
    for i in 0..8 {
        let pixbuf = set_image(brd[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);
    //2nd row
    for i in 8..16 {
        let pixbuf = set_image(brd[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);
    //3rd row
    for i in 16..24 {
        let pixbuf = set_image(brd[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);
    //4th row
    for i in 24..32 {
        let pixbuf = set_image(brd[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);
    //5th row
    for i in 32..40 {
        let pixbuf = set_image(brd[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);
    //6th row
    for i in 40..48 {
        let pixbuf = set_image(brd[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);
    //7th row
    for i in 48..56 {
        let pixbuf = set_image(brd[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);
    //8th row
    for i in 56..64 {
        let pixbuf = set_image(brd[i].clone(), dim_square);
        ctx.set_source_pixbuf(&pixbuf, 5.0, 5.0);
        ctx.paint();
        cairo::Context::translate(ctx, dim_square as f64, 0.0);
    }
    cairo::Context::translate(ctx, (-dim_square * BOARD_SIZE) as f64, dim_square as f64);

}


// ========= funciones auxiliares ================

fn create_board () -> Vec<Squares> {
    
    let mut is_white: bool = true;
    let mut vec_brd: Vec<Squares> = Vec::new();
    
    // first black_side row
    for _i in 0..8 {
        if is_white {
            let node = Squares::new("w".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        else {
            let node = Squares::new("b".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        is_white = !is_white;
    }
    //2nd row
    is_white = false;
    for _i in 0..8 {
        if is_white {
            let node = Squares::new("w".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        else {
            let node = Squares::new("b".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        is_white = !is_white;
    }
    //3rd row
    is_white = true;
    for _i in 0..8 {
        if is_white {
            let node = Squares::new("w".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        else {
            let node = Squares::new("b".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        is_white = !is_white;
    }
    //4th row
    is_white = false;
    for _i in 0..8 {
        if is_white {
            let node = Squares::new("w".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        else {
            let node = Squares::new("b".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        is_white = !is_white;
    }
    //5th row
    is_white = true;
    for _i in 0..8 {
        if is_white {
            let node = Squares::new("w".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        else {
            let node = Squares::new("b".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        is_white = !is_white;
    }
    //6th row
    is_white = false;
    for _i in 0..8 {
        if is_white {
            let node = Squares::new("w".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        else {
            let node = Squares::new("b".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        is_white = !is_white;
    }
    //7th row
    is_white = true;
    for _i in 0..8 {
        if is_white {
            let node = Squares::new("w".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        else {
            let node = Squares::new("b".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        is_white = !is_white;
    }
    //8th row
    is_white = false;
    for _i in 0..8 {
        if is_white {
            let node = Squares::new("w".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        else {
            let node = Squares::new("b".to_string(), "sq".to_string());
            vec_brd.push(node);
        }
        is_white = !is_white;
    }

    vec_brd
}


fn create_pieces() -> Vec<Squares> {
    let mut vec_pieces: Vec<Squares> = Vec::new();

    // background color = white // color = white
    // empty square
    let mut item = Squares::new("w".to_string(), "sq".to_string());
    vec_pieces.push(item);
    // king
    item = Squares::new("w".to_string(), "wk".to_string());
    vec_pieces.push(item);
    // queen
    item = Squares::new("w".to_string(), "wq".to_string());
    vec_pieces.push(item);
    // rook
    item = Squares::new("w".to_string(), "wr".to_string());
    vec_pieces.push(item);
    // bishop
    item = Squares::new("w".to_string(), "wb".to_string());
    vec_pieces.push(item);
    // knight
    item = Squares::new("w".to_string(), "wn".to_string());
    vec_pieces.push(item);
    // pawn
    item = Squares::new("w".to_string(), "wp".to_string());
    vec_pieces.push(item);
    // empty square
    item = Squares::new("w".to_string(), "sq".to_string());
    vec_pieces.push(item);

    // background color = white // color = black
    // empty square
    item = Squares::new("b".to_string(), "sq".to_string());
    vec_pieces.push(item);
    // king
    item = Squares::new("b".to_string(), "bk".to_string());
    vec_pieces.push(item);
    // queen
    item = Squares::new("b".to_string(), "bq".to_string());
    vec_pieces.push(item);
    // rook
    item = Squares::new("b".to_string(), "br".to_string());
    vec_pieces.push(item);
    // bishop
    item = Squares::new("b".to_string(), "bb".to_string());
    vec_pieces.push(item);
    // knight
    item = Squares::new("b".to_string(), "bn".to_string());
    vec_pieces.push(item);
    // pawn
    item = Squares::new("b".to_string(), "bp".to_string());
    vec_pieces.push(item);
    // empty square
    item = Squares::new("b".to_string(), "sq".to_string());
    vec_pieces.push(item);

    vec_pieces
}


fn set_image(item: Squares, dim_square: i32 ) -> gdk_pixbuf::Pixbuf {
    let sq = format!("{}{}{}{}", IMAGES, item.img_name, item.sqcolor, ".gif");
    let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (sq).expect("error al obtener pixbuf");
    let pixbuf = pixbuf1.scale_simple (
            dim_square, dim_square,
            gdk_pixbuf::InterpType::Bilinear
            ).expect("error al escalar pixbuf");
    pixbuf
}


/**********************
 * Right side of paned *
***********************/

fn init_options(vbox: &gtk::Box, board_display: &gtk::DrawingArea) {
    
    // Cargamos el CSS
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSS_MATCH_FEN.as_bytes())
        .expect("Failed to load CSS");
    // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
    // que agregamos se puedan aplicar a nuestra ventana.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    // Table_grid : setting fens
    let table = gtk::Grid::new();
    table.set_row_spacing(5);
    table.set_column_spacing(15);

    let mut fila = 0;
    let lbl_start = gtk::Label::new(Some("Set Start Position"));
    // aqui vamos a cambiar el tamaño y apariencia del texto de la etiqueta
    let attr_list = set_label_attributes();
    lbl_start.set_attributes(Some(&attr_list));
    let btn_start = gtk::Button::with_label("Confirm");
    gtk::WidgetExt::set_widget_name(&btn_start, "btnstart");
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_start, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&btn_start, 1, fila, 1, 1);

    fila = 1;
    let lbl_pos = gtk::Label::new(Some("Set FEN  Position"));
    // aqui vamos a cambiar el tamaño y apariencia del texto de la etiqueta
    let attr_list = set_label_attributes();
    lbl_pos.set_attributes(Some(&attr_list));
    let btn_fen = gtk::Button::with_label("Confirm");
    gtk::WidgetExt::set_widget_name(&btn_fen, "btnfen");
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_pos, true, true, 20);
    let entry_fen = gtk::Entry::new();
    let txt_fen: String;
    unsafe {
        if super::xboard::FEN_ACTUAL.clone().is_some() {
            txt_fen = super::xboard::FEN_ACTUAL.clone().unwrap();
        }
        else { txt_fen = "".to_string(); }
    }
    entry_fen.set_text(&txt_fen);
    gtk::WidgetExt::set_widget_name(&entry_fen, "entryfen");
    entry_fen.set_max_width_chars(50);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&btn_fen, 1, fila, 1, 1);
    table.attach(&entry_fen, 2, fila, 1, 1);
    
    vbox.pack_start(&table, false, false, 20);

    // table of radiobuttons to capture different fens from board

    let radio_clean = gtk::RadioButton::with_label("clean all");    // nota 1
    let radio_pos = gtk::RadioButton::with_label_from_widget(&radio_clean,"Pieces at square");
    let radio_fen = gtk::RadioButton::with_label_from_widget(&radio_clean,"Exact FEN");
    let radio_mat = gtk::RadioButton::with_label_from_widget(&radio_clean,"(endings) positions with this material \nfrom move 30 onwards");
    
    let table1 = gtk::Grid::new();
    table1.set_row_spacing(5);
    table1.set_column_spacing(15);

    let mut fila = 0;
    let lbl_clean = gtk::Label::new(None);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::Start);
    hbox.pack_start(&radio_clean, true, true, 20);
    table1.attach(&hbox, 0, fila, 1, 1);
    table1.attach(&lbl_clean, 1, fila, 1, 1);

    fila = 1;
    let lbl_pos = gtk::Label::new(None);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::Start);
    hbox.pack_start(&radio_pos, true, true, 20);
    table1.attach(&hbox, 0, fila, 1, 1);
    table1.attach(&lbl_pos, 1, fila, 1, 1);

    fila = 2;
    let lbl_fen = gtk::Label::new(None);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::Start);
    hbox.pack_start(&radio_fen, true, true, 20);
    table1.attach(&hbox, 0, fila, 1, 1);
    table1.attach(&lbl_fen, 1, fila, 1, 1);

    fila = 3;
    let lbl_mat = gtk::Label::new(None);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::Start);
    hbox.pack_start(&radio_mat, true, true, 20);
    table1.attach(&hbox, 0, fila, 1, 1);
    table1.attach(&lbl_mat, 1, fila, 1, 1);

    vbox.pack_start(&table1, false, false, 20);


    /* closures radio buttons */
    {
        let lbl_pos_clon = lbl_pos.clone();
        let lbl_fen_clon = lbl_fen.clone();
        let lbl_mat_clon = lbl_mat.clone();
        radio_clean.connect_toggled(move |_w| {
            lbl_pos_clon.set_text("");
            lbl_fen_clon.set_text("");
            lbl_mat_clon.set_text("");
            unsafe {
                DATA_POSITION = None;
            }
        });

        let lbl_pos_clon = lbl_pos.clone();
        let lbl_fen_clon = lbl_fen.clone();
        let lbl_mat_clon = lbl_mat.clone();
        radio_pos.connect_toggled(move |_w| {
            lbl_fen_clon.set_text("");
            lbl_mat_clon.set_text("");
            let mut fen: String = String::from("");
            unsafe {
                let vec_sq = FEN_BOARD.clone().unwrap();
                for i in 0..vec_sq.len() {
                    if i%8 == 0 && fen.len() > 0 {
                        fen.push_str("/");
                    }
                    match vec_sq[i].img_name.as_str() {
                        "sq" => fen.push_str("?"),
                        "wp" => fen.push_str("P"),
                        "wk" => fen.push_str("K"),
                        "wn" => fen.push_str("N"),
                        "wb" => fen.push_str("B"),
                        "wr" => fen.push_str("R"),
                        "wq" => fen.push_str("Q"),
                        "bp" => fen.push_str("p"),
                        "bk" => fen.push_str("k"),
                        "bn" => fen.push_str("n"),
                        "bb" => fen.push_str("b"),
                        "br" => fen.push_str("r"),
                        "bq" => fen.push_str("q"),
                        _ => (),
                    }
                }
                lbl_pos_clon.set_text(&fen);
            }
            
            let tmp = DataPosition::data_position_fen(fen);
            unsafe {
                DATA_POSITION = Some(tmp);
            }
        });

        let lbl_pos_clon = lbl_pos.clone();
        let lbl_mat_clon = lbl_mat.clone();
        let lbl_fen_clon = lbl_fen.clone();
        radio_fen.connect_toggled(move |_w| {
            lbl_mat_clon.set_text("");
            lbl_pos_clon.set_text("");
            let mut fen: String = String::from("");

            let mut tmp: String = create_exact_fen(0_usize, 8_usize);
            fen = format!("{}{}", fen, tmp);

            tmp = create_exact_fen(8_usize, 16_usize);
            fen = format!("{}{}", fen, tmp);

            tmp = create_exact_fen(16_usize, 24_usize);
            fen = format!("{}{}", fen, tmp);

            tmp = create_exact_fen(24_usize, 32_usize);
            fen = format!("{}{}", fen, tmp);

            tmp = create_exact_fen(32_usize, 40_usize);
            fen = format!("{}{}", fen, tmp);

            tmp = create_exact_fen(40_usize, 48_usize);
            fen = format!("{}{}", fen, tmp);

            tmp = create_exact_fen(48_usize, 56_usize);
            fen = format!("{}{}", fen, tmp);

            tmp = create_exact_fen(56_usize, 64_usize);
            fen = format!("{}{}", fen, tmp);

            // kings are mandatory in exact fen
            if !fen.contains("K") {
                lbl_fen_clon.set_text("King piece must be placed");
                unsafe { DATA_POSITION = None; }
                return;
            }
            if !fen.contains("k") {
                lbl_fen_clon.set_text("King piece must be placed");
                unsafe { DATA_POSITION = None; }
                return;
            }
            lbl_fen_clon.set_text(&fen);

            // convert to wchess.square
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
            
            unsafe {
                let tmp = DataPosition::data_exact_fen(fen);
                DATA_POSITION = Some(tmp);
            }
        });

        let lbl_pos_clon = lbl_pos.clone();
        let lbl_mat_clon = lbl_mat.clone();
        let lbl_fen_clon = lbl_fen.clone();
        radio_mat.connect_toggled(move |_w| {
            lbl_fen_clon.set_text("");
            lbl_pos_clon.set_text("");

            let w_kings: usize = 1;
            let b_kings: usize = 1;
            let w_pawns: usize   ;
            let b_pawns: usize   ;
            let w_knights: usize ;
            let b_knights: usize ;
            let w_bishops: usize ;
            let b_bishops: usize ;
            let w_rooks: usize   ;
            let b_rooks: usize   ;
            let w_queens: usize  ;
            let b_queens: usize  ;

            let nr_pieces: Vec<Squares>;
            unsafe{
                nr_pieces = FEN_BOARD.clone().unwrap();
            }

            // now count the quantity of each piece
            w_pawns = nr_pieces.iter().filter(|&n| n.img_name == "wp").count();
            b_pawns = nr_pieces.iter().filter(|&n| n.img_name == "bp").count();
            w_knights = nr_pieces.iter().filter(|&n| n.img_name == "wn").count();
            b_knights = nr_pieces.iter().filter(|&n| n.img_name == "bn").count();
            w_bishops = nr_pieces.iter().filter(|&n| n.img_name == "wb").count();
            b_bishops = nr_pieces.iter().filter(|&n| n.img_name == "bb").count();
            w_rooks = nr_pieces.iter().filter(|&n| n.img_name == "wr").count();
            b_rooks = nr_pieces.iter().filter(|&n| n.img_name == "br").count();
            w_queens = nr_pieces.iter().filter(|&n| n.img_name == "wq").count();
            b_queens = nr_pieces.iter().filter(|&n| n.img_name == "bq").count();

            let txt = format!("K{}+k{}+P{}+p{}+N{}+n{}+B{}+b{}+R{}+r{}+Q{}+q{}",
                w_kings, b_kings, w_pawns, b_pawns,
                w_knights, b_knights, w_bishops, b_bishops,
                w_rooks, b_rooks, w_queens, b_queens
                );

            lbl_mat_clon.set_text(&txt);

            unsafe {
                let tmp = DataPosition::data_material(txt);
                DATA_POSITION = Some(tmp);
            }
        });
    }

    
    /* closures table fen setting*/
    {
        let board_area = board_display.clone();
        btn_start.connect_button_press_event( move |_, _event| {
            set_fen( "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(), &board_area);
            Inhibit(true)
        });
    }
    {
        let board_area = board_display.clone();
        btn_fen.connect_button_press_event( move |_, _event| {
            set_fen( entry_fen.get_text().to_string(), &board_area);
            Inhibit(true)
        });
    }

}


fn set_fen (fen: String, board_display: &gtk::DrawingArea) {
    
    // a quick/light validation of FEN
    let vfen = fen.clone();
    let parts: Vec<&str> = vfen.split(" ").collect();
    
    if parts.len() != 6 { return; }
    if !parts[0].contains("/") { return; }

    let mut board = wchess::new_board();
    wchess::set_fen(&mut board, fen.as_str());
    let arr = wchess::get_board_array(&mut board);
    

    //println!("{:?}", arr);

    let mut local_arr: [i32; 64] = [0;64];
    
    //first row of white
    let mut idx:usize = 56;
    for i in 0..8 {
        local_arr[idx] = arr[i];
        idx += 1;
    }
    //second row of white
    idx = 48;
    for i in 8..16 {
        local_arr[idx] = arr[i];
        idx += 1;
    }
    //3rd row of white
    idx = 40;
    for i in 16..24 {
        local_arr[idx] = arr[i];
        idx += 1;
    }
    //4th row of white
    idx = 32;
    for i in 24..32 {
        local_arr[idx] = arr[i];
        idx += 1;
    }
    //5th row of white
    idx = 24;
    for i in 32..40 {
        local_arr[idx] = arr[i];
        idx += 1;
    }
    //6th row of white
    idx = 16;
    for i in 40..48 {
        local_arr[idx] = arr[i];
        idx += 1;
    }
    //7th row of white
    idx = 8;
    for i in 48..56 {
        local_arr[idx] = arr[i];
        idx += 1;
    }
    //8th row of white
    idx = 0;
    for i in 56..64 {
        local_arr[idx] = arr[i];
        idx += 1;
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

    unsafe {
        let mut vec_boardfen = FEN_BOARD.clone().unwrap();
        for i in 0..64 {
            match local_arr[i] {
                0  => vec_boardfen[i].img_name = "sq".to_string(),
                1  => vec_boardfen[i].img_name = "wp".to_string(),
                2  => vec_boardfen[i].img_name = "wk".to_string(),
                3  => vec_boardfen[i].img_name = "wn".to_string(),
                5  => vec_boardfen[i].img_name = "wb".to_string(),
                6  => vec_boardfen[i].img_name = "wr".to_string(),
                7  => vec_boardfen[i].img_name = "wq".to_string(),
                9  => vec_boardfen[i].img_name = "bp".to_string(),
                10 => vec_boardfen[i].img_name = "bk".to_string(),
                11 => vec_boardfen[i].img_name = "bn".to_string(),
                13 => vec_boardfen[i].img_name = "bb".to_string(),
                14 => vec_boardfen[i].img_name = "br".to_string(),
                15 => vec_boardfen[i].img_name = "bq".to_string(),
                _ => (),
            }
        }
        
        FEN_BOARD = Some(vec_boardfen);
    }
    
    board_display.queue_draw();

}

fn set_label_attributes() -> pango::AttrList {
    let attr_list = pango::AttrList::new();
        
    let mut attr = pango::Attribute::new_foreground(0, 0, 65535)
                            .expect("Couldn't create new background");
    attr.set_start_index(0);
    attr_list.insert(attr);
    
    let mut attr = pango::Attribute::new_scale(1.2)
                            .expect("Couldn't create new scale");
    attr.set_start_index(0);
    attr_list.insert(attr);

    attr_list
}


fn create_exact_fen(from: usize, to: usize) -> String {
    
    let mut fen: String = String::from("");
    let mut empties: i32 = 0;
    let vec_sq: Vec<Squares>;
    unsafe {
        vec_sq = FEN_BOARD.clone().unwrap();
    }

    for i in from..to {
        if vec_sq[i].img_name == "sq" { empties += 1; }
        else if vec_sq[i].img_name != "sq" {
            if empties != 0 {
                fen = format!("{}{}", fen, empties);
                empties = 0;
            }
            if vec_sq[i].img_name == "wp" { fen.push_str("P"); }
            if vec_sq[i].img_name == "wk" { fen.push_str("K"); }
            if vec_sq[i].img_name == "wn" { fen.push_str("N"); }
            if vec_sq[i].img_name == "wb" { fen.push_str("B"); }
            if vec_sq[i].img_name == "wr" { fen.push_str("R"); }
            if vec_sq[i].img_name == "wq" { fen.push_str("Q"); }

            if vec_sq[i].img_name == "bp" { fen.push_str("p"); }
            if vec_sq[i].img_name == "bk" { fen.push_str("k"); }
            if vec_sq[i].img_name == "bn" { fen.push_str("n"); }
            if vec_sq[i].img_name == "bb" { fen.push_str("b"); }
            if vec_sq[i].img_name == "br" { fen.push_str("r"); }
            if vec_sq[i].img_name == "bq" { fen.push_str("q"); }
        }
    }
    if empties != 0 {
        fen = format!("{}{}", fen, empties);
        //empties = 0;
    }
    if to != 64 {
        fen.push_str("/");
    }
    fen
}