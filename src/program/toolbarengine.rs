#![allow(warnings, unused)]


use gtk::*;
use gio::prelude::*;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use super::xboard::{FEN_ACTUAL};
use super::globs::{DEFAULT_POSITION};
use super::wchess;


const BTNS_BLUE: &str = "./icons/btnstoolbar_blue/";
const ENGINE_PATH: &str = "./engine/stockfish.exe";

pub const CSSBAR_ENGINE: &'static str = "
#barengine { 
    background-color: rgba(216,216,216, 0.92);
    /*margin: 1px;*/
    padding: 0.10625rem 0.10625rem 0.10625rem 0.10625rem;
}/*3 3 3 3*/
#lblengine > button:disabled {
    color: #2C3F75;
    /*padding: 1px;
    margin: 1px;*/
}
#enginetext, text{
    background-color: rgba(242,242,242,0.99);
    font-size: 14px;
    font-family: Segoe UI;     /*ChessSansUscf;*/
    padding: 15px;
    color: #08088A;
}
";


#[derive(Clone)]
pub struct Engine {
    pub tbeng : gtk::Toolbar,
    btn_stop:   gtk::ToolButton,
    btn_start:  gtk::ToolButton,
    view: gtk::TextView,
    pub scrolled_view : gtk::ScrolledWindow,
    pub motor: gio::Subprocess,
    fen_to_analyze: String,
    shown_lines: Vec<String>,
}


impl Engine {
    pub fn new() -> Self {
        // Cargamos el CSS
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSSBAR_ENGINE.as_bytes())
            .expect("Failed to load CSS");
        // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
        // que agregamos se puedan aplicar a nuestra ventana.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        let toolbar_eng = gtk::Toolbar::new();
        gtk::WidgetExt::set_widget_name(&toolbar_eng, "barengine");
        let separador = gtk::SeparatorToolItem::new();
        toolbar_eng.insert(&separador, -1);

        let size_btn: i32 = 15;

        // button config engine
        let lbl_engine = gtk::ToolButton::new(None::<&gtk::Image>, Some("Motor de Ajedrez"));
        lbl_engine.set_sensitive(false);
        gtk::WidgetExt::set_widget_name(&lbl_engine, "lblengine");
        gtk::WidgetExt::set_tooltip_markup(&lbl_engine, Some("Max. depth = 20"));
        toolbar_eng.insert(&lbl_engine, -1);

        let separador = gtk::SeparatorToolItem::new();
        toolbar_eng.insert(&separador, -1);

        // button stop engine
        let btn_path = format!("{}{}", BTNS_BLUE, "pause.png");
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
        let btn_stop = gtk::ToolButton::new(Some(&icon), None);
        btn_stop.set_sensitive(false);
        gtk::WidgetExt::set_widget_name(&btn_stop, "cutgame");
        toolbar_eng.insert(&btn_stop, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_stop, Some("Stop engine"));

        // button start engine
        let btn_path = format!("{}{}", BTNS_BLUE, "play.png");
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
        let btn_start = gtk::ToolButton::new(Some(&icon), None);
        gtk::WidgetExt::set_widget_name(&btn_stop, "cutgame");
        toolbar_eng.insert(&btn_start, -1);
        gtk::WidgetExt::set_tooltip_markup(&btn_start, Some("Start engine"));

        // ahora el textview para mostrar los calculos del motor
        // Cree un nuevo búfer y una nueva vista para mostrar el búfer.
        let view = gtk::TextView::new();
        let buffer = view.get_buffer().expect("error");
        // inicializamos/vaciamos el buffer;
        buffer.set_text("");
        
        view.set_wrap_mode(gtk::WrapMode::WordChar);
        view.set_cursor_visible(false);
        view.set_editable(false);
        view.set_left_margin(5);
        view.set_right_margin(5);
        view.set_wrap_mode(gtk::WrapMode::Word);
        gtk::WidgetExt::set_widget_name(&view, "enginetext");

        //view.set_size_request(-1, 300);

        // Set up a scroll window
        let scrolled_win = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_win.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        gtk::WidgetExt::set_widget_name(&scrolled_win, "scrolled");
        scrolled_win.add(&view);

        // The engine pipes
        let os_string = std::ffi::OsString::from(ENGINE_PATH);
        let os_str = std::ffi::OsStr::new(&os_string); // -> &OsStr
        let p = gio::Subprocess::newv(&[os_str], 
                gio::SubprocessFlags::STDIN_PIPE |
                gio::SubprocessFlags::STDOUT_PIPE |
                gio::SubprocessFlags::STDERR_PIPE)
                .expect("error al crear el subproceso Engine");
        let fen_to_analyze = DEFAULT_POSITION.to_string();
        
        let engine = Engine {
            tbeng : toolbar_eng,
            btn_stop : btn_stop,
            btn_start: btn_start,
            view: view,
            scrolled_view: scrolled_win,
            motor: p,
            fen_to_analyze: fen_to_analyze,
            shown_lines: Vec::new(),
        };

        
        // The closures
        let eng = engine.clone();
        engine.btn_start.connect_clicked (move |button| {
            let mut engine1 = eng.clone();
            let fen_actual: String;
            unsafe {
                fen_actual = FEN_ACTUAL.clone().unwrap();
            }
            
            button.set_sensitive(false);
            engine1.btn_stop.set_sensitive(true);
            engine1.fen_to_analyze = fen_actual.clone();
            engine1.set_pos_fen (&fen_actual);
            engine1.go_depth(20);
        });


        let eng = engine.clone();
        engine.btn_stop.connect_clicked (move |button| {
            button.set_sensitive(false);
            eng.btn_start.set_sensitive(true);
            eng.set_stop();
        });

        engine
    }


    pub fn get_handshake (&mut self) {
        let vec_gu8 = self.motor.get_stdout_pipe().unwrap()
            .read_bytes(1024, None::<&gio::Cancellable>).unwrap();
        let uci_response = self.read_left_output_no_moves(vec_gu8);
        let buffer = self.view.get_buffer()
            .expect("error al crear el buffer");
        for elem in uci_response {
          let mut iter = buffer.get_end_iter();
          buffer.insert(&mut iter, &elem);
        }
        self.view.show_all();
    }


    pub fn get_is_ready (&mut self) -> String {
        let mut retorno: String = String::new();
        let s = b"isready\n";
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(s, None::<&gio::Cancellable>).unwrap();
        let vec_gu8 = self.motor.get_stdout_pipe().unwrap()
            .read_bytes(1024, None::<&gio::Cancellable>).unwrap();
        let uci_response = self.read_left_output_no_moves(vec_gu8);
        let buffer = self.view.get_buffer()
            .expect("error al crear el buffer");
        for elem in uci_response {
          let mut iter = buffer.get_end_iter();
          buffer.insert(&mut iter, &elem);
          retorno = elem;
        }
        self.view.show_all();
        retorno
    }
      
      
    pub fn get_uci_options (&mut self) {
        let s = b"uci\n";
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(s, None::<&gio::Cancellable>).unwrap();
        let vec_gu8 = self.motor.get_stdout_pipe().unwrap()
            .read_bytes(4096, None::<&gio::Cancellable>).unwrap();
        let uci_response = self.read_left_output_no_moves(vec_gu8);
        let buffer = self.view.get_buffer()
            .expect("error al crear el buffer");
        for elem in uci_response {
          let mut iter = buffer.get_end_iter();
          buffer.insert(&mut iter, &elem);
        }
        self.view.show_all();
    }
      
      
    pub fn set_options (&mut self, opt: &[u8]) {
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(opt, None::<&gio::Cancellable>).unwrap();
        // here there is no output from engine
    }
      
      
    pub fn set_initial_pos (&mut self) {
        let pos = "position startpos\n".as_bytes();
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(pos, None::<&gio::Cancellable>).unwrap();
        // here there is no output from engine
    }
      
      
    pub fn set_initial_pos_with_moves (&mut self, moves: &str) {
        let p = format!("position startpos moves {}\n", moves);
        let pos = p.clone();
        let pos = pos.as_str().as_bytes();
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(pos, None::<&gio::Cancellable>).unwrap();
        // here there is no output from engine
    }
      
      
    pub fn set_stop (&self) {
        let pos = "stop\n".as_bytes();
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(pos, None::<&gio::Cancellable>).unwrap();
        /*
        let stdin_opt = self.motor.get_stdin_pipe();
        match stdin_opt {
            Some(v_stdin) => {
                let v_cancel = v_stdin.write_all(pos, None::<&gio::Cancellable>);
                if v_cancel.is_ok() {
                    let _i = v_cancel.unwrap();
                }
            },
            None => {},
        }
        */
        // here there is no output from engine
    }


    pub fn set_quit (&self) {
        let pos = "quit\n".as_bytes();
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(pos, None::<&gio::Cancellable>).unwrap();
        //super::utils::alerta(&format!("{:?}", _i.0));
        
        // here there is no output from engine
    }

      
    pub fn set_pos_fen (&mut self, fen: &str) {
        let p = format!("position fen {}\n", fen);
        let pos = p.clone();
        let pos = pos.as_str().as_bytes();
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(pos, None::<&gio::Cancellable>).unwrap();
        // here there is no output from engine
        self.fen_to_analyze = fen.to_string();
    }
      
      
    pub fn go_depth(&mut self, depth: i32) {
        let p = format!("go depth {}\n", depth);
        let pos = p.clone();
        let pos = pos.as_str().as_bytes();
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(pos, None::<&gio::Cancellable>).unwrap();
        self.write_moves_stdout();
    }
      
      
    pub fn go_infinite(&mut self) {
        // clean the textbuffer
        let buf = self.view.get_buffer().unwrap();
        buf.set_text("");

        let pos = b"go infinite\n";
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(pos, None::<&gio::Cancellable>).unwrap();
        
        self.write_moves_stdout();

    }
      
      
      pub fn go_by_time (&mut self, movetime: i32) {
        let p = format!("go movetime {}\n", movetime);
        let pos = p.clone();
        let pos = pos.as_str().as_bytes();
        let _i = self.motor.get_stdin_pipe().unwrap()
            .write_all(pos, None::<&gio::Cancellable>).unwrap();
        self.write_moves_stdout();
      }
      
    fn write_moves_stdout (&mut self) {
        
        loop {
            let fin: bool;
            //let vec_gu8 = self.motor.get_stdout_pipe().unwrap()
            //        .read_bytes(2048, None::<&gio::Cancellable>).unwrap();
            let tmp = self.motor.get_stdout_pipe();
            if tmp.is_some() {
                let tmp1 = tmp.unwrap();
                let tmp2 = tmp1.read_bytes(4096, None::<&gio::Cancellable>);
                if tmp2.is_ok() {
                    let vec_gu8 = tmp2.unwrap(); 
                    fin = self.read_left_output(vec_gu8);
                    //if fin {
                    //    break;
                    //}
                }
                else { break; }
            }
            else {
                break;
            }
            
            //let fin = self.read_left_output(vec_gu8);
            
            if fin {
                break;
            }
            
            /*
            // when depth = infinite
            unsafe {
                let fen_board = FEN_ACTUAL.clone().unwrap();
                println!("355");
                
                if self.fen_to_analyze != fen_board {
                    self.set_stop();
                    self.fen_to_analyze = fen_board.clone();
                    println!("358");
                    self.set_pos_fen(&fen_board);
                    self.go_infinite();
                    println!("360");
                }
            }
            */
        }
    }
      
      
    fn read_left_output_no_moves(&mut self, datos: glib::Bytes) -> Vec<String> {
        let mut s = String::new();
        let vec_u8 = std::ops::Deref::deref(&datos);
        let mut salida: Vec<String> = Vec::new();
        
        for i in 0..vec_u8.len() {
          s.push(vec_u8[i] as char);
          if vec_u8[i] as char == '\n' {
            salida.push(s.clone());
            s.clear();
          }
        }
        salida
    }
      
      
      
    fn read_left_output(&mut self, datos: glib::Bytes) -> bool {
        let mut s = String::new();
        let vec_u8 = std::ops::Deref::deref(&datos);
        let mut fin_analisis = false;
        let buffer = self.view.get_buffer()
                  .expect("error al crear el buffer");
        
        for i in 0..vec_u8.len() {
            // TAB 9
            //windows CR+LF (13+10)
            //linux LF
            if vec_u8[i] == 13 { continue; }
            if vec_u8[i] == 9 { continue; }
            if vec_u8[i] != 10 {
                s.push(vec_u8[i] as char);
            }

            if vec_u8[i] == 10  && s.len() > 0 { //end of line
                
                //sync thread
                let (tx, rx) = mpsc::channel();
                let s_clon = s.clone();

                thread::spawn(move || {
                    let linea = write_pretty (s_clon.trim().to_string());
                    tx.send(linea).unwrap();
                });
                let linea = rx.recv().unwrap();
                if linea.contains("Mate") {
                    self.btn_stop.set_sensitive(false);
                    self.btn_start.set_sensitive(true);
                    self.set_stop();
                    thread::sleep(Duration::from_millis(100));
                }

                if linea.len() >= 2 {
                    
                    self.shown_lines.push(linea.clone());
                    if self.shown_lines.len() > 4 {
                        let _ = self.shown_lines.remove(0);
                    }
                    buffer.set_text("");
                    for i in 0..self.shown_lines.len() {
                        let mut iter = buffer.get_end_iter();
                        buffer.insert(&mut iter, &self.shown_lines[i]);
                    }
                    
                    // refresh screen
                    while gtk::events_pending() {
                        gtk::main_iteration();
                    }

                    if linea.contains("Mejor") || linea.contains("bestmove") {  // || linea.contains("Mate") {
                        fin_analisis = true;
                        self.btn_stop.set_sensitive(false);
                        self.btn_start.set_sensitive(true);
                        self.set_stop();
                        break;
                    }
                }

                s.clear();

            }

        }
        
        fin_analisis
    }
}


fn write_pretty(linea: String) -> String {
    //let mut board = wchess::new_board();
    let mut v_fen: String;
    unsafe {
        v_fen = FEN_ACTUAL.clone().unwrap();
    }
    let arr_fen: Vec<String> = v_fen.split(" ").map(|s| s.to_string()).collect();
    let mut num_mov = 1;
    let mut start_pv = true;
    let mut nr_movs_toprint: i16 = 1;

    let lin_vec: Vec<String> = linea.split(" ").map(|s| s.to_string()).collect();
    if lin_vec[0] == "bestmove".to_string() {
        
        let uci_string = lin_vec[1].clone();

        let mut board = wchess::new_board();
        wchess::set_fen(&mut board, &v_fen);
        let movim = wchess::move_uci(&mut board, &uci_string.trim());

        if movim.movim.move_int == 0 {
            return format!("Mejor jugada:\t1... {}\n", lin_vec[1]);
        }
        
        if arr_fen[1] == "w" {
            return format!("Mejor jugada:\t1. {}\n", movim.san); //&lin_vec[1]);
        }
        else {
            return format!("Mejor jugada:\t1... {}\n", movim.san); //&lin_vec[1]);
        }
        
    }
    else if lin_vec[0] == "info" {
      let mut retorno = String::new();
      let mut movs = false;
      for i in 1..lin_vec.len() {
        if lin_vec[i-1] == "depth" {
            retorno = format!("Depth: {} ", &lin_vec[i]);
        }
        if lin_vec[i-1] == "cp" {
            let eval = lin_vec[i].parse::<f32>().unwrap() / 100.0;
            retorno = format!("{}\tEval.:\t{:>6.2} ", retorno, eval);
        }
        if lin_vec[i-1] == "mate" {
            //let entero = lin_vec[i].parse::<i32>().unwrap();
            retorno = format!("{}\tMate:\t{:>6} ", retorno, lin_vec[i]);
        }
        if lin_vec[i-1] == "pv" {
            retorno = format!("{}\tMoves.: ", retorno);
            //num_mov += 1;
            movs = true;
        }
        if movs {
            let s = &lin_vec[i];
            let uci_string = s.trim();
        
            let mut board = wchess::new_board();
            wchess::set_fen(&mut board, &v_fen);
            let movim = wchess::move_uci(&mut board, &uci_string);
            v_fen = wchess::get_fen(&mut board);

            let arr_v_fen: Vec<String> = v_fen.split(" ").map(|s| s.to_string()).collect();

            num_mov += 1;
            if num_mov % 2 == 0  {
                if arr_v_fen[1] == "w" && start_pv {
                    retorno = format!("*{}{}... {} ", retorno, num_mov/2, movim.san);
                    num_mov += 1;
                }
                else {
                    retorno = format!("{} {}. {} ", retorno, num_mov/2, movim.san);
                }
            }
            else {
                retorno = format!("{} {} ", retorno, movim.san);
            }
            start_pv = false;
            nr_movs_toprint += 1;
            if nr_movs_toprint > 18 {
                retorno = format!("{}\n", retorno);
                return retorno;
            }
        }
      }
      
    }
    // the other possibilities are ommitted
    "".to_string()
  }