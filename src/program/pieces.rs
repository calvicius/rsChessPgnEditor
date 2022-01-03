use std::collections::HashMap;

/*
Crea el mapa de piezas
======================
*/

pub fn crea_lista_piezas (dir: &str) -> HashMap<String,  gdk_pixbuf::Pixbuf>{
  let mut piezas: HashMap<String,  gdk_pixbuf::Pixbuf> = HashMap::new();
  let directorio = dir.to_string();
  // las piezas negras
  // alfil negro
  let mut pieza = "b".to_string();
  let mut nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  let mut pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // rey negro
  pieza = "k".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // caballo negro
  pieza = "n".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // peon negro
  pieza = "p".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // dama negra
  pieza = "q".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // torre negra
  pieza = "r".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  
  // las piezas blancas
  // alfil blanco
  pieza = "B".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // rey blanco
  pieza = "K".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // caballo blanco
  pieza = "N".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // peon blanco
  pieza = "P".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // dama blanca
  pieza = "Q".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // torre blanca
  pieza = "R".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  
  
  piezas
}

fn pieza_pixbuf (nom_fichero: String) -> gdk_pixbuf::Pixbuf {
  let pixbuf = gdk_pixbuf::Pixbuf::from_file (
      nom_fichero
    ).expect("error al obtener pixbuf");
    
  pixbuf
}
