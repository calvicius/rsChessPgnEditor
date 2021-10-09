use gtk::*;

use super::globs;
//use super::board_gui::draw_notation;
use super::notationview;
use super::xboard::{FLIPPED, NODO_ACTIVO, NODOS, FEN_ACTUAL};
use super::levels as lh;
//use super::tags_textbuffer as ttbuf;

//const BTNS_BLACK: &str = "./icons/btnsboard_black/";
//const BTNS_BLUE: &str = "./icons/btnsboard_blue/";
pub const BTNS_BROWN: &str = "./icons/btnsboard_brown/";


pub const CSS: &'static str = "
#flipped {
    font-size: 16px;
    transition: all 200ms ease-out;
    min-width:0px;
    min-height:0px;
    padding: 0px 0px;
    border-width: 0px;
    border-style: hidden;
}
#flipped:hover{
    background-image: none;
    color: black;
    background: #DDB88C;
}
#first {
    font-size: 16px;
    transition: all 200ms ease-out;
    min-width:0px;
    min-height:0px;
    padding: 0px 0px;
    border-width: 0px;
    border-style: hidden;
}
#first:hover{
    background-image: none;
    color: black;
    background: #DDB88C;
}
#prev {
    font-size: 16px;
    transition: all 200ms ease-out;
    min-width:0px;
    min-height:0px;
    padding: 0px 0px;
    border-width: 0px;
    border-style: hidden;
}
#prev:hover{
    background-image: none;
    color: black;
    background: #DDB88C;
}
#next {
    font-size: 16px;
    transition: all 200ms ease-out;
    min-width:0px;
    min-height:0px;
    padding: 0px 0px;
    border-width: 0px;
    border-style: hidden;
}
#next:hover{
    background-image: none;
    color: black;
    background: #DDB88C;
}
#last {
    font-size: 16px;
    transition: all 200ms ease-out;
    min-width:0px;
    min-height:0px;
    padding: 0px 0px;
    border-width: 0px;
    border-style: hidden;
}
#last:hover{
    background-image: none;
    color: black;
    background: #DDB88C;
}
";


#[derive(Clone)]
pub struct ButtonsBoard {
    pub buttonsbar: gtk::Box,
    btn_flip:  gtk::Button,
    btn_first: gtk::Button,
    btn_prev:  gtk::Button,
    btn_next:  gtk::Button,
    btn_last:  gtk::Button,
}


impl ButtonsBoard {
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

        // selected colour of buttons
        // BTNS_BROWN 
        let size_btn: i32 = 35;
        //unsafe { size_btn = (DIM_SQUARE * 0.9) as i32; }
        let action_bar = gtk::Box::new (gtk::Orientation::Horizontal, 5);   //gtk::ActionBar::new();
        
        // botones lado izquierdo
        // button flip
        let button_hbox = gtk::Box::new (gtk::Orientation::Horizontal, 5);
        let mut btn_path = format!("{}{}", BTNS_BROWN, "retweet.png");
        let btn_str = btn_path.as_str();
        let btn_flip = gtk::Button::new();
        btn_flip.set_size_request(size_btn, -1);
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        button_hbox.pack_start(&icon, false, false, 5);
        btn_flip.add(&button_hbox);
        gtk::WidgetExt::set_tooltip_markup(&btn_flip, Some("Flip Board"));
        gtk::WidgetExt::set_widget_name(&btn_flip, "flipped");
        action_bar.pack_start (&btn_flip, false, false, 0);

        // button first move of variation
        let button_hbox = gtk::Box::new (gtk::Orientation::Horizontal, 5);
        btn_path = format!("{}{}", BTNS_BROWN, "fast-backward.png");
        let btn_str = btn_path.as_str();
        let btn_first = gtk::Button::new();
        btn_first.set_size_request(size_btn, -1);
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        button_hbox.pack_start(&icon, false, false, 5);
        btn_first.add(&button_hbox);
        gtk::WidgetExt::set_tooltip_markup(&btn_first, Some("First Move of variation"));
        gtk::WidgetExt::set_widget_name(&btn_first, "first");
        action_bar.pack_start (&btn_first, false, false, 0);

        // button previous move
        let button_hbox = gtk::Box::new (gtk::Orientation::Horizontal, 5);
        btn_path = format!("{}{}", BTNS_BROWN, "angle-double-left.png");
        let btn_str = btn_path.as_str();
        let btn_prev = gtk::Button::new();
        btn_prev.set_size_request(size_btn, -1);
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        button_hbox.pack_start(&icon, false, false, 5);
        btn_prev.add(&button_hbox);
        gtk::WidgetExt::set_tooltip_markup(&btn_prev, Some("Previous Move"));
        gtk::WidgetExt::set_widget_name(&btn_prev, "prev");
        action_bar.pack_start (&btn_prev, false, false, 0);

        // button next move
        let button_hbox = gtk::Box::new (gtk::Orientation::Horizontal, 5);
        btn_path = format!("{}{}", BTNS_BROWN, "angle-double-right.png");
        let btn_str = btn_path.as_str();
        let btn_next = gtk::Button::new();
        btn_next.set_size_request(size_btn, -1);
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        button_hbox.pack_start(&icon, false, false, 5);
        btn_next.add(&button_hbox);
        gtk::WidgetExt::set_tooltip_markup(&btn_next, Some("Next Move"));
        gtk::WidgetExt::set_widget_name(&btn_next, "next");
        action_bar.pack_start (&btn_next, false, false, 0);

        // button last move
        let button_hbox = gtk::Box::new (gtk::Orientation::Horizontal, 5);
        btn_path = format!("{}{}", BTNS_BROWN, "fast-forward.png");
        let btn_str = btn_path.as_str();
        let btn_last = gtk::Button::new();
        btn_last.set_size_request(size_btn, -1);
        let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
            btn_str
            ).expect("error al obtener pixbuf");
        let pixbuf = pixbuf1.scale_simple (
                size_btn,
                size_btn,
                gdk_pixbuf::InterpType::Bilinear
                ).expect("error al escalar pixbuf");
        let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
        button_hbox.pack_start(&icon, false, false, 5);
        btn_last.add(&button_hbox);
        gtk::WidgetExt::set_tooltip_markup(&btn_last, Some("Last Move of variation"));
        gtk::WidgetExt::set_widget_name(&btn_last, "last");
        action_bar.pack_start (&btn_last, false, false, 0);

        ButtonsBoard {
            buttonsbar: action_bar,
            btn_flip: btn_flip,
            btn_first: btn_first,
            btn_prev: btn_prev,
            btn_next: btn_next,
            btn_last: btn_last,
        }

    }


    pub fn btns_board_closures(self, weak_area: glib::object::WeakRef<gtk::DrawingArea>, 
                    weak_san_viewer: glib::object::WeakRef<gtk::TextView>) {
        
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

        let area_clon1 = board_area.clone();
        self.btn_flip.connect_button_press_event ( move |_w, _ev| {
            unsafe {
                FLIPPED = !FLIPPED;
            }
            area_clon1.queue_draw();
            Inhibit(false)
        });

        let area_clon2 = board_area.clone();
        let view_clon2 = txtview.clone();
        self.btn_prev.connect_button_press_event ( move |_w, _ev| {
            
            let nodes: globs::ListNodes;
            let cur_node: String;
            unsafe {
                nodes = NODOS.clone().unwrap();
                cur_node = NODO_ACTIVO.clone().unwrap();
            }
            let prev_node_indx = nodes.nodes[&cur_node].parent_indx.clone();
            
            // first move of variation
            if &cur_node[cur_node.len()-2..] == "1n" {
                //println!("es variante {} - {}", cur_node, &cur_node[0..cur_node.len()-2]);
                // 2z1n es la primera jugada de una variante, 2z1n1z es la segunda jugada
                // 2z1n1n es la primera de una subvariante, 2z1n1n1z es la segunda jugada
                // etc..
                // si se cumple la condición es que ya estamos en la primera jugada de la variante
                Inhibit(false);
            }
            else if prev_node_indx == lh::root_node() {
                Inhibit(false);
            }
            else {
                btns_select_node(nodes, &view_clon2, &area_clon2, prev_node_indx);
            }

            Inhibit(true)
        });


        let area_clon3 = board_area.clone();
        let view_clon3 = txtview.clone();
        self.btn_next.connect_button_press_event ( move |_w, _ev| {
            
            let mut nodes: globs::ListNodes;
            let cur_node: String;
            unsafe {
                nodes = NODOS.clone().unwrap();
                cur_node = NODO_ACTIVO.clone().unwrap();
            }
            
            let next_node_indx = lh::get_next_mainline_indx(cur_node.clone());
            if !nodes.node_exists(next_node_indx.clone()) {
                Inhibit(false);
            }
            else if nodes.nodes[&cur_node.clone()].children.len() == 1 {
                // only one choice
                btns_select_node(nodes, &view_clon3, &area_clon3, next_node_indx);
            }
            else {
                let new_next = nodes.nodes[&cur_node.clone()].children[0].clone();
                btns_select_node(nodes, &view_clon3, &area_clon3, new_next);
            }

            Inhibit(true)
        });


        let area_clon4 = board_area.clone();
        let view_clon4 = txtview.clone();
        self.btn_first.connect_button_press_event ( move |_w, _ev| {
            
            let nodes: globs::ListNodes;
            let mut cur_node: String;
            unsafe {
                nodes = NODOS.clone().unwrap();
                cur_node = NODO_ACTIVO.clone().unwrap();
            }
            //let prev_node_indx = nodes.nodes[&cur_node].parent_indx.clone();
            // we are into main line
            let mut first_node_indx: String;
            if &cur_node[cur_node.len()-1..] == "z" && !cur_node.contains("n") {
                cur_node = "1z".to_string();
            }
            // maybe we are in a variation. eg: 72z1n1z
            while cur_node.len() > 2 && cur_node.contains("n") {
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
            
            btns_select_node(nodes, &view_clon4, &area_clon4, cur_node);

            Inhibit(true)
        });


        let area_clon5 = board_area.clone();
        let view_clon5 = txtview.clone();
        self.btn_last.connect_button_press_event ( move |_w, _ev| {
            
            let mut nodes: globs::ListNodes;
            let mut cur_node: String;
            unsafe {
                nodes = NODOS.clone().unwrap();
                cur_node = NODO_ACTIVO.clone().unwrap();
            }

            let mut next_node_indx = lh::get_next_mainline_indx(cur_node.clone());
            
            while nodes.node_exists(next_node_indx.clone()) {
                if nodes.nodes[&next_node_indx.clone()].children.len() == 1 {
                    next_node_indx = lh::get_next_mainline_indx(next_node_indx.clone());
                    cur_node = next_node_indx.clone();
                    continue;
                }
                else {
                    if nodes.nodes[&next_node_indx.clone()].children.len() < 1 { break; }
                    next_node_indx = nodes.nodes[&next_node_indx.clone()].children[0].clone();
                    cur_node = next_node_indx.clone();
                }
            }

            btns_select_node(nodes, &view_clon5, &area_clon5, cur_node);
            
            Inhibit(true)
        });
    }
}


pub fn btns_select_node (nodes: globs::ListNodes,
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
            let iters = notationview::ITERS_FONDO_TAG.clone();
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

        let marca = buf.create_mark(Some("dummy"), &start_tag_iter.1, true);
        view.scroll_to_mark(&marca.unwrap(), 0.1, false, 0.0, 0.0);

        buf.apply_tag_by_name("selected", &start_tag_iter.1, &end_tag_iter.0);
        unsafe {
            notationview::ITERS_FONDO_TAG = Some((start_tag_iter.1, end_tag_iter.clone().0));
        }

    }
}


pub fn btns_delete_variation (nodes: globs::ListNodes,
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
        if end_tag_iter1. is_none() {
            end_tag_iter = (buf.get_end_iter(), buf.get_end_iter());
        }
        else {
            end_tag_iter = end_tag_iter1.unwrap();
        }
        
        let marca = buf.create_mark(Some("dummy"), &start_tag_iter.1, true);
        view.scroll_to_mark(&marca.unwrap(), 0.1, false, 0.0, 0.0);
        
        buf.apply_tag_by_name("selected", &start_tag_iter.1, &end_tag_iter.0);

        unsafe {
            notationview::ITERS_FONDO_TAG = Some((start_tag_iter.1, end_tag_iter.clone().0));
        }
        
        view.show_all();
        while gtk::events_pending() {
            gtk::main_iteration();
        }
        
    }

    view.show();
    while gtk::events_pending() {
        gtk::main_iteration();
    }
}

