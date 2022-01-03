/*
auxiliary functions
*/
//use std::env;
use std::fs::File;
use std::io::Write;
//use std::sync::mpsc::{Sender, Receiver};
//use std::sync::mpsc;
//use std::thread;

use gtk;
use gtk::prelude::*;

use super::globs;
use super::pgn as ph;
use super::xboard::{NODOS, CABECERA};

pub const ICONS_BLUE: &str = "./icons/btnsboard_blue/";
const PIECES_CHESS: &str = "./piezas/Merida96/";
pub static mut PROMOTED_PIECE: &str = "q";


pub const CSSDIALOG: &'static str = "
#dialalert .dialog-action-area { 
    /*margin: 0.825rem;*/
    padding: 10px;
}

#dialalert .dialog-action-area button {
    background-color: #333333;
    border-color: #333333;
    border-bottom-color: #333333;
    background-image: none;
    outline-color: rgba(44, 63, 117, 0.3);
    color: #ececec;
    box-shadow: none;
}

#dialsavepgn .dialog-action-area { 
    /*margin: 0.825rem;*/
    padding: 10px;
}

#dialsavepgn .dialog-action-area button {
    background-color: #333333;
    border-color: #333333;
    border-bottom-color: #333333;
    background-image: none;
    outline-color: rgba(44, 63, 117, 0.3);
    color: #ececec;
    box-shadow: none;
}
";

pub const CSSPOPUP: &'static str = "
#winpopup {
    border: 1px solid #2C3F75; 
    border-radius: 3px;
    border-color: #2C3F75;
    /*border-width: 10px;*/
    /*outline-style: solid;*/
    /*background-color: #2C3F75;*/
}
";

#[derive(Clone)]
pub struct PopUpWindow {
    pub win_popup: gtk::Window,
}


impl PopUpWindow {
    pub fn init(etiq: &str) -> Self {
        // Cargamos el CSS
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSSPOPUP.as_bytes())
            .expect("Failed to load CSS");
        // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
        // que agregamos se puedan aplicar a nuestra ventana.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        let win_popup = gtk::Window::new(gtk::WindowType::Popup);
        win_popup.set_size_request(250, 140);
        win_popup.set_position(gtk::WindowPosition::CenterAlways);
        win_popup.set_border_width(10);
        gtk::WidgetExt::set_widget_name(&win_popup, "winpopup");

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 40);
        win_popup.add(&vbox);

        let pb = gtk::ProgressBar::new();
        pb.set_text(Some(etiq));
        pb.set_show_text(true);
        pb.pulse();
        vbox.pack_start(&pb, false, false, 15);

        let tick = move || {
            pb.pulse();
            glib::Continue(true)
        };
        
        glib::timeout_add_local(50, tick);  // 50 milsegundos

        win_popup.show_all();

        PopUpWindow {
            win_popup
        }
    }

    pub fn destruye (self) {
        self.win_popup.close();
    }
}

// Alert dialog for some error
pub fn alerta(msg: &str) { 
    // Cargamos el CSS
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSSDIALOG.as_bytes())
        .expect("Failed to load CSS");
    // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
    // que agregamos se puedan aplicar a nuestra ventana.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    let dialog = gtk::Dialog::with_buttons(
                    Some("Alerta"),
                    None::<&gtk::Window>,   // es el parent
                    gtk::DialogFlags::MODAL,
                    &[(&"OK", gtk::ResponseType::Ok)]
                );
    
    gtk::WidgetExt::set_widget_name(&dialog, "dialalert");
    dialog.set_position(gtk::WindowPosition::CenterAlways);
    let top_area = dialog.get_content_area(); // -> Box
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 20);
    top_area.pack_start(&hbox, false, true, 20);

    let icon_size = 60;

    let icon_path = format!("{}{}", ICONS_BLUE, "ban.png");
    let icon_str = icon_path.as_str();
    let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
        icon_str).expect("error al obtener pixbuf");
    let pixbuf = pixbuf1.scale_simple (
            icon_size, icon_size,
            gdk_pixbuf::InterpType::Bilinear).expect("error al escalar pixbuf");
    let icon = gtk::Image::from_pixbuf(Some(&pixbuf));
    
    //let stock = gtk::Image::from_icon_name(Some("dialog-warning-symbolic"), gtk::IconSize::Dialog);
    hbox.pack_start(&icon, true, true, 10);
    
    let lbl_msg = gtk::Label::new(Some(msg));
    
    // ponemos el tipo de letra de la etiqueta y otras caracteristicas
    // creamos una lista de atributos para la etiqueta
    let attr_list = pango::AttrList::new();
    let mut attr = pango::Attribute::new_family("Courier").unwrap();  // -> Option<Attribute>
    attr.set_start_index(0);    // empieza en el primer caracter
    attr_list.insert(attr);
    // ahora el tamaño de la letra
    let mut attr = pango::Attribute::new_scale(1.20).unwrap();  // -> Option<Attribute>
    attr.set_start_index(0); 
    attr_list.insert(attr);
    // ahora el color del texto
    let mut attr = pango::Attribute::new_foreground(0, 0, 65535).expect("error en foreground");    // -> Option<Attribute>
    attr.set_start_index(0);    // empieza en el primer caracter
    attr_list.insert(attr);
    // actualizamos los atributos a la etiqueta
    lbl_msg.set_attributes(Some(&attr_list));
    // definimos el tamaño de la etiqueta
    lbl_msg.set_size_request(-1, 40);    // width, height; -1 = unset
    

    hbox.pack_start(&lbl_msg, true, true, 30);
    
    dialog.show_all();
    dialog.run();
    //unsafe { dialog.destroy(); }
    dialog.close();
}



pub fn chooser_savepgn (){
    // Cargamos el CSS
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSSDIALOG.as_bytes())
        .expect("Failed to load CSS");
    // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
    // que agregamos se puedan aplicar a nuestra ventana.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    let dialog = gtk::FileChooserDialog::with_buttons(
                    Some("Abrir Fichero"),
                    None::<&gtk::Window>,   // es el parent
                    gtk::FileChooserAction::Save,
                    &[(&"OK", gtk::ResponseType::Ok), (&"Cancel <Escape>", gtk::ResponseType::Cancel)]
                );
    dialog.set_default_size(500, 400);
    gtk::WidgetExt::set_widget_name(&dialog, "dialsavepgn");

    dialog.connect_key_press_event( move |widget, event_key| {
        if gdk::EventKey::get_keyval(event_key) == gdk::keys::constants::Escape {   //gdk::enums::key::Escape {
            widget.close();
            Inhibit(true);
        }
        if gdk::EventKey::get_keyval(event_key) == gdk::keys::constants::Return {   //gdk::enums::key::Return {
            let resultado: gtk::ResponseType = gtk::ResponseType::Ok.into();
            
            if resultado == gtk::ResponseType::Ok {
                //println!("74 {:#?}", resultado); It must b Ok
                Inhibit(true);
            }
            else {
                Inhibit(false);
            }
        }
        Inhibit(false)
    });
    
    dialog.show_all();
    let result = dialog.run();
    

    let headerinfo: globs::GameInfo;
    let nodes: globs::ListNodes;
    unsafe {
        headerinfo = CABECERA.clone().unwrap();
        nodes = NODOS.clone().unwrap();
    }
    
    //let popup = PopUpWindow::init("Working ...");
    //let pgn_game = ph::create_pgn(nodes, headerinfo);

    //popup.clone().destruye();
    let popup: PopUpWindow;

    if result == gtk::ResponseType::Ok.into() {
        popup = PopUpWindow::init("Working ...");
        let pgn_game = ph::create_pgn(nodes, headerinfo);

        popup.clone().destruye();

        let selected_pathbuf = dialog.get_filename().expect("error al seleccionar fichero");   // -> std::pathbuf
        let selected_file = selected_pathbuf.into_os_string().into_string().unwrap();
        
        // This creates the file if it does not exist (and empty the file if it exists).
        let mut file = File::create(&selected_file).unwrap();
        write!(&mut file, "{}", &pgn_game).unwrap();

    }
    else {
        //popup.destruye();
        alerta("File no selected");
    }
    
    dialog.close();
}



pub const CSSPGN: &'static str = "
#dialpgn .dialog-action-area { 
    
    padding: 10px;
}
#dialpgn .dialog-action-area button {
    background-color: #333333;
    border-color: #333333;
    border-bottom-color: #333333;
    background-image: none;
    outline-color: rgba(44, 63, 117, 0.3);
    color: #ececec;
    box-shadow: none;
}
";


pub fn pgn_select() -> Option<String> {
    // Cargamos el CSS
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSSPGN.as_bytes())
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    let dialog = gtk::FileChooserDialog::with_buttons(
                    Some("Abrir Fichero"),
                    None::<&gtk::Window>,   // es el parent
                    gtk::FileChooserAction::Open,
                    &[(&"OK", gtk::ResponseType::Ok), (&"Cancel", gtk::ResponseType::Cancel)]
                );
    dialog.set_current_folder("./pgns");
    gtk::WidgetExt::set_widget_name(&dialog, "dialpgn");
    dialog.show_all();
    let result = dialog.run();
    

    if result == gtk::ResponseType::Ok.into() {
        let selected_pathbuf = dialog.get_filename().expect("error al seleccionar fichero");   // -> std::pathbuf
        let selected_file = selected_pathbuf.into_os_string().into_string().unwrap();
        unsafe {
            dialog.destroy();
        }
        return Some(selected_file);
    }
    else { 
        unsafe {
            dialog.destroy();
        }
        return None; }
    
    
}


pub const CSSPROMOTE: &'static str = "
/*
#barpromo .inline-toolbar toolbutton > button:active { 
    background-image: none;
    background-color: #414141;
    border-color: #414141;
    box-shadow: none;
}
*/
#dialogpromo .dialog-action-area { 
    
    padding: 10px;
}
#dialogpromo .dialog-action-area button {
    background-color: #333333;
    border-color: #333333;
    border-bottom-color: #333333;
    background-image: none;
    outline-color: rgba(44, 63, 117, 0.3);
    color: #ececec;
    box-shadow: none;
}
";


pub fn popup_promotion() -> String {
    // Cargamos el CSS
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSSPROMOTE.as_bytes())
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    let dialog = gtk::Dialog::with_buttons(
        Some("Pawn promotion"),
        None::<&gtk::Window>,   // es el parent
        gtk::DialogFlags::MODAL,
        &[(&"OK", gtk::ResponseType::Ok)]
    );

    dialog.set_position(gtk::WindowPosition::CenterAlways);
    gtk::WidgetExt::set_widget_name(&dialog, "dialogpromo");
    
    let top_area = dialog.get_content_area();
    
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 20);
    top_area.pack_start(&hbox, false, true, 20);

    let toolbar = gtk::Toolbar::new();
    gtk::WidgetExt::set_widget_name(&toolbar, "barpromo");
    hbox.pack_start(&toolbar, true, true, 10);
    

    let size_btn = 50;

    // button queen
    let btn_path = format!("{}{}", PIECES_CHESS, "wq.png");
    let btn_str = btn_path.as_str();
    let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
        btn_str
        ).expect("error al obtener pixbuf");
    let pixbuf = pixbuf1.scale_simple (
            size_btn, size_btn,
            gdk_pixbuf::InterpType::Bilinear
            ).expect("error al escalar pixbuf");
    let img = gtk::Image::from_pixbuf(Some(&pixbuf));
    let btn_queen = gtk::ToolButton::new(Some(&img), None);
    gtk::WidgetExt::set_widget_name(&btn_queen, "queen");
    toolbar.insert(&btn_queen, -1);
    gtk::WidgetExt::set_tooltip_markup(&btn_queen, Some("Promote pawn to queen"));

    // button rook
    let btn_path = format!("{}{}", PIECES_CHESS, "wr.png");
    let btn_str = btn_path.as_str();
    let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
        btn_str
        ).expect("error al obtener pixbuf");
    let pixbuf = pixbuf1.scale_simple (
            size_btn, size_btn,
            gdk_pixbuf::InterpType::Bilinear
            ).expect("error al escalar pixbuf");
    let img = gtk::Image::from_pixbuf(Some(&pixbuf));
    let btn_rook = gtk::ToolButton::new(Some(&img), None);
    gtk::WidgetExt::set_widget_name(&btn_rook, "rook");
    toolbar.insert(&btn_rook, -1);
    gtk::WidgetExt::set_tooltip_markup(&btn_rook, Some("Promote pawn to rook"));

    // button bishop
    let btn_path = format!("{}{}", PIECES_CHESS, "wb.png");
    let btn_str = btn_path.as_str();
    let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
        btn_str
        ).expect("error al obtener pixbuf");
    let pixbuf = pixbuf1.scale_simple (
            size_btn, size_btn,
            gdk_pixbuf::InterpType::Bilinear
            ).expect("error al escalar pixbuf");
    let img = gtk::Image::from_pixbuf(Some(&pixbuf));
    let btn_bishop = gtk::ToolButton::new(Some(&img), None);
    gtk::WidgetExt::set_widget_name(&btn_bishop, "bishop");
    toolbar.insert(&btn_bishop, -1);
    gtk::WidgetExt::set_tooltip_markup(&btn_bishop, Some("Promote pawn to bishop"));

    // button knight
    let btn_path = format!("{}{}", PIECES_CHESS, "wn.png");
    let btn_str = btn_path.as_str();
    let pixbuf1 = gdk_pixbuf::Pixbuf::from_file (
        btn_str
        ).expect("error al obtener pixbuf");
    let pixbuf = pixbuf1.scale_simple (
            size_btn, size_btn,
            gdk_pixbuf::InterpType::Bilinear
            ).expect("error al escalar pixbuf");
    let img = gtk::Image::from_pixbuf(Some(&pixbuf));
    let btn_knight = gtk::ToolButton::new(Some(&img), None);
    gtk::WidgetExt::set_widget_name(&btn_knight, "knight");
    toolbar.insert(&btn_knight, -1);
    gtk::WidgetExt::set_tooltip_markup(&btn_knight, Some("Promote pawn to knight"));

    btn_queen.connect_clicked(move |_button| {
        unsafe { PROMOTED_PIECE = "q"; }
    });

    btn_rook.connect_clicked(move |button| {
        button.grab_focus();
        unsafe { PROMOTED_PIECE = "r"; }
    });

    btn_bishop.connect_clicked(move |_button| {
        unsafe { PROMOTED_PIECE = "b"; }
    });

    btn_knight.connect_clicked(move |_button| {
        unsafe { PROMOTED_PIECE = "n"; }
    });

    dialog.show_all();
    let result = dialog.run();

    dialog.close();

    if result == gtk::ResponseType::Ok.into() {
        let piece: String;
        unsafe { piece = PROMOTED_PIECE.clone().to_string(); }
        return piece;
    }
    else {
        //unsafe { PROMOTED_PIECE = "q"; }
        return "q".to_string();
    }
}
