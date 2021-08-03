use regex::Regex;

use gtk::*;
use gtk::prelude::*;

use super::notationview;
use super::board_gui;
use super::xboard;



pub static mut VEC_GAMES: Option<Vec<String>> = None;
pub static mut SELECTED_GAME : Option<String> = None;
pub static mut PROCESSING_FILE: bool = false;

pub const CSSBARSTATUS: &'static str = "
statusbar { 
    background-color: rgba(11,97,11, 0.92);
    color: #ececec;
    /*padding-right: 25px;*/
    padding-left: 100px;
    
    /*padding: 0.20625rem 0.20625rem 0.20625rem 0.20625rem;*/
}
/*
#treeview row:selected {
    border-color: #400;
    border-top-width: 2px;
    border-bottom-width: 2px;
    background: rgba(11,97,11, 0.92);
    color: #000;
}
*/
#treeview.view:selected {
    background: rgba(11,97,11, 0.62);
}
#treeview {
    background-color: rgba(242,242,242,0.99);
}
#treeview.view header {
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
pub struct TreeGames {
    pub scrolled_win: gtk::ScrolledWindow,
    pub model_tgames: gtk::ListStore,
    tview: gtk::TreeView,
    pub stat_bar: gtk::Statusbar,
    selection: gtk::TreeSelection,
}


impl TreeGames {
    pub fn new() -> Self {
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
        // Returns a new context identifier, given a description of the actual context.
        let id1 = stat_bar.get_context_id("Statusbar");
        stat_bar.push(id1, "Games contained in PGN file");

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
        gtk::WidgetExt::set_widget_name(&tview, "treeview");
        scrolled_win.add(&tview);
        let selection = tview.clone().get_selection();

        let tg = TreeGames {
            scrolled_win: scrolled_win,
            model_tgames: model_tgames,
            tview: tview,
            stat_bar: stat_bar,
            selection: selection,
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
            
            let mut attr = pango::Attribute::new_foreground(2048, 33423, 2048)
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
        
        let _tooltips = TreeviewTooltips::init(&tg.tview);
        // poblamos el arbol
        tg.clone().model_tgames.clear();
        tg.clone().display_data();
        
        tg
    }


    pub fn closures_treegame (mut self, 
            weak_san_viewer: glib::object::WeakRef<gtk::TextView>,
            header: super::header::Form) {


        // recover the weakref
        let txtview = match weak_san_viewer.upgrade() {
            Some(txtview) => txtview,
            None => return,
        };
        
        
        self.selection = self.tview.get_selection();
        // https://gtk-rs.org/docs/gtk/trait.TreeSelectionExt.html#tymethod.connect_changed
        self.selection.connect_changed(move |widget| {
            
            if let Some((modelo, iter)) = widget.get_selected() {
                let mut header_clon = header.clone();
                let mut txtview_clon = txtview.clone();
                
                let game_str: String;
                let data_opt: Option<Vec<String>>;
                unsafe {
                    data_opt = VEC_GAMES.clone();
                }

                if data_opt.is_some() {
                    let game_number = modelo.get_value(&iter, 0);

                    // necesitamos traducir estos valores para ser entendibles
                    let mut game_number1 = game_number.get::<u64>().unwrap().unwrap();

                    let data_vec = data_opt.unwrap();

                    //hack
                    if game_number1 >= data_vec.len() as u64 {
                        game_number1 = 0;
                    }
                    game_str = data_vec[game_number1 as usize].clone();
                    unsafe {
                        SELECTED_GAME = Some(game_str.clone());
                        notationview::ITERS_FONDO_TAG = None;
                        board_gui::CASILLA_ORIGEN = Some("999".to_string());
                        xboard::CABECERA = None;
                        xboard::NODOS = None;
                        xboard::FEN_ACTUAL = None;
                        xboard::FLIPPED = false;
                        xboard::NODO_ACTIVO = None;
                    }
                    
                    notationview::reset_buffer(&txtview_clon);
                    super::mainwin::init_game(&mut header_clon, &mut txtview_clon, &game_str, game_number1 as u64);
                    
                }
                else {
                    unsafe {
                        SELECTED_GAME = None;
                    }
                }
            }
            
        });
        
    }


    pub fn display_data(self) {
        unsafe { PROCESSING_FILE = true; }
        // poblamos el arbol
        let data_opt: Option<Vec<String>>;
        unsafe {
            data_opt = VEC_GAMES.clone();
        }
        self.model_tgames.clear();  // if is present another pgn file
        while gtk::events_pending() {
            gtk::main_iteration();
        }

        if data_opt.is_some() {
            let raw_data = data_opt.unwrap();
            // Temporalmente "congela / freeze" la actualización.
            self.tview.freeze_child_notify(); 
            
            for i in 1..raw_data.len() {
                
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
                
                if i%50 == 0 {
                    while gtk::events_pending() {
                        gtk::main_iteration();
                    }
                }
                
            }
            //Reactiva la actualización
            self.tview.thaw_child_notify(); // nota 3
        }
        unsafe { PROCESSING_FILE = false; }
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





const CSS: &'static str = "
#aviso {
    color: #ececec;
    font: 13px Sans;
    font-weight: 700;
    background-color: rgba(50,50,50, 0.92);
    
}
#wintooltip {
    background-color: rgba(50,50,50, 0.92);
    background-image: none;
}
#lbltooltip {
    background-color: rgba(50,50,50, 0.80);
    background-image: none;
    padding: 15px;
}
";

// La estructura para mostrar tooltips en treeview
#[derive(Clone)]
struct TreeviewTooltips {
    tooltip_window: gtk::Window,
}

impl TreeviewTooltips {
    fn init(view: &gtk::TreeView) -> Self {
        // creamos la ventana de tooltip
        let tooltip = TreeviewTooltips { tooltip_window: gtk::Window::new(gtk::WindowType::Popup)};
        gtk::WidgetExt::set_widget_name(&tooltip.tooltip_window, "wintooltip");
        
        let tool_clon = tooltip.clone();
        TreeviewTooltips::maneja_tooltip(tooltip, view);
        tool_clon
    }
    
    fn maneja_tooltip (self, view: &gtk::TreeView) {
        gtk::WidgetExt::set_widget_name(&self.tooltip_window, "gtk-tooltips");
        self.tooltip_window.set_resizable(false);
        self.tooltip_window.set_border_width(4);
        self.tooltip_window.set_app_paintable(true);
        
        self.tooltip_window.connect_draw( move |widget, _contexto| {
            // el contexto es para Cairo
            // para dibujar la caja del tooltip hay que hacerlo con Cairo
            // no es el objetivo de este ejemplo
            // vamos a hacer un CSS
            // Necesitamos nombrarlo para poder aplicar CSS en el widget correspondiente.
            gtk::WidgetExt::set_widget_name(widget, "aviso");
            // 
            let provider = gtk::CssProvider::new();
            provider
                .load_from_data(CSS.as_bytes())
                .expect("Failed to load CSS");
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );
            
            Inhibit(false)
        });
        
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::Fill);
        label.set_use_markup(true);
        label.show();
        self.tooltip_window.add(&label);
        TreeviewTooltips::text_view(self, view);
    }
    
    fn text_view(self, view: &gtk::TreeView) {
        let self_clon = self.clone();
        view.connect_motion_notify_event( move |widget, event| {
            TreeviewTooltips::on_motion (&self_clon, widget, event);
            Inhibit(true)
        });
        let self_clon = self.clone();
        view.connect_leave_notify_event ( move |widget, event| {
            TreeviewTooltips::on_leave (&self_clon, widget, event);
            Inhibit(false)
        });
    }
    
    fn on_motion (&self, widget: &gtk::TreeView, event: &gdk::EventMotion) {
        // esta pieza de codigo ... gracias a:
        // https://stackoverflow.com/questions/24844489/how-to-use-gdk-device-get-position
        let display = gdk::Display::get_default().expect("error al obtener el display");
        let device_manager = gdk::Display::get_device_manager(&display).expect("error al obtener el device_manager");
        let device = gdk::DeviceManager::get_client_pointer (&device_manager).expect("error al obtener el device");
        
        let view_win = gtk::WidgetExt::get_window(widget).expect("error al obtener la gdk:Window");
        // Esto nos permite detectar si el raton está en la región del encabezado.
        let (_op_win, _hor, ver, _m_type) = gdk::WindowExt::get_device_position(&view_win, &device); // -> (Option<Window>, i32, i32, ModifierType)
        
        // ejecutando el programa hasta aqui vemos que la cabecera está por debajo de ver < 25 (en mi sistema)
        // Tener en cuenta que 25 es el estándar para nuestra altura de diseño del encabezado de columna. 
        // Sí, este es un esquema rigido. El valor puede ser diferente en su sistema si usa una fuente diferente.
        if ver<=25 { 
            self.tooltip_window.hide(); // lo escondemos
        } else {
            self.tooltip_window.show();  // lo mostramos
        
            let (x, y)= gdk::EventMotion::get_position(&event);  // -> (f64, f64)
            if let Some(path_pos) = widget.get_path_at_pos(x as i32, y as i32) { // -> Option<(Option<TreePath>, Option(TreeViewColumn)>
                if let Some(path) = path_pos.0 {
                    match path_pos.1 {
                        Some(col) => {
                            let col_title = col.get_title().expect("error en col.get_title");
                            let (ancho, _alto) = self.tooltip_window.get_size_request();  // -> (i32, i32)
                            
                            // configurar la localizacion del tooltip
                            let (x_root, y_root) = gdk::EventMotion::get_root(event);  // -> (f64, f64)
                            self.tooltip_window.move_((x_root - (ancho/2) as f64) as i32, (y_root + 12.0) as i32);
                            
                            let hijos = gtk::ContainerExt::get_children(&self.tooltip_window);  // -> Vec<Widget>
                            let col_clon = col_title.as_str().clone();
                            for i in 0..hijos.len() {
                                let widget = hijos[i].clone();
                                let chisme = widget.downcast::<gtk::Label>().expect("tooltip_window no tiene hijos Label\n\n");
                                gtk::WidgetExt::set_widget_name(&chisme, "lbltooltip");
                                // ahora ya sabemos que es una etiqueta
                                chisme.set_text(&format!("row {} col {}", path, col_clon)); 
                                let txt_game: String;
                                unsafe {
                                    let indx_txt = format!("{}", path);
                                    let indx = indx_txt.parse::<usize>().unwrap();
                                    let vecgame = VEC_GAMES.clone().unwrap();
                                    txt_game = vecgame[indx+1].clone();
                                }
                                let cabecera: Vec<String> = txt_game.split("\n\n").map(|s| s.to_string()).collect();
                                chisme.set_text(&cabecera[0]);
                            }
                            self.tooltip_window.show(); // lo mostramos
                            
                        },
                        None => {},
                    }
                }
                else { 
                    //println!("-- no hay path --");
                }
            }
            else {
                // no hay path_pos
                // Esta línea corrige el error, cuando el raton está fuera por debajo del treeview.
                self.tooltip_window.hide();
            }
        }
    }
    
    fn on_leave(&self, _widget: &gtk::TreeView, _event: &gdk::EventCrossing) {
        self.tooltip_window.hide(); // el raton está fuera del treeview. lo escondemos.
    }
}
