
use gtk::*;
//use gdk_pixbuf::Pixbuf;
use gdk::prelude::GdkContextExt;
//use glib::Cast;

use std::collections::HashMap;

use super::pgn as ph;
use super::globs;
use super::wchess;
use super::xboard;
use super::internalxboard;
use super::pieces;

const FILAS: i32 = 8;
const COLUMNAS: i32 = 8;
const COLOR1: (f64, f64, f64) = (166.0 / 255.0, 109.0 / 255.0, 79.0 / 255.0);
const COLOR2: (f64, f64, f64) = (221.0 / 255.0, 184.0 / 255.0, 140.0 / 255.0);
const DIR_PIECES: &str = "./piezas/Merida96/";
const XY_FLIPPED: [[&str; 8]; 8] = [
  ["h8", "g8", "f8", "e8", "d8", "c8", "b8", "a8"],
  ["h7", "g7", "f7", "e7", "d7", "c7", "b7", "a7"],
  ["h6", "g6", "f6", "e6", "d6", "c6", "b6", "a6"],
  ["h5", "g5", "f5", "e5", "d5", "c5", "b5", "a5"],
  ["h4", "g4", "f4", "e4", "d4", "c4", "b4", "a4"],
  ["h3", "g3", "f3", "e3", "d3", "c3", "b3", "a3"],
  ["h2", "g2", "f2", "e2", "d2", "c2", "b2", "a2"],
  ["h1", "g1", "f1", "e1", "d1", "c1", "b1", "a1"]
];

pub static mut DIM_SQUARE: f64 = 45.0;
pub static mut CASILLA_ORIGEN: Option<String> = None;
static mut RATON_X: f64 = 0.0;
static mut RATON_Y: f64 = 0.0;
static mut TURNO: u8 = 0;   // 0 = white to move; 1 = black to move


lazy_static! {
    static ref TABLERO_INVERTIDO: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("a1", "h8"); m.insert("b1", "g8"); m.insert("c1", "f8"); m.insert("d1", "e8");
        m.insert("e1", "d8"); m.insert("f1", "c8"); m.insert("g1", "b8"); m.insert("h1", "a8");
        
        m.insert("a2", "h7"); m.insert("b2", "g7"); m.insert("c2", "f7"); m.insert("d2", "e7");
        m.insert("e2", "d7"); m.insert("f2", "c7"); m.insert("g2", "b7"); m.insert("h2", "a7");
        
        m.insert("a3", "h6"); m.insert("b3", "g6"); m.insert("c3", "f6"); m.insert("d3", "e6");
        m.insert("e3", "d6"); m.insert("f3", "c6"); m.insert("g3", "b6"); m.insert("h3", "a6");
        
        m.insert("a4", "h5"); m.insert("b4", "g5"); m.insert("c4", "f5"); m.insert("d4", "e5");
        m.insert("e4", "d5"); m.insert("f4", "c5"); m.insert("g4", "b5"); m.insert("h4", "a5");
        
        m.insert("a5", "h4"); m.insert("b5", "g4"); m.insert("c5", "f4"); m.insert("d5", "e4");
        m.insert("e5", "d4"); m.insert("f5", "c4"); m.insert("g5", "b4"); m.insert("h5", "a4");
        
        m.insert("a6", "h3"); m.insert("b6", "g3"); m.insert("c6", "f3"); m.insert("d6", "e3");
        m.insert("e6", "d3"); m.insert("f6", "c3"); m.insert("g6", "b3"); m.insert("h6", "a3");
        
        m.insert("a7", "h2"); m.insert("b7", "g2"); m.insert("c7", "f2"); m.insert("d7", "e2");
        m.insert("e7", "d2"); m.insert("f7", "c2"); m.insert("g7", "b2"); m.insert("h7", "a2");
        
        m.insert("a8", "h1"); m.insert("b8", "g1"); m.insert("c8", "f1"); m.insert("d8", "e1");
        m.insert("e8", "d1"); m.insert("f8", "c1"); m.insert("g8", "b1"); m.insert("h8", "a1");
        
        m
    };
}


#[derive(Clone, PartialEq)]
pub struct BoardGui {
    pub area_tablero : gtk::DrawingArea,
    pub listapiezas : HashMap<String,  gdk_pixbuf::Pixbuf>
}


impl BoardGui {
    pub fn new() -> Self {
        let area_tablero = gtk::DrawingArea::new();
        gtk::prelude::WidgetExtManual::add_events(&area_tablero, 
                gdk::EventMask::POINTER_MOTION_MASK | 
                gdk::EventMask::BUTTON_PRESS_MASK | 
                gdk::EventMask::BUTTON_RELEASE_MASK);
        
        // init some globals
        unsafe {
            CASILLA_ORIGEN = Some("999".to_string());
        }
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
                match xboard::FEN_ACTUAL.clone() {
                    Some(vfen) => current_fen = vfen,
                    None => {
                        xboard::FEN_ACTUAL = Some(globs::DEFAULT_POSITION.to_string());
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
            let volteado: bool;
            unsafe {
                volteado = xboard::FLIPPED;
            }
            let tablero_interno = internalxboard::procesa_notacion(grafico, volteado);

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

            // las piezas
            for (xycoord, valor) in &tablero_interno {
                let (x, y) = internalxboard::num_notacion(xycoord);
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
                unsafe {
                    if CASILLA_ORIGEN == Some("999".to_string()) {
                        ctx.paint();
                    }
                    
                    if volteado {
                        if CASILLA_ORIGEN != Some(TABLERO_INVERTIDO.get(xycoord.as_str()).unwrap().to_string()) {
                            ctx.paint();
                        }
                    }
                    else if !volteado {
                        if CASILLA_ORIGEN != Some(xycoord.clone()) {
                            ctx.paint();
                        }
                    }
                }   // end unsafe
            }
            unsafe {
                if CASILLA_ORIGEN != Some("999".to_string()) {
                    let mut pieza_interna = tablero_interno.get(&CASILLA_ORIGEN.clone().unwrap());
                    
                    if volteado {
                        let flipada = TABLERO_INVERTIDO.get(&CASILLA_ORIGEN.clone().unwrap().as_str()).unwrap().to_string();
                        pieza_interna = tablero_interno.get(&flipada);
                    }
                    
                    match pieza_interna {
                    Some(pieza_interna) => {
                        let pieza = list_piec.get(pieza_interna)
                            .expect("error al obtener la pieza");
                        let pixbuf = pieza.scale_simple (
                            (dim_square * 0.90) as i32,
                            (dim_square * 0.90) as i32,
                            gdk_pixbuf::InterpType::Bilinear
                            ).expect("error al escalar pixbuf");
                        //
                        let _sr1 = ctx.set_source_pixbuf(&pixbuf, RATON_X - (dim_square/2.0), 
                            RATON_Y - (dim_square/2.0));
                        ctx.paint();
                    }
                    None => {}
                    };
                }
            }   // end unsafe

            Inhibit(false)
        });

        
        BoardGui {
            area_tablero: area_tablero,
            listapiezas: lista_piezas,
        }
    }
    

}



pub fn buttons_board_area(weak_area: glib::object::WeakRef<gtk::DrawingArea>, 
                    weak_san_viewer: glib::object::WeakRef<gtk::TextView>) {

    // recover the weakref
    let board_area = match weak_area.upgrade() {
        Some(board_area) => board_area,
        None => return,
    };
    
    // **** los closures de los botones del drag-drop en el tablero ******
    {
        board_area.connect_button_press_event ( move |widget, event| {
          on_pieza_presionada(widget, event);
          Inhibit(false)
        });
        
        board_area.connect_motion_notify_event ( move |widget, event| {
          on_pieza_moviendo(widget, event);
          Inhibit(false)
        });
        
        //let weak_visor_partida = visor_partida.downgrade();
        board_area.connect_button_release_event(move |widget, event| {
          let visor_partida = match weak_san_viewer.upgrade() {
            Some(visor_partida) => visor_partida,
            None => return Inhibit(true),
          };
          
          on_pieza_soltada(widget, &visor_partida, event);
          Inhibit(false)
        });
      }
      // ******** fin de los closures del drg-drop del ratón *********
}


/*
  Las funciones de los closures de los botones del ratón
  drag-drop
*/

fn on_pieza_presionada(_widget: &gtk::DrawingArea, 
        event: &gdk::EventButton) {

    let e = event;
    /* Comienza el arrastre de una pieza */
    // para averiguar la casilla de inicio
    //let mut obj_tab = VAR_TABLERO.write().unwrap();
    let col_tamano: f64;
    let fila_tamano: f64;
    let flipped: bool;
    unsafe {
        col_tamano = DIM_SQUARE;
        fila_tamano = DIM_SQUARE;
        flipped = xboard::FLIPPED;
    }
    if e.get_button() == 1 {    // left button
        let (x, y) = event.get_position();

        let seleccionada_columna = (x / col_tamano) as usize;
        let seleccionada_fila = 7 - (y / fila_tamano) as usize;
        
        let pos = internalxboard::alfa_notacion((seleccionada_fila, seleccionada_columna));
        //let pos = pos_string.clone().as_str();

        //averiguamos la pieza y color que esta en tablero[pos] --> pos es la casilla b8, c7, etc.
        //si el turno (w o b) coincide con el color de la pieza (mays, minusculas), iniciamos el movimiento
        //if pos in obj_tab.tablero_interno {
        let mut tablero_interno: HashMap<String, String>;
        unsafe {
            match &internalxboard::INTERNAL_BOARD {
                Some(_) => {},
                None => {
                    let mut b = wchess::new_board();
                    let fen_some = xboard::FEN_ACTUAL.clone();
                    if fen_some.is_some() {
                        let fen_string = fen_some.unwrap();
                        let fen_str: &str = fen_string.as_str().clone();
                        wchess::set_fen(&mut b, fen_str);
                        let grafico = wchess::graphical_board(&mut b);
                        tablero_interno = internalxboard::procesa_notacion(grafico, flipped);
                        //unsafe {
                            internalxboard::INTERNAL_BOARD = Some(tablero_interno);
                        //}
                    }
                },
            }
        }
        unsafe {
            tablero_interno = internalxboard::INTERNAL_BOARD.clone().unwrap();
        }
        if tablero_interno.contains_key(&pos) {
            let es_mayus = tablero_interno.get(&pos).unwrap().as_str().chars().nth(0).unwrap().is_uppercase();
            // turno =0 juegan blancas; mayusculas=true es una pieza blanca
            // turno = 1 juegan negras; mayusculas=false es una pieza negra
            let turno: u8;
            unsafe { turno = TURNO; }
            if (turno == 0 && es_mayus) || (turno == 1 && !es_mayus) {
                if flipped {
                    unsafe {
                        CASILLA_ORIGEN = Some(XY_FLIPPED[seleccionada_fila][seleccionada_columna].to_string());
                    }
                }
                else {
                    unsafe { CASILLA_ORIGEN = Some(pos); }
                }
                unsafe {
                    RATON_X = x;
                    RATON_Y = y;
                }
            }
            else {
                unsafe {
                    CASILLA_ORIGEN = Some("999".to_string());
                    RATON_X = 0.0;
                    RATON_Y = 0.0;
                }
            }
        }
        else {
            unsafe {
                // usamos "999" como casilla inexistente o sin pieza
                CASILLA_ORIGEN = Some("999".to_string());
                RATON_X = 0.0;
                RATON_Y = 0.0;
            }
        }
    }
}


fn on_pieza_moviendo (widget: &gtk::DrawingArea, 
                event: &gdk::EventMotion) {

    //let mut obj_tab = VAR_TABLERO.write().unwrap();
    let is_999: String;
    unsafe {
        is_999 = CASILLA_ORIGEN.clone().unwrap();
    }
    if is_999 == "999".to_string() {
        unsafe {
            RATON_X = 0.0;
            RATON_Y = 0.0;
        }
    }
    else {
        let e = event;
        let (x, y) = gdk::EventMotion::get_position(e); // -> (f64, f64)
        unsafe {
            RATON_X = x;
            RATON_Y = y;
        }
        widget.queue_draw();
    }
}


fn on_pieza_soltada(widget: &gtk::DrawingArea, 
                view: &gtk::TextView,
                event: &gdk::EventButton) {

    /* Final del arrastre de la pieza */
    
    // reseteamos la informacion del arrastre
    let casilla_origen: String;
    let col_tamano: f64;
    let fila_tamano: f64;
    unsafe {
        casilla_origen = CASILLA_ORIGEN.clone().unwrap();
        CASILLA_ORIGEN = Some("999".to_string());
        RATON_X = 0.0;
        RATON_Y = 0.0;
    
        // ahora obtenemos la casilla destino
        col_tamano = DIM_SQUARE;
        fila_tamano = DIM_SQUARE;
    }
    let (x, y) = event.get_position();
    let seleccionada_columna = (x / col_tamano) as usize;
    let seleccionada_fila = 7 - (y / fila_tamano) as usize;

    let casilla_destino: String;
    unsafe {
        if xboard::FLIPPED {
            casilla_destino = XY_FLIPPED[seleccionada_fila][seleccionada_columna].to_string();
        }
        else {
            casilla_destino = internalxboard::alfa_notacion((seleccionada_fila, seleccionada_columna));
        }
    }

    xboard::xb_make_move(widget, view, casilla_origen, casilla_destino);

}




pub fn draw_notation(view: &gtk::TextView , nodes: &mut globs::ListNodes) {
    
    ph::traverse_nodes(view, nodes);
    while gtk::events_pending() {
        gtk::main_iteration();
    }
}