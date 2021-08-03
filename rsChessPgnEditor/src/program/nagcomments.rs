use gtk::*;
use gtk::prelude::*;

use super::globs;
use super::utils;
use super::xboard::{NODO_ACTIVO, NODOS};
use super::notationview::{ITERS_FONDO_TAG, select_node_noexists, reset_buffer};
//use super::tags_textbuffer as ttbuf;
use super::pgn as ph;


pub const CSSMENU: &'static str = "
#menunag, #nagcalidad, #nagevalua, #nagotros { 
    
    border-radius:0.4125rem ; 
    margin: 0.275rem;
    padding: 0.275rem 0; 
    
    background-color: #414141;
    /*
    background-image:linear-gradient(to bottom left,rgba(72,72,72,0.97),rgba(71,69,67,0.97),rgba(52,52,52,0.97)); 
    border: none;
    */
    color: #ececec;
}
#menunag menuitem:hover { 
    box-shadow: inset 0 -3px #414141;   /*#15539e;*/
    background-color: #7b7d7d ;
    color: #ececec;
}

#sep1, #sep2 { 

    background: rgba(236,236,236, 0.28); 
    min-width: 1px; 
    min-height: 1px; 
}

";


const CSSCOMMENT: &'static str = "
#commtxt, text{
    /*background-color: #f2f2f2;*/
    background-color: rgba(242,242,242,0.99);
    font-size: 16px;
    font-family: Sans;
    padding: 10px;
}
#dialogcomment .dialog-action-area button {
    background-color: #333333;
    border-color: #333333;
    border-bottom-color: #333333;
    background-image: none;
    outline-color: rgba(44, 63, 117, 0.3);
    color: #ececec;
    box-shadow: none;
}
";


pub fn crea_menu_nag (visor: &gtk::TextView, area: &gtk::DrawingArea) {
    // Cargamos el CSS
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSSMENU.as_bytes())
        .expect("Failed to load CSS");
    // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
    // que agregamos se puedan aplicar a nuestra ventana.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    let menu = gtk::Menu::new();
    gtk::WidgetExt::set_widget_name(&menu, "menunag");

    let comen = gtk::MenuItem::with_label("Comentario posterior ...");
    menu.append(&comen);
    let separator1 = gtk::SeparatorMenuItem::new();
    gtk::WidgetExt::set_widget_name(&separator1, "sep1");
    menu.append(&separator1);

    let calidad = gtk::MenuItem::with_label("!, ?, ...");
    let sub_calidad = gtk::Menu::new();
    gtk::WidgetExt::set_widget_name(&sub_calidad, "nagcalidad");
    calidad.set_submenu(Some(&sub_calidad));
    let calidad_ninguno = gtk::MenuItem::with_label("(Ninguno)");
    let calidad_buena = gtk::MenuItem::with_label("Buena jugada - !");
    let calidad_mala = gtk::MenuItem::with_label("Mala jugada - ?");
    let calidad_muy_buena = gtk::MenuItem::with_label("Muy buena jugada - \u{203C}");
    let calidad_muy_mala = gtk::MenuItem::with_label("Muy mala jugada - \u{2047}");
    let calidad_interesante = gtk::MenuItem::with_label("Jugada interesante - \u{2049}");
    let calidad_dudosa = gtk::MenuItem::with_label("Jugada dudosa - \u{2048}");
    let calidad_forzada = gtk::MenuItem::with_label("Jugada forzada - \u{25A1}");
    let calidad_zugzwang = gtk::MenuItem::with_label("Zugzwang - \u{2A00}");
    sub_calidad.append(&calidad_ninguno);
    sub_calidad.append(&calidad_buena);
    sub_calidad.append(&calidad_mala);
    sub_calidad.append(&calidad_muy_buena);
    sub_calidad.append(&calidad_muy_mala);
    sub_calidad.append(&calidad_interesante);
    sub_calidad.append(&calidad_dudosa);
    sub_calidad.append(&calidad_forzada);
    sub_calidad.append(&calidad_zugzwang);

    let evalua = gtk::MenuItem::with_label("\u{2213}, =, ...");
    let sub_evalua = gtk::Menu::new();
    gtk::WidgetExt::set_widget_name(&sub_evalua, "nagevalua");
    evalua.set_submenu(Some(&sub_evalua));
    let evalua_ninguno = gtk::MenuItem::with_label("(Ninguno)");
    let evalua_igualdad = gtk::MenuItem::with_label("Igualdad - =");
    let evalua_confuso = gtk::MenuItem::with_label("Poco claro - \u{221E}");
    let evalua_ligera_blanca = gtk::MenuItem::with_label("Ligera ventaja blanca - \u{2A72}");
    let evalua_ligera_negra = gtk::MenuItem::with_label("Ligera ventaja negra - \u{2A71}");
    let evalua_moderada_blanca = gtk::MenuItem::with_label("Moderada ventaja blanca - \u{00B1}");
    let evalua_moderada_negra = gtk::MenuItem::with_label("Moderada ventaja negra - \u{2213}");
    let evalua_decisiva_blanca = gtk::MenuItem::with_label("Decisiva ventaja blanca - +-");
    let evalua_decisiva_negra = gtk::MenuItem::with_label("Decisiva ventaja negra - -+");
    let evalua_compensacion = gtk::MenuItem::with_label("Compensación - =/\u{221E}");
    let evalua_ataque = gtk::MenuItem::with_label("Ataque - \u{2192}");
    let evalua_iniciativa = gtk::MenuItem::with_label("Iniciativa - \u{2191}");
    let evalua_contrajuego = gtk::MenuItem::with_label("Contrajuego - \u{21C6}");
    let evalua_desarrollo = gtk::MenuItem::with_label("Ventaja de desarrollo - \u{27F3}");
    let evalua_novedad = gtk::MenuItem::with_label("Novedad - N");
    let evalua_tiempo = gtk::MenuItem::with_label("Apuro de tiempo - \u{1F540}");  //
    sub_evalua.append(&evalua_ninguno);
    sub_evalua.append(&evalua_igualdad);
    sub_evalua.append(&evalua_confuso);
    sub_evalua.append(&evalua_ligera_blanca);
    sub_evalua.append(&evalua_ligera_negra);
    sub_evalua.append(&evalua_moderada_blanca);
    sub_evalua.append(&evalua_moderada_negra);
    sub_evalua.append(&evalua_decisiva_blanca);
    sub_evalua.append(&evalua_decisiva_negra);
    sub_evalua.append(&evalua_compensacion);
    sub_evalua.append(&evalua_ataque);
    sub_evalua.append(&evalua_iniciativa);
    sub_evalua.append(&evalua_contrajuego);
    sub_evalua.append(&evalua_desarrollo);
    sub_evalua.append(&evalua_novedad);
    sub_evalua.append(&evalua_tiempo);

    let otros = gtk::MenuItem::with_label("Otros ...");
    let sub_otros = gtk::Menu::new();
    gtk::WidgetExt::set_widget_name(&sub_otros, "nagotros");
    otros.set_submenu(Some(&sub_otros));
    let otros_ninguno = gtk::MenuItem::with_label("(Ninguno)");
    let otros_editorial = gtk::MenuItem::with_label("Comentario Editorial - RR");
    let otros_mejor = gtk::MenuItem::with_label("Es mejor - \u{2313}");
    let otros_peor = gtk::MenuItem::with_label("Es peor - \u{2264}");
    let otros_idea = gtk::MenuItem::with_label("Con la idea - \u{2206}");
    let otros_contra = gtk::MenuItem::with_label("Dirigido contra - \u{2207}");
    sub_otros.append(&otros_ninguno);
    sub_otros.append(&otros_editorial);
    sub_otros.append(&otros_mejor);
    sub_otros.append(&otros_peor);
    sub_otros.append(&otros_idea);
    sub_otros.append(&otros_contra);

    //let close = MenuItem::new_with_label("Close");                        // on_close

    menu.append(&calidad);
    menu.append(&evalua);
    menu.append(&otros);
    let separator2 = gtk::SeparatorMenuItem::new();
    gtk::WidgetExt::set_widget_name(&separator2, "sep2");
    menu.append(&separator2);

    menu.show_all();
    // https://gtk-rs.org/docs/gtk/prelude/trait.GtkMenuExtManual.html#tymethod.popup_easy
    menu.popup_easy(1, 3);

    // los closures
    {
        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        comen.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                        Some(visor) => visor,
                        None => return,
                    };
            let area = match weak_area.upgrade() {
                        Some(area) => area,
                        None => return,
                    };
            let active_node: String;
            let mut nodes: globs::ListNodes;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
                nodes = NODOS.clone().unwrap();
            }
            if !nodes.node_exists(active_node.clone()) {
                let txt = format!("Move {} does not exists", active_node.clone());
                utils::alerta(&txt);
                return;
            }
            // just in case
            //let mut old_comment = String::from("");
            //let mut new_comment = String::from("");

            let old_comment = nodes.nodes[&active_node.clone()].comment.clone();
            let new_comment = mod_comment(old_comment.clone());
            if new_comment.len() == 0 && old_comment.len() == 0 { return; }
            if new_comment == old_comment { return; }

            nodes.set_comment(active_node.clone(), new_comment);
            reset_buffer(&visor);
            super::board_gui::draw_notation(&visor, &mut nodes.clone());
            select_node_noexists (nodes, &visor, &area, active_node);
            
        });


        // now NAGS
        // instead of $1, etc only the number as usize
        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_ninguno.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            let nodes: globs::ListNodes;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
                nodes = NODOS.clone().unwrap();
            }
            let nag = 0_usize;
            do_empty_nag(nag, active_node, nodes, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_buena.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 1_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_mala.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 2_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_muy_buena.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 3_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_muy_mala.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 4_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_interesante.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 5_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_dudosa.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 6_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_forzada.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 7_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        calidad_zugzwang.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 22_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });
        // ---------------------
        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_ninguno.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            let nodes: globs::ListNodes;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
                nodes = NODOS.clone().unwrap();
            }
            let nag = 0_usize;
            do_empty_nag(nag, active_node, nodes, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_igualdad.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 10_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_confuso.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 13_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_ligera_blanca.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 14_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_ligera_negra.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 15_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_moderada_blanca.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 16_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_moderada_negra.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 17_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_decisiva_blanca.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 18_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_decisiva_negra.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 19_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_compensacion.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 44_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_ataque.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 40_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_iniciativa.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 36_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_contrajuego.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 132_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_desarrollo.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 32_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_novedad.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 146_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        evalua_tiempo.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 136_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });
        // ---------------
        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        otros_ninguno.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            let nodes: globs::ListNodes;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
                nodes = NODOS.clone().unwrap();
            }
            let nag = 0_usize;
            do_empty_nag(nag, active_node, nodes, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        otros_editorial.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 145_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        otros_mejor.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 142_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        otros_peor.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 143_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        otros_idea.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 140_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });

        let weak_visor = glib::object::ObjectExt::downgrade(visor);
        let weak_area = glib::object::ObjectExt::downgrade(area);
        otros_contra.connect_activate( move |_m| {
            let visor = match weak_visor.upgrade() {
                Some(visor) => visor,
                None => return,
            };
            let area = match weak_area.upgrade() {
                    Some(area) => area,
                    None => return,
            };

            let active_node: String;
            unsafe {
                active_node = NODO_ACTIVO.clone().unwrap();
            }
            let nag = 141_usize;
            do_other_nags(nag, active_node, &visor, &area);
        });
    }
}


fn mod_comment (texto: String) -> String {
    
    let dialog = gtk::Dialog::with_buttons(
                          Some("Comentario jugada"),
                          None::<&Window>,   // es el parent
                          gtk::DialogFlags::MODAL,
                          &[("Ok", gtk::ResponseType::Ok),
                          ("Cancelar", gtk::ResponseType::Cancel)]
                      );
    
    // Cargamos el CSS sucede aquí.
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(CSSCOMMENT.as_bytes())
        .expect("Failed to load CSS");
    // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
    // que agregamos se puedan aplicar a nuestra ventana.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    dialog.set_size_request(350, 200);
    dialog.set_position(gtk::WindowPosition::Mouse);
    gtk::WidgetExt::set_widget_name(&dialog, "dialogcomment");
    let top_area = dialog.get_content_area(); // -> Box
    

    let vista_comen = gtk::TextView::new();
    gtk::WidgetExt::set_widget_name(&vista_comen, "commtxt");
    top_area.pack_start(&vista_comen, true, true, 3);
    let buffer_comen = vista_comen.get_buffer().expect("error en get_buffer");
    buffer_comen.set_text("");  // first we clear the buffer
    buffer_comen.set_text(&texto);
    
    let texto_retorno;  // = texto;

    dialog.show_all();
    let result = dialog.run();

    if result == gtk::ResponseType::Ok.into() {
        let inicio = buffer_comen.get_start_iter();
        let fin = buffer_comen.get_end_iter();
        let gstring = buffer_comen.get_text(&inicio, &fin, true)
                    .expect("error al obtener el gstring");
        let txtstr = gstring.as_str();
        // reemplazamos caracteres no compatibles
        let mut temp = txtstr.replace("{", "[");
        temp = temp.replace("}", "]");
        temp = temp.replace("\"", "'");
        temp = temp.replace("\n", " ");
        texto_retorno = temp;
    }
    else {
        buffer_comen.set_text("");
        texto_retorno = texto;
    }

    dialog.close();
    texto_retorno
}


fn do_empty_nag(nag: usize, cur_node: String, 
        mut listnodes: globs::ListNodes, view: &TextView,
        area: &gtk::DrawingArea) {
    if nag != 0 { return; }

    if !listnodes.node_exists(cur_node.clone()) {
        let txt = format!("Move {} does not exists", cur_node.clone());
        utils::alerta(&txt);
        return;
    }

    if listnodes.nodes[&cur_node.clone()].nag.len() == 0 {
        // do nothing
        return;
    }
    else {
        //let old_nag = listnodes.nodes[&cur_node.clone()].nag.clone();
        let new_nag = "".to_string();
        //println!("old-nag {}", old_nag);
        
        let mut iters_saved: (gtk::TextIter, gtk::TextIter);
        unsafe {
            match ITERS_FONDO_TAG.clone() {
                Some(iters) => {
                    iters_saved = iters;
                },
                None => {
                    utils::alerta("No move selected");
                    return;
                },
            }
        }
        
        let buf = view.get_buffer().unwrap();
        //  ahora nos colocamos dos espacios hacia delante
        let mut end_tag_iter: (gtk::TextIter, gtk::TextIter);
        let mut end_tag_iter1 = gtk::TextIter::forward_search(&mut iters_saved.1,
                " ",
                gtk::TextSearchFlags::VISIBLE_ONLY,
                None);
        let mut tag_begin: gtk::TextIter = buf.get_end_iter();  // arbitrary initial value
        // como hay un nag hacemos otro salto de espacio
        if end_tag_iter1.is_some() {
            tag_begin = end_tag_iter1.clone().unwrap().1.clone();
            end_tag_iter1 = gtk::TextIter::forward_search(&mut end_tag_iter1.unwrap().1,
                " ",
                gtk::TextSearchFlags::VISIBLE_ONLY,
                None);
        }
        
        if end_tag_iter1.is_none() {    // there isn't space after last move
            end_tag_iter = (buf.get_end_iter(), buf.get_end_iter());
        }
        else {
            end_tag_iter = end_tag_iter1.clone().unwrap();
        }
        //let tmp = format!("{} ", new_comment);
        //buf.insert(&mut end_tag_iter.1, &tmp);
        if end_tag_iter1.is_some() {
            buf.delete(&mut tag_begin, &mut end_tag_iter.1);
        }
        else {
            //println!("error en tag iter");
            buf.delete(&mut end_tag_iter.0, &mut end_tag_iter.1);
        }
        //println!("399 old tag {}", old_nag);
        //view.set_editable(true);
        //view.show();

        // now update nodes
        listnodes.set_nag(cur_node.clone(), new_nag);
        // the iters have changed (reselect the node)
        select_node_noexists (listnodes, &view, &area, cur_node);
    }

}


fn do_other_nags(nag: usize, cur_node: String, 
        view: &TextView,
        area: &gtk::DrawingArea) {

    if nag == 0 { return; }

    let mut listnodes: globs::ListNodes;
    unsafe {
        listnodes = NODOS.clone().unwrap();
    }

    if !listnodes.node_exists(cur_node.clone()) {
        let txt = format!("Move {} does not exists", cur_node.clone());
        utils::alerta(&txt);
        return;
    }

    let old_nag = listnodes.nodes[&cur_node.clone()].nag.clone();
    let new_nag = ph::NAG_SYMBOLS[nag];
    
    let mut iters_saved: (gtk::TextIter, gtk::TextIter);
        unsafe {
            match ITERS_FONDO_TAG.clone() {
                Some(iters) => {
                    iters_saved = iters;
                },
                None => {
                    utils::alerta("No move selected");
                    return;
                },
            }
        }
    
    let buf = view.get_buffer().unwrap();
    //  ahora nos colocamos unespacio hacia de lante o dos (si hay una nag insertada)
    let mut end_tag_iter: (gtk::TextIter, gtk::TextIter);
    let mut end_tag_iter1 = gtk::TextIter::forward_search(&mut iters_saved.1,
            " ",
            gtk::TextSearchFlags::VISIBLE_ONLY,
            None);
    // verficamos si hay un NAG oara saltarlo
    if listnodes.nodes[&cur_node.clone()].nag.len() > 0 {
        if end_tag_iter1.is_some() {
            //end_tag_iter1 = end_tag_iter1.clone();
            end_tag_iter1 = gtk::TextIter::forward_search(&mut end_tag_iter1.unwrap().1,
                " ",
                gtk::TextSearchFlags::VISIBLE_ONLY,
                None);
        }
    }
    if end_tag_iter1.is_none() {    // there isn't space after last move
        end_tag_iter = (buf.get_end_iter(), buf.get_end_iter());
    }
    else {
        end_tag_iter = end_tag_iter1.unwrap();
    }
    let tmpnag: String;
    if old_nag.len() > 0 {
        tmpnag = format!("{}", new_nag);
    }
    else {
        tmpnag = format!(" {}", new_nag);
    }
    buf.insert(&mut end_tag_iter.0, &tmpnag);

    //view.show();

    // now update nodes
    let fullnag = format!("{}${}", old_nag, nag);
    listnodes.set_nag(cur_node.clone(), fullnag);
    // the iters have changed
    select_node_noexists (listnodes, &view, &area, cur_node);
}