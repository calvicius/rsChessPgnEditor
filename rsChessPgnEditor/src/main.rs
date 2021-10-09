//#![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;

extern crate gtk;
extern crate gdk;
extern crate pango;

//use gtk;

use std::process;

mod program;

use program::mainwin;


fn main() {
    if gtk::init().is_err() {
        eprintln!("No se ha podido iniciar la aplicacion GTK");
        process::exit(1);
    }
    
    let _app = mainwin::App::new();
    
    gtk::main();

}
