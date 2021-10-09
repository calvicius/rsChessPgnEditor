use std::collections::HashMap;

use super::xboard;

pub static mut INTERNAL_BOARD: Option<HashMap<String, String>> = None;

/*
 funciones para crear un tablero interno del tablero grafico
 ===========================================================
*/

pub fn procesa_notacion(arr_piezas: Vec<&str>,
    flipped: bool) -> HashMap<String, String> {

    let mut tablero: HashMap<String, String> = HashMap::new();
    let mut grafico: Vec<Vec<&str>> = Vec::new();
    let mut temporal: Vec<&str> = Vec::new();
    let mut sitio_piezas = arr_piezas.clone();

    if flipped {
        //sitio_piezas = arr_piezas.iter().rev().collect();
        sitio_piezas.reverse();
    }

    // ahora hacemos un array bidimensional
    for i in 0..8 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 8..16 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 16..24 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 24..32 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 32..40 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 40..48 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 48..56 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    temporal = Vec::new();
    for i in 56..64 {
        temporal.push(sitio_piezas[i].clone());
    }
    grafico.push(temporal);
    //temporal = Vec::new();

    for fila in 0..grafico.len() {
        for col in 0..grafico[fila].len() {
            let alfabeto = &grafico[fila][col];
            if *alfabeto == "*".to_string() {   //empty square
                continue;
            }
            
            let xycoord = alfa_notacion((7-fila, col));
            if xycoord != "None" {
                tablero.insert(xycoord, alfabeto.to_string());
            }
        }
    }
    unsafe{
        INTERNAL_BOARD = Some(tablero.clone());
    }
    tablero
}


// Necesitamos una manera de convertir las coordenadas x e y de una pieza 
// a su notación equivalente alfabética, por ejemplo, A1, D5, E3, etc.
pub fn alfa_notacion (xycoord: (usize, usize)) -> String {

    if !esta_en_tablero(xycoord) {
        return "None".to_string();
    }
    format!("{}{}",xboard::Y_EJE[xycoord.1], xboard::X_EJE[xycoord.0])
}

// la definición de un método para comprobar si una determinada
// coordenada está en el tablero
fn esta_en_tablero(coord: (usize, usize)) -> bool {
    //if coord.1 < 0 || coord.1 > 7 || coord.0 < 0 || coord.0 > 7 {
    if coord.1 > 7 || coord.0 > 7 {
        return false;
    }
    else { return true; }
    //false
}

// Necesitamos convertir una notacion a1, a8, etc a coordenadas x,y
// definimos un método que toma una coordenada x, y como una tupla y 
// devuelve su notación numérica equivalente, de la siguiente manera:
pub fn num_notacion(xycoord: &str) -> (usize, usize) {
    let car = xycoord.chars().nth(0).unwrap();
    let num_car = xycoord.chars().nth(1).unwrap();
    let col = xboard::Y_EJE.iter().position(|&x| x == car)
        .expect("error al obtener el num de col."); // Option<usize>
    let fila = (num_car.to_string()).parse::<usize>().unwrap() - 1;

    (fila, col)
}

/*
Fin de las funciones del tablero interno
*/ 