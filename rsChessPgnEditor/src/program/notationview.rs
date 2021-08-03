//use std::{thread, time};

use gtk::*;

use regex::Regex;

use super::tags_textbuffer as ttbuf;
use super::globs;
//use super::board_gui::BoardGui;
use super::xboard::{FEN_ACTUAL, NODO_ACTIVO, NODOS};

pub static mut ITERS_FONDO_TAG: Option<(gtk::TextIter, gtk::TextIter)> = None;



const CSS: &'static str = "
#santext, text{
    background-color: rgba(242,242,242,0.99);
    font-size: 18px;
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
// font-family: ChessSansUscf;

#[derive(Clone)]
pub struct NotationView {
    pub view: gtk::TextView,
    pub scrolled: gtk::ScrolledWindow,
}


impl NotationView {
    pub fn new() -> Self {
        // Cargamos el CSS sucede aquí.
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSS.as_bytes())
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
        //view.set_overwrite(true);
        view.set_left_margin(5);
        view.set_right_margin(25);
        view.set_wrap_mode(gtk::WrapMode::Word);

        // Set up a scroll window
        let scrolled_win = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_win.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        gtk::WidgetExt::set_widget_name(&scrolled_win, "scrolled");
        scrolled_win.add(&view);
   
        //view.grab_focus();  // poner el foco despues de empaquetarlo dentro de otro widget
        gtk::WidgetExt::set_widget_name(&view, "santext");

        // creamos los tags de cada jugada
        ttbuf::tags_branchlevel(&mut view);
        //ttbuf::tags_mvnr(&mut view);
        ttbuf::tags_move(&mut view);
        //ttbuf::tags_nag(&mut view);
        //ttbuf::tags_comment(&mut view);

        NotationView {
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
            /*
            let display = gdk::Display::get_default()
                .expect("error en display");
            let gcursor = gdk::Cursor::new_for_display(&display, 
                gdk::CursorType::Arrow);
            let gwindow = gtk::TextViewExt::get_window(view, 
                gtk::TextWindowType::Text)
                    .expect("error en gwindow");
            gdk::WindowExt::set_cursor(&gwindow, Some(&gcursor));
            */
        }
        
    }
}


pub fn continua_si_link (iter: &gtk::TextIter,
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


pub fn select_node_exists (nodes: globs::ListNodes,
        view: &gtk::TextView,
        board_area: &gtk::DrawingArea,
        cur_node: String) {

    let buf = view.get_buffer().unwrap();

    let tags = buf.get_tag_table().unwrap();

    let encontrado = tags.lookup(cur_node.as_str());
    let mut iter_inicio = buf.get_start_iter();

    if encontrado.clone().is_some() {
        let tag = encontrado.unwrap();
        iter_inicio.forward_to_tag_toggle(Some(&tag));
        
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
            FEN_ACTUAL = Some(nodes.nodes[&cur_node].fen.clone());
            NODO_ACTIVO = Some(cur_node);
        }
        board_area.queue_draw();
        
        let start_tag_iter = gtk::TextIter::backward_search(&mut iter_inicio,
            " ",
            gtk::TextSearchFlags::VISIBLE_ONLY,
            None).unwrap();
        
        let end_tag_iter: (gtk::TextIter, gtk::TextIter);
        let end_tag_iter1 = gtk::TextIter::forward_search(&mut iter_inicio,
            " ",
            gtk::TextSearchFlags::VISIBLE_ONLY,
            None);
        if end_tag_iter1.is_none() {
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
    while gtk::events_pending() {
        gtk::main_iteration();
    }
}


pub fn select_node_noexists (nodes: globs::ListNodes,
        view: &gtk::TextView,
        board_area: &gtk::DrawingArea,
        cur_node: String) {

    let buf = view.get_buffer().unwrap();

    let tags = buf.get_tag_table().unwrap();

    let encontrado = tags.lookup(cur_node.as_str());
    let mut iter_inicio = buf.get_start_iter();

    if encontrado.clone().is_some() {
        let tag = encontrado.clone().unwrap();
        iter_inicio.forward_to_tag_toggle(Some(&tag));
        
        // update board_area
        unsafe {
            FEN_ACTUAL = Some(nodes.nodes[&cur_node].fen.clone());
            NODO_ACTIVO = Some(cur_node.clone());
            NODOS = Some(nodes.clone());
        }
        board_area.queue_draw();
        
        let start_tag_iter = gtk::TextIter::backward_search(&mut iter_inicio,
            " ",
            gtk::TextSearchFlags::VISIBLE_ONLY,
            None).unwrap();

        let end_tag_iter: (gtk::TextIter, gtk::TextIter);
        let end_tag_iter1 = gtk::TextIter::forward_search(&mut iter_inicio,
            " ",
            gtk::TextSearchFlags::VISIBLE_ONLY,
            None);
        if end_tag_iter1.is_none() {
            end_tag_iter = (buf.get_end_iter(), buf.get_end_iter());
        }
        else {
            end_tag_iter = end_tag_iter1.unwrap();
        }
        
        let marca = buf.create_mark(Some("dummy"), &start_tag_iter.1, true);
        view.scroll_to_mark(&marca.unwrap(), 0.1, false, 0.0, 0.0);
        
        buf.apply_tag_by_name("selected", &start_tag_iter.1, &end_tag_iter.0);

        unsafe {
            ITERS_FONDO_TAG = Some((start_tag_iter.1, end_tag_iter.clone().0));
        }
    }

    //view.show();
    //while gtk::events_pending() {
    //    gtk::main_iteration();
    //}
}


pub fn reset_buffer(view: &gtk::TextView) {
    let txtbuf = gtk::TextBuffer::new::<gtk::TextTagTable>(None::<&gtk::TextTagTable>);
    view.set_buffer(Some(&txtbuf));
    // creamos los tags estáticos
    ttbuf::tags_branchlevel(&view);
    //ttbuf::tags_mvnr(&view);
    ttbuf::tags_move(&view);
    //ttbuf::tags_nag(&view);
    //ttbuf::tags_comment(&view);
}



pub fn load_gif(view: &gtk::TextView) {

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.set_homogeneous(false);
    let gif_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let img = gtk::Image::from_file("./icons/Loading-bar.gif");
    gif_box.set_homogeneous(true);
    gif_box.pack_start(&img, true, true, 250);

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