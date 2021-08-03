use gtk::*;
//use gtk::prelude;
use gdk::prelude::*;
//use glib::*;

//use std::time::{Duration, Instant};
//use std::time;
//use std::thread;
use std::process;


use super::globs;
use super::xboard;
use super::board_gui as b_gui;
use super::notationview;
use super::header;
use super::buttonsboard;
use super::toolbargame;
use super::toolbarengine;
use super::treegames;
use super::utils;

pub const CSSPANED: &'static str = "
#panedgame > separator { 

    min-width: 0px; 
    min-height: 0px;
    border-style: none;
    background-size: 0px 0px;
}
#panedgame > separator.wide { 
    min-width: 0px; 
    min-height: 0px; 
    background-color: #F2F2F2;
}

#paneltree {
    background-color: rgba(242,242,242,0.99);
    border: solid 1px;
    margin: 2px;
}

#paneltree > separator {
    background-color: rgba(50,50,50, 0.92);
    margin: 0px;
    padding: 0px;
}
/*
#paneltree > separator.wide {
    min-width: 1px; 
    min-height: 1px;
    
    background-size: 1px 1px;
}
*/
#winhighest {
    /*border-radius:0.4125rem ; */
    margin: 0.275rem;
    padding: 0.275rem 0;
}

";



pub struct App {
    pub window: gtk::Window,
}


impl App {
    // el constructor de la Aplicacion
    pub fn new() -> Self {
        
        // Cargamos el CSS
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSSPANED.as_bytes())
            .expect("Failed to load CSS");
        // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
        // que agregamos se puedan aplicar a nuestra ventana.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        let game: &str = globs::EMPTY_GAME;

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        //window.set_size_request(400, 740);
        window.set_title("Chess PGN game Editor");
        gtk::WidgetExt::set_widget_name(&window, "winhighest");

/*
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
*/
        let vbox_window = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // el panel izquierdo del treeview

        let vbox_engine = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let panel_treegames = gtk::Paned::new(Orientation::Horizontal);
        gtk::Paned::set_size_request (&panel_treegames, 1680, 750);
        panel_treegames.set_position(480);
        gtk::WidgetExt::set_widget_name(&panel_treegames, "paneltree");

        let vbox_tree = gtk::Box::new(Orientation::Vertical, 0);
        gtk::WidgetExt::set_widget_name(&vbox_tree, "vboxtree");
        let tree_games = treegames::TreeGames::new();
        vbox_tree.pack_start(&tree_games.stat_bar, false, true, 0);
        vbox_tree.pack_start(&tree_games.scrolled_win, true, true, 0);

        panel_treegames.add1(&vbox_tree);


        let panel = gtk::Paned::new(Orientation::Horizontal);
        let panel_game_engine = gtk::Paned::new(Orientation::Vertical);
        gtk::Paned::set_size_request (&panel_game_engine, 1200, 750);     //width, heoght
        panel_game_engine.set_position(580);
        //panel_game_engine.add1(&vbox_window);
        panel_game_engine.pack1(&vbox_window, false, false);
        panel_game_engine.add2(&vbox_engine);

        let bargame = toolbargame::BarGame::new();
        vbox_window.pack_start(&bargame.tb, false, false, 0);

        //let panel = gtk::Paned::new(Orientation::Horizontal);
        gtk::Paned::set_size_request (&panel, 1200, 520);     //width, heoght
        gtk::WidgetExt::set_widget_name(&panel, "panedgame");
        panel.set_position(480); // la posicion del divisor de los paneles
        vbox_window.pack_start(&panel, true, true, 0);

        panel_treegames.add2(&panel_game_engine);
        window.add(&panel_treegames);

        /*
         ---- la parte del motor de ajedrez
        */
        
        let engine = toolbarengine::Engine::new();
        vbox_engine.pack_start(&engine.tbeng, false, true, 0);
        vbox_engine.pack_start(&engine.scrolled_view, true, true, 0);

        /*
         ---- la parte del tablero a la izquierda ----
        */
        let vbox_left = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let tablero = b_gui::BoardGui::new();
        let frm = gtk::AspectFrame::new(None, 0.5, 0.1, 5.0, true);
        frm.add(&tablero.area_tablero);

        let vbox_tablero = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        vbox_tablero.set_hexpand(true);
        vbox_tablero.pack_start(&frm, true, true, 10);
        vbox_left.pack_start(&vbox_tablero, true, true, 5);

        // los botones del tablero
        let btns_brd = buttonsboard::ButtonsBoard::new();
        gtk::WidgetExt::set_halign (&btns_brd.buttonsbar, gtk::Align::Center);
        //gtk::WidgetExt::set_valign (&btns_brd.buttonsbar, gtk::Align::Center);
        vbox_left.pack_start(&btns_brd.buttonsbar, false, true, 15);

        panel.add1(&vbox_left);

        // ---- la parte de la partida a la derecha
        let vbox_game = gtk::Box::new(gtk::Orientation::Vertical, 0);
        // añadimos el box de la partida a la derecha del panel
        panel.add2(&vbox_game);

        let mut form = header::Form::new();
        vbox_game.pack_start(&form.event_header, false, true, 10);
        window.show_all();
        while gtk::events_pending() {
            gtk::main_iteration();
        }

        let view_struct = notationview::NotationView::new();
        let mut view = view_struct.clone().view;
        let scrolled_win = view_struct.clone().scrolled;
        vbox_game.pack_start(&scrolled_win, true, true, 10);

        // dibujamos un gif animado de espera
        //notationview::load_gif(&view);
        //view.show_all();
        
        window.show_all();

        // necesitaremos estas referencias debiles mas abajo
        // antes de que sean prestadas (borrowed)
        let ba = tablero.area_tablero.downgrade();  // -> glib::object::WeakRef<gtk::DrawingArea>
        let vw = view_struct.view.downgrade();      // -> glib::object::WeakRef<gtk::TextView>
        //let fh = form.event_header.downgrade();

        //llevamos la partida a pantalla
        init_game(&mut form, &mut view, game, 0_u64);
        view_struct.clone().view_closures(tablero.area_tablero);
        btns_brd.clone().btns_board_closures(ba.clone(), vw.clone());
        bargame.btns_bargame_closures(ba.clone(), vw.clone(), form.clone(), tree_games.clone(), &engine);
        tree_games.closures_treegame(vw.clone(), form);

        
        window.show_all();
        while gtk::events_pending() {
            gtk::main_iteration();
        }
        
        // algunos closures de clicks externos a sus structs
        b_gui::buttons_board_area(ba, vw);

        // exit application
        window.connect_delete_event(move |_, _| {
            //engine.set_stop();
            //engine.set_quit();
            //let one_secs = time::Duration::from_millis(500);
            //thread::sleep(one_secs);
            //engine.motor.force_exit();
            if gtk::events_pending() {
                engine.set_quit();
                engine.motor.force_exit();
                utils::alerta("Hay eventos pendientes.\n¿Motor activo?.\n¿Fichero PGN todavia procesando?");
                process::exit(0);
                //Inhibit(true)
            }
            else {
                engine.motor.force_exit();
                main_quit();
                Inhibit(false)
            }
        });

        App {
            window: window,
        }
    }
}


use std::time::{Duration, Instant};
pub fn init_game(form: &mut header::Form, view: &mut gtk::TextView, game: &str, game_nr: u64) {   //-> globs::ListNodes {
    // dibujamos un gif animado de espera
    notationview::load_gif(&view);
    view.show_all();

    //let partida_vacia = globs::EMPTY_GAME.to_string();
    //let game_info = globs::GameInfo::new();
    let mut db_entry = globs::DbEntry::new();

    let start = Instant::now();
    // set an empty game for future DB
    db_entry.id = game_nr;    // this game number will be changed with db record numbar
    //db_entry.game_info = game_info.clone(); // info partida vacia;
    db_entry.pgn = game.to_string();

    let mut rv: globs::ListNodes = xboard::read_nodes_from_file(&mut db_entry);

    form.modif_lbl_header();
    b_gui::draw_notation(view, &mut rv);

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    
}

