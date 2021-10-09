use gtk;
use gtk::prelude::*;


// https://gtk-rs.org/docs/gtk/trait.TextTagExt.html

pub fn tags_branchlevel(view: &gtk::TextView) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tag_margin0 = gtk::TextTag::new(Some("branchlevel0"));
    
    tag_margin0.set_property_foreground(Some("#1E1E1E"));
    tag_margin0.set_property_left_margin(8);
    tag_margin0.set_property_right_margin(8);
    tag_margin0.set_property_pixels_above_lines(8);
    tag_margin0.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_margin0);
    
    let tag_margin1 = gtk::TextTag::new(Some("branchlevel1"));
    
    tag_margin1.set_property_foreground(Some("#0440BC"));
    tag_margin1.set_property_left_margin(34);
    tag_margin1.set_property_right_margin(12);
    tag_margin1.set_property_pixels_above_lines(8);
    tag_margin1.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_margin1);

    let tag_margin2 = gtk::TextTag::new(Some("branchlevel2"));
    
    tag_margin2.set_property_foreground(Some("#00B300"));
    tag_margin2.set_property_left_margin(60);
    tag_margin2.set_property_right_margin(12);
    tag_margin2.set_property_pixels_above_lines(8);
    tag_margin2.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_margin2);

    let tag_margin3 = gtk::TextTag::new(Some("branchlevel3"));
    
    tag_margin3.set_property_foreground(Some("#0440BC"));
    tag_margin3.set_property_left_margin(82);
    tag_margin3.set_property_right_margin(12);
    tag_margin3.set_property_pixels_above_lines(8);
    tag_margin3.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_margin3);

    let tag_margin4 = gtk::TextTag::new(Some("branchlevel4"));
    
    tag_margin4.set_property_foreground(Some("#00B300"));
    tag_margin4.set_property_left_margin(114);
    tag_margin4.set_property_right_margin(12);
    tag_margin4.set_property_pixels_above_lines(8);
    tag_margin4.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_margin4);

    let tag_margin5 = gtk::TextTag::new(Some("branchlevel5"));
    tag_margin5.set_property_foreground(Some("#0440BC"));

    let mut _x = tabla_tags.add(&tag_margin5);

    let tag_figurine = gtk::TextTag::new(Some("figurine"));
    tag_figurine.set_property_font(Some("ChessSansUscf 11"));

    let mut _x = tabla_tags.add(&tag_figurine);

}


pub fn tags_branchlevel_matched(view: &gtk::TextView) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tag_margin0 = gtk::TextTag::new(Some("branchlevel0"));
    
    tag_margin0.set_property_foreground(Some("#1E1E1E"));
    tag_margin0.set_property_left_margin(6);
    tag_margin0.set_property_right_margin(6);
    tag_margin0.set_property_pixels_above_lines(6);
    tag_margin0.set_property_pixels_below_lines(6);

    let mut _x = tabla_tags.add(&tag_margin0);
    
    let tag_margin1 = gtk::TextTag::new(Some("branchlevel1"));
    
    tag_margin1.set_property_foreground(Some("#0440BC"));
    tag_margin1.set_property_left_margin(22);
    tag_margin1.set_property_right_margin(10);
    tag_margin1.set_property_pixels_above_lines(6);
    tag_margin1.set_property_pixels_below_lines(6);

    let mut _x = tabla_tags.add(&tag_margin1);

    let tag_margin2 = gtk::TextTag::new(Some("branchlevel2"));
    
    tag_margin2.set_property_foreground(Some("#00B300"));
    tag_margin2.set_property_left_margin(38);
    tag_margin2.set_property_right_margin(10);
    tag_margin2.set_property_pixels_above_lines(6);
    tag_margin2.set_property_pixels_below_lines(6);

    let mut _x = tabla_tags.add(&tag_margin2);

    let tag_margin3 = gtk::TextTag::new(Some("branchlevel3"));
    
    tag_margin3.set_property_foreground(Some("#0440BC"));
    tag_margin3.set_property_left_margin(50);
    tag_margin3.set_property_right_margin(10);
    tag_margin3.set_property_pixels_above_lines(6);
    tag_margin3.set_property_pixels_below_lines(6);

    let mut _x = tabla_tags.add(&tag_margin3);

    let tag_margin4 = gtk::TextTag::new(Some("branchlevel4"));
    
    tag_margin4.set_property_foreground(Some("#00B300"));
    tag_margin4.set_property_left_margin(66);
    tag_margin4.set_property_right_margin(10);
    tag_margin4.set_property_pixels_above_lines(6);
    tag_margin4.set_property_pixels_below_lines(6);

    let mut _x = tabla_tags.add(&tag_margin4);

    let tag_margin5 = gtk::TextTag::new(Some("branchlevel5"));
    tag_margin5.set_property_foreground(Some("#0440BC"));

    let mut _x = tabla_tags.add(&tag_margin5);

    let tag_figurine = gtk::TextTag::new(Some("figurine"));
    tag_figurine.set_property_font(Some("ChessSansUscf 10"));

    let mut _x = tabla_tags.add(&tag_figurine);

}

/*
pub fn tags_mvnr(view: &gtk::TextView) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tag_mvnr = gtk::TextTag::new(Some("mvnr"));

    let mut _x = tabla_tags.add(&tag_mvnr);
}
*/

pub fn tags_move(view: &gtk::TextView) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tag_move = gtk::TextTag::new(Some("selected"));
    tag_move.set_property_weight(600);

    let mut _x = tabla_tags.add(&tag_move);
}

/*
pub fn tags_nag(view: &gtk::TextView) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tag_nag = gtk::TextTag::new(Some("nag"));
    tag_nag.set_property_pixels_above_lines(8);
    tag_nag.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_nag);
}
*/
/*
pub fn tags_comment(view: &gtk::TextView) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tag_com = gtk::TextTag::new(Some("comment"));
    tag_com.set_property_pixels_above_lines(8);
    tag_com.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_com);
}
*/

pub fn create_tag_node(view: &gtk::TextView, node_idx: String) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tag_node = gtk::TextTag::new(Some(node_idx.as_str()));

    let mut _x = tabla_tags.add(&tag_node);
}

/*
pub fn create_tag_start_comment(view: &gtk::TextView, comm: String) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tmp_comm = format!("startComment{}", comm);
    let tag_com = gtk::TextTag::new(Some(tmp_comm.as_str()));
    tag_com.set_property_pixels_above_lines(8);
    tag_com.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_com);
}
*/
/*
pub fn create_tag_mvnr(view: &gtk::TextView, node_idx: String) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tmp_mvnr = format!("mvnr{}", node_idx);
    let tag_mvnr = gtk::TextTag::new(Some(tmp_mvnr.as_str()));
    tag_mvnr.set_property_pixels_above_lines(8);
    tag_mvnr.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_mvnr);
}
*/
/*
pub fn create_tag_comment(view: &gtk::TextView, idx: String) {
    let buf = view.get_buffer().expect("error al obtener el buffer");
    let tabla_tags = buf.get_tag_table().expect("error al obtener la tabla de tags");

    let tmp_comm = format!("comment{}", idx);
    let tag_com = gtk::TextTag::new(Some(tmp_comm.as_str()));
    tag_com.set_property_pixels_above_lines(8);
    tag_com.set_property_pixels_below_lines(8);

    let mut _x = tabla_tags.add(&tag_com);
}
*/