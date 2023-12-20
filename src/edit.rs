use std::ptr::null;

use egui::{*, emath::RectTransform};

use crate::draws_functions::{Circle, Single_Line, Draws, Rectangle, Segment, Text};


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

pub fn write_segments(draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform,stroke: &Stroke)  {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click_and_drag());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return 
    }
    if interaction.drag_started_by(PointerButton::Primary) {
        draws.push(Draws::Segment({
            let mut s = Segment::new(stroke);
            s.points[0] = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
            s
        }));
        //draws[new_draw_id].center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
        let new_draw_id = draws.len() - 1;
        draws[new_draw_id].to_segment().unwrap().points[1] = point;
        //point.distance(draws[new_draw_id].to_circle().unwrap().center);
        
    }
}

pub fn write_text(painter: &Painter,draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform,last_index: &mut Option<usize>,stroke: &Stroke) -> Option<usize> {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return None
    }
    if interaction.clicked() {
        draws.push(Draws::Text({
            let mut t = Text::new(stroke);
            t.point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
            t
        }));
        let new_draw_id = draws.len() - 1;
        *last_index = Some(new_draw_id);
        return Some(new_draw_id)
    }
    *last_index = None;
    None
    
}

pub fn read_keyboard_input(ui: &mut Ui, text: &mut Text,last_index: &mut Option<usize>) {
    println!("entra in readdddd");
    let input = ui.input(|i| i.events.clone() /*i.key_pressed(egui::Key::A)*/);
    input.iter().for_each(|event| {
        match event {
            Event::Key { key: Key::Enter, pressed: true, repeat:false, modifiers: Modifiers::SHIFT } => {
                let new_line = String::from("\n");
                text.add_input(&new_line);
            }

            Event::Key { key: Key::Enter, pressed: true, repeat:false, modifiers: Modifiers::NONE } => {
                *last_index = None;
            }

            Event::Key { key: Key::Backspace, pressed: true, repeat:false, modifiers: Modifiers::NONE } => {
                text.remove_input();
            }
            Event::Text(key) => {
                println!("{:?}",key);
                text.add_input(key);
            }
            _ =>{

            }
        }
    }

    )
}


