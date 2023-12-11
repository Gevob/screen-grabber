use std::ptr::null;

use egui::{*, emath::RectTransform};

use crate::draws_functions::{Circle, Single_Line, Draws};


pub fn write_lines(lines: &mut Vec<Single_Line>, ui: &mut Ui, original: RectTransform)  {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click_and_drag());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return 
    }
    if interaction.drag_started_by(PointerButton::Primary) {
        lines.push(Single_Line::new());
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let new_draw_id = lines.len() - 1;
        lines[new_draw_id].points.push(original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap()));
    }  
}

pub fn write_circles(draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform)  {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click_and_drag());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return 
    }
    if interaction.drag_started_by(PointerButton::Primary) {
        draws.push(Draws::circle({
            let mut c = Circle::new();
            c.center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
            c
        }));
        //draws[new_draw_id].center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
        let new_draw_id = draws.len() - 1;
        draws[new_draw_id].to_circle().unwrap().radius = point.distance(draws[new_draw_id].to_circle().unwrap().center);
        
    }  
}

pub fn write_rects(draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform)  {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click_and_drag());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return 
    }
    if interaction.drag_started_by(PointerButton::Primary) {
        draws.push(Draws::circle({
            let mut c = Circle::new();
            c.center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
            c
        }));
        //draws[new_draw_id].center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
        let new_draw_id = draws.len() - 1;
        draws[new_draw_id].to_circle().unwrap().radius = point.distance(draws[new_draw_id].to_circle().unwrap().center);
        
    }  
}

