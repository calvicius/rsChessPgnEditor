use gtk::{self, prelude::*};

pub mod getbook;
pub mod keys;
use crate::program::wchess;

const CSSBOOKMOVES: &str = "
#top_area {
    background-color: rgba(44, 63, 117, 0.75);
}
";

pub fn init_search( path: &str, fen: &str) {
    // Cargamos el CSS sucede aquí.
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSSBOOKMOVES.as_bytes())
        .expect("Failed to load CSS");
    // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
    // que agregamos se puedan aplicar a nuestra ventana.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    let dialog = gtk::Dialog::with_buttons(
            Some("Book moves"),
            None::<&gtk::Window>,   // es el parent
            gtk::DialogFlags::MODAL,
            &[] // no le añadimos botones
    );
    dialog.set_size_request(200, 50);
    dialog.set_position(gtk::WindowPosition::Mouse);
    let top_area = dialog.get_content_area();
    WidgetExt::set_widget_name(&top_area, "top_area");
    let view = gtk::TextView::new();
    view.set_editable(false);
    view.set_cursor_visible(false);
    //view.set_top_margin(10);
    //view.set_bottom_margin(10);
    view.set_left_margin(20);
    view.set_right_margin(20);
    view.set_pixels_above_lines(6);
    top_area.pack_start(&view, true, true, 10);

    let buffer = view.get_buffer().unwrap();
    let tabla_tags = buffer.get_tag_table().expect("error al obtener la tabla de tags");
    let tag_figurine = gtk::TextTag::new(Some("figurine"));
    tag_figurine.set_property_font(Some("ChessSansUscf 10"));
    let mut _x = tabla_tags.add(&tag_figurine);
    buffer.set_text("");

    dialog.show_all();

    let mut moves: Vec<getbook::PolyglotEntry> = getbook::init_reader(path, fen);
    
    // order vector of moves by weight
    moves.sort_by(|a, b| b.weight.cmp(&a.weight));

    let mut total_moves = 0;
    for entry in moves.clone() {
        total_moves += entry.weight;
    }

    let mut board = wchess::new_board();
    
    for entry in moves.clone() {
        let v_fen = fen.clone();
        wchess::set_fen(&mut board, v_fen);
        let mut uci_move = getbook::get_uci_move(entry.mv);

        // polyglot castle is chess960 style
        if uci_move == "e1h1" && board.square[wchess::globs::E1] == wchess::globs::WHITE_KING as i32 {
            uci_move = "e1g1".to_string();
        }
        if uci_move == "e1a1" && board.square[wchess::globs::E1] == wchess::globs::WHITE_KING as i32 {
            uci_move = "e1c1".to_string();
        }
        if uci_move == "e8h8" && board.square[wchess::globs::E8] == wchess::globs::BLACK_KING as i32 {
            uci_move = "e8g8".to_string();
        }
        if uci_move == "e8a8" && board.square[wchess::globs::E8] == wchess::globs::BLACK_KING as i32 {
            uci_move = "e8c8".to_string();
        }

        // make the move
        let internal_move = wchess::move_uci(&mut board, &uci_move);
        let san_move = internal_move.san;

        //println!("move = {} - weight = {}", uci_move, entry.weight);
        let percentage: f32 = (entry.weight as f32 * 100.0) / total_moves as f32; 
        let txt_linea = format!("{:<14}\t{:.2} %\n", san_move, percentage);

        let mut start_iter = buffer.get_end_iter();
        let marca1 = buffer.create_mark(Some("marca1"), &start_iter, true)
                        .expect("error al crear marca");
        
        buffer.insert(&mut start_iter, &txt_linea );

        let end_iter = buffer.get_end_iter();
        let marca2 = buffer.create_mark(Some("marca2"), &end_iter, true)
                        .expect("error al crear marca");
        let start_iter = buffer.get_iter_at_mark(&marca1);
        let end_iter = buffer.get_iter_at_mark(&marca2);
        buffer.apply_tag_by_name("figurine", &start_iter, &end_iter);
    }

    dialog.run();
    dialog.close();
}
