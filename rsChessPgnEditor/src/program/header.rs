use gtk::*;
use gtk::{Window};
use gtk::prelude::*;

use super::globs;
use super::xboard::CABECERA;
use super::utils;



#[derive(Clone)]
pub struct Form {
    pub event_header: gtk::EventBox,
    lbl_header : gtk::Label,
}


impl Form {
    pub fn new() -> Self {
        let event_header = gtk::EventBox::new();
        let lbl_header = gtk::Label::new(Some("Blancas - Negras - Resultado"));
        gtk::WidgetExt::set_tooltip_markup(&lbl_header, Some("Click to modify header game"));
        event_header.add(&lbl_header);

        let attr_list = pango::AttrList::new();
        let mut attr = pango::Attribute::new_family("Courier").unwrap();  // -> Option<Attribute>
        attr.set_start_index(0);    // el primer caracter de un str
        attr_list.insert(attr);
        //etc...
        let mut attr = pango::Attribute::new_foreground(0, 0, 65535)
                                .expect("Couldn't create new foreground");
        attr.set_start_index(0);
        attr_list.insert(attr);
        
        let mut attr = pango::Attribute::new_scale(1.2)
                                .expect("Couldn't create new scale");
        attr.set_start_index(0);
        attr_list.insert(attr);
        lbl_header.set_attributes(Some(&attr_list));
        
        event_header.connect_button_press_event( move |widg, _event| {
            modif_internal_header(widg);
            Inhibit(true)
        });
        
        Form {
            event_header: event_header,
            lbl_header : lbl_header,
        }
        
    }


    pub fn modif_lbl_header (&mut self) {
        /*
        let widg = self.table.get_child_at(0,1).unwrap();   // es un gtk::Widget
        // https://gtk-rs.org/docs/gtk/struct.Widget.html  --> Blanket Implementations in gtk::prelude
        let lbl : &gtk::Label = gtk::Widget::downcast_ref(&widg).unwrap();
        // ahora el texto en formato gstring
        let texto1 = lbl.get_text();
        // convertimos a texto normal
        let text_row1 = glib::GString::as_str(&texto1);
        println!("67 {}", text_row1);
        */

        unsafe {
            if let Some(cabecera) = CABECERA.clone() {
                let blancas = cabecera.white;
                let negras = cabecera.black;
                let res: &str;
                match cabecera.res.as_str() {
                    "1" => res = "1-0", // => return "1".to_string(),
                    "2" => res ="1/2-1/2",   // => return "2".to_string(),
                    "3" => res = "0-1",     // => return String::from("3"),
                    "4" => res = "*",       // => return String::from("4"),
                    _ => res = " ",
                };
                
                let txt_lbl = format!("{}   {}   {}", blancas, negras, res);
                self.lbl_header.set_text(&txt_lbl);
            }
        }
        
    }

}


pub fn modif_internal_header(ev_box: &gtk::EventBox) {
    // http://gtk-rs.org/docs/gtk/trait.BinExt.html
    let widg = ev_box.get_child().unwrap();   // es un gtk::Widget
    let label : &gtk::Label = gtk::Widget::downcast_ref(&widg).unwrap();

    let mut cabecera: globs::GameInfo;
    unsafe {
        if CABECERA.is_none() {
            utils::alerta("No hay datos en la cabecera del PGN");
            return
        }
        cabecera = CABECERA.clone().unwrap();
    }
    
    let dialog = gtk::Dialog::with_buttons(
                    Some("Modificar datos de la partida PGN"),
                    None::<&Window>,   // es el parent
                    gtk::DialogFlags::MODAL,
                    &[("Grabar", gtk::ResponseType::Ok), ("Cancelar", gtk::ResponseType::Close)]
                );
    dialog.set_position(gtk::WindowPosition::CenterAlways);
    let top_area = dialog.get_content_area(); // -> Box
    let vbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    top_area.pack_start(&vbox, true, false, 20);
    
    let table = gtk::Grid::new();
    table.set_row_spacing(5);
  
    // TODO: function to automate all of this
    let mut fila = 0;
    let lbl_event = gtk::Label::new(Some("Event"));
    let entry_event = gtk::Entry::new();
    entry_event.set_text(&cabecera.event);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_event, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_event, 1, fila, 1, 1);
    
    fila = 1;
    let lbl_date = gtk::Label::new(Some("Date"));
    let entry_date = gtk::Entry::new();
    entry_date.set_text(&cabecera.date);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_date, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_date, 1, fila, 1, 1);
    
    fila = 2;
    let lbl_round = gtk::Label::new(Some("Round"));
    let entry_round = gtk::Entry::new();
    entry_round.set_text(&cabecera.round);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_round, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_round, 1, fila, 1, 1);
    
    fila = 3;
    let lbl_white = gtk::Label::new(Some("White"));
    let entry_white = gtk::Entry::new();
    entry_white.set_text(&cabecera.white);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_white, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_white, 1, fila, 1, 1);
    
    fila = 4;
    let lbl_black = gtk::Label::new(Some("Black"));
    let entry_black = gtk::Entry::new();
    entry_black.set_text(&cabecera.black);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_black, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_black, 1, fila, 1, 1);
    
    fila = 5;
    let lbl_result = gtk::Label::new(Some("Result"));
    let entry_result = gtk::Entry::new();
    entry_result.set_text(&cabecera.res);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_result, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_result, 1, fila, 1, 1);
    
    fila = 6;
    let lbl_eco = gtk::Label::new(Some("ECO"));
    let entry_eco = gtk::Entry::new();
    entry_eco.set_text(&cabecera.eco);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_eco, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_eco, 1, fila, 1, 1);
    
    fila = 7;
    let lbl_w_elo = gtk::Label::new(Some("White ELO"));
    let entry_w_elo = gtk::Entry::new();
    entry_w_elo.set_text(&cabecera.elow);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_w_elo, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_w_elo, 1, fila, 1, 1);
    
    fila = 8;
    let lbl_b_elo = gtk::Label::new(Some("Blacl ELO"));
    let entry_b_elo = gtk::Entry::new();
    entry_b_elo.set_text(&cabecera.elob);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::End);
    hbox.pack_start(&lbl_b_elo, true, true, 20);
    table.attach(&hbox, 0, fila, 1, 1);
    table.attach(&entry_b_elo, 1, fila, 1, 1);
    
    vbox.pack_start(&table, true, true, 20);
    
    dialog.show_all();
    let result = dialog.run();
    if result == gtk::ResponseType::Ok.into() {
        unsafe {
            cabecera.event = entry_event.get_text().to_string();   //.as_str()
            cabecera.site = entry_event.get_text().to_string();
            cabecera.date = entry_date.get_text().to_string();
            cabecera.round = entry_round.get_text().to_string();
            cabecera.white = entry_white.get_text().to_string();
            cabecera.black = entry_black.get_text().to_string();
            cabecera.res = entry_result.get_text().to_string();
            cabecera.eco = entry_eco.get_text().to_string();
            cabecera.elow = entry_w_elo.get_text().to_string();
            cabecera.elob = entry_b_elo.get_text().to_string();
            
            CABECERA = Some(cabecera);
        }

        let txt_lbl = format!("{} - {} - {}", entry_white.get_text().to_string(), 
                entry_black.get_text().to_string(), entry_result.get_text().to_string());
        label.set_text(&txt_lbl);
        label.show();
    }
    
    //unsafe { dialog.destroy(); }
    dialog.close();
}