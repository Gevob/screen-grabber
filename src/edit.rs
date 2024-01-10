
use egui::{*, emath::RectTransform};
use egui::epaint::*;
use crate::draws_functions::{Circle, Single_Line, Draws, Rectangle, Segment, Text, Crop, Last_Action};
use ::image::{RgbaImage,GenericImageView};

pub fn write_lines(draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform,stroke: &Stroke,last_actions: &mut Vec<Last_Action>)  {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click_and_drag());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return 
    }
    if interaction.drag_started_by(PointerButton::Primary) {
        draws.push(Draws::Line({
            let l = Single_Line::new(stroke);
            l
        }));
        last_actions.push(Last_Action::Annotation);
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let new_draw_id = draws.len() - 1;
        draws[new_draw_id].to_line().unwrap().points.push(original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap()));
    }  
}

pub fn write_circles(draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform,stroke: &Stroke,last_actions: &mut Vec<Last_Action>)  {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click_and_drag());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return 
    }
    if interaction.drag_started_by(PointerButton::Primary) {
        draws.push(Draws::circle({
            let mut c = Circle::new(stroke);
            c.center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
            c
        }));
        //draws[new_draw_id].center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
        last_actions.push(Last_Action::Annotation);
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
        let new_draw_id = draws.len() - 1;
        draws[new_draw_id].to_circle().unwrap().radius = point.distance(draws[new_draw_id].to_circle().unwrap().center);
    }  
}

pub fn write_rects(draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform,stroke: &Stroke,last_actions: &mut Vec<Last_Action>)  {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click_and_drag());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return 
    }
    if interaction.drag_started_by(PointerButton::Primary) {
        draws.push(Draws::Rect({
            let mut r = Rectangle::new(original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap()),stroke);
            r
        }));
        last_actions.push(Last_Action::Annotation);
        //draws[new_draw_id].center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
        let new_draw_id = draws.len() - 1;
        draws[new_draw_id].to_rect().unwrap().rect = draws[new_draw_id].to_rect().unwrap().from_two_point(point);
        //point.distance(draws[new_draw_id].to_circle().unwrap().center);
        
    }
}


pub fn write_segments(draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform,stroke: &Stroke, last_actions: &mut Vec<Last_Action>)  {
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
        last_actions.push(Last_Action::Annotation);
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
        let new_draw_id = draws.len() - 1;
        draws[new_draw_id].to_segment().unwrap().points[1] = point;
        //point.distance(draws[new_draw_id].to_circle().unwrap().center);
        
    }
    // if interaction.drag_released_by(PointerButton::Primary){
    //     last_actions.push(Last_Action::Annotation);
    // }
    //last_actions.push(Last_Action::Annotation);
}

pub fn write_text(painter: &Painter,draws: &mut Vec<Draws>, ui: &mut Ui, original: RectTransform,last_index: &mut Option<usize>,stroke: &Stroke,last_actions: &mut Vec<Last_Action>) -> Option<usize> {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return None
    }
    if interaction.clicked() {
        draws.push(Draws::Text({
            let mut new_stroke = stroke.clone();
            new_stroke.width = new_stroke.width * 8.0;
            let mut t = Text::new(&new_stroke);
            t.point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
            t
        }));
        let new_draw_id = draws.len() - 1;
        *last_index = Some(new_draw_id);
        last_actions.push(Last_Action::Annotation);
        return Some(new_draw_id)
    }
    *last_index = None;
    None
    
}

pub fn read_keyboard_input(ui: &mut Ui, text: &mut Text,last_index: &mut Option<usize>) {
    
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
                
                text.add_input(key);
            }
            _ =>{

            }
        }
    }

    )
}


pub fn erase_edit(draws: &mut Vec<Draws>,ui: &mut Ui, original: RectTransform,painter: &Painter,garbage: &mut Vec<Draws>,last_actions: &mut Vec<Last_Action>) {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return
    }
    if interaction.clicked() {
        println!("Ho cliccatooooooooooooooooo");
        let coordinates = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap()); // saves the coordinates on logical rectangle when clicked
        // check if there is a draw on that coordinates
        let mut id: Option<i32> = None;
        draws.iter().enumerate().for_each( |(index, draw)| {
            match draw {
                Draws::Line(single_line) => { 
                    //println!("entro per cancellare la linea");     
                    single_line.points.iter().for_each(|(point)| {
                        //println!("Lines: {:?}, clicked: {:?}",point,coordinates);
                        let error = original.scale()[0]*single_line.stroke.width;

                        println!("scaling 0: {:?}, scaling 1: {:?}",original.scale()[0],original.scale()[1]);
                        if check_distance_from_point_any_line(point, &error, &coordinates) {
                            println!("FOUND");
                            id = Some(index as i32);
                           
                        }
                    });  
                }
                Draws::Circle(circle) => {
                    let center = circle.center;
                    let radius = circle.radius;
                    let error = original.scale()[0]*circle.stroke.width;
                    if check_distance_from_point_circle(&center, &error, &coordinates,&radius) {
                        println!("FOUND");
                        id = Some(index as i32);
                       
                    }
                }
                Draws::Rect(rectangle) => {
                    let error = original.scale()[0]*rectangle.stroke.width;
                    let h_l = Pos2::new(rectangle.rect.left(),rectangle.rect.top());
                    let h_r = Pos2::new(rectangle.rect.right(),rectangle.rect.top());
                    let l_r = Pos2::new(rectangle.rect.right(),rectangle.rect.bottom());
                    let l_l = Pos2::new(rectangle.rect.left(),rectangle.rect.bottom());
                    if check_distance_segment(&h_l, &h_r, &coordinates,&rectangle.stroke) ||
                       check_distance_segment(&h_r, &l_r, &coordinates,&rectangle.stroke) ||
                       check_distance_segment(&l_r, &l_l, &coordinates,&rectangle.stroke) ||
                       check_distance_segment(&l_l, &h_l, &coordinates,&rectangle.stroke) {
                        println!("FOUND");
                        id = Some(index as i32);
                    }
                }
                Draws::Text(text) => {
                    let galley = painter.layout_no_wrap(text.letters.clone(), FontId::monospace(text.stroke.width), text.stroke.color);
                    let rect = Align2::CENTER_CENTER.anchor_rect(Rect::from_min_size(text.point, galley.size()));
                    if Shape::galley(rect.min, galley).visual_bounding_rect().contains(coordinates) {
                        println!("FOUND");
                        id = Some(index as i32);
                    }
                }
                Draws::Segment(segment) => {
                    let error = original.scale()[0]*segment.stroke.width;

                    println!("scaling 0: {:?}, scaling 1: {:?}",original.scale()[0],original.scale()[1]);
                    if check_distance_segment(&segment.points[0], &segment.points[1], &coordinates,&segment.stroke) {
                        println!("FOUND");
                        id = Some(index as i32);
                       
                    }
                }
                // Utilizza l'underscore per trattare tutti gli altri casi
                _ => {
                }
            }
        });

        if id.is_some() {
            garbage.push(draws[id.unwrap() as usize].clone());
            draws.remove(id.unwrap() as usize);
            last_actions.push(Last_Action::Erase);
            
        }
    }
}

fn check_distance_segment(p1: &Pos2,p2: &Pos2,coord: &Pos2, stroke: &Stroke) -> bool {
    let mut p2c = Pos2::new(p2.x,p2.y);
    if p2.x == p1.x {
        p2c.x = p2c.x * 1.05;
    }
    if p2.y == p1.y {
        p2c.y = p2c.y * 1.05;
    }
    let mut result_x = (coord.x - p1.x) / (p2c.x - p1.x);
    let mut result_y = (coord.y - p1.y) / (p2c.y - p1.y);
    let mut m = ((p2c.y - p1.y) / (p2.x - p1.x)).abs();
    if m > 200.0 {
        m = 150.0;
    }
    let mut error = 0.027 / m;
    println!("Error before changing: {:?}",error);
    if error < 0.00099 {
        println!("1");
        error = (error * 3250.0) * 1.95;
    }
    else if error < 0.0099 {
        println!("2");
        error = (error * 225.0) * 1.05;
    }
    else if error < 0.003 {
        println!("3");
        error = (error * 1000.0) * 1.75;
    }
    println!("p1: {:?}, p2: {:?}",p1,p2);
    result_y = f32::trunc(result_y * 100.0) / 100.0;
    result_x = f32::trunc(result_x * 100.0) / 100.0;
    let difference =  (result_y - result_x).abs();
    println!("slope: {:?} clicked: {:?}, difference: {:?}, error: {:?}",m,coord,difference,error);
    println!("x: {:?}, y : {:?}",result_x,result_y);
    let stroke_influnce = 1.0 + (stroke.width / 500.0);
    // se il punto è esattamente sulla linea allora sono uguali ma devo verificare che siano simili per via della scarsa precisione
    if difference <= (error * stroke_influnce) && difference >= - (error * stroke_influnce){
        return true;
    }
    false
}

fn check_distance_from_point_circle(center: &Pos2,error: &f32,coord: &Pos2,radius: &f32) -> bool {
    let radius_from_click = ((center.x - coord.x).powf(2.0) + (center.y - coord.y).powf(2.0)).sqrt();
    
    let factor =  (*error * 1.10);
    let proportional_d = radius/factor;
    println!("center: {:?}, coord: {:?}d: {:?}, error: {:?}",center,coord,radius,error);
    println!("radius: {:?}, calculated one: {:?}",radius,radius_from_click);
    //println!("factor: {:?}, proportion: {:?}",factor,proportional_d);
    if /*proportional_d <= *error*/ radius_from_click <= (radius + factor) && radius_from_click >= (radius - factor) {
        println!("center: {:?}, coord: {:?}d: {:?}, error: {:?}",center,coord,radius,error);
        println!("factor: {:?}, proportion: {:?}",factor,proportional_d);
        return true
    }
    false
}

fn check_distance_from_point_any_line(center: &Pos2,error: &f32,coord: &Pos2) -> bool {
    let d = ((center.x - coord.x).powf(2.0) + (center.y - coord.y).powf(2.0)).sqrt();
    
    let factor =   100.0 / (*error * 1.05);
    let proportional_d = d/factor;
    //println!("factor: {:?}, proportion: {:?}",factor,proportional_d);
    if proportional_d <= *error{
        println!("center: {:?}, coord: {:?}d: {:?}, error: {:?}",center,coord,d,error);
        println!("factor: {:?}, proportion: {:?}",factor,proportional_d);
        return true
    }
    false
}


pub fn crop_rectangle(crop: &mut Crop,ui: &mut Ui, original: RectTransform,physical: RectTransform) {
    let interaction = ui.interact(*original.from(), ui.id(), Sense::click_and_drag());
    let click = interaction.interact_pointer_pos();
    if click.is_none() {
        return 
    }
    if interaction.drag_started_by(PointerButton::Primary) {
        //*crop = Crop::new(original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap()));
        let point = interaction.interact_pointer_pos().unwrap();
        println!("Vedremo 0? : {:?}",original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap()));
        crop.first_point_logical = original.transform_pos_clamped(point);
        crop.first_point = physical.transform_pos_clamped(point);
        // draws.push(Draws::Rect({
        //     let mut r = Rectangle::new(original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap()),stroke);
        //     r
        // }));
        //draws[new_draw_id].center = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
    }
    if interaction.dragged_by(PointerButton::Primary) {
        let point = interaction.interact_pointer_pos().unwrap();
        crop.from_two_point(physical.transform_pos_clamped(point));
        crop.from_two_point_logical(original.transform_pos_clamped(point));
        //println!("crop first: {:?}",crop.first_point);
        //println!("crop rect: {:?}",crop.rectangle);
        // let point = original.transform_pos_clamped(interaction.interact_pointer_pos().unwrap());
        // let new_draw_id = draws.len() - 1;
        //crop.from_two_point(point)
        // draws[new_draw_id].to_rect().unwrap().rect = draws[new_draw_id].to_rect().unwrap().from_two_point(point);
        //point.distance(draws[new_draw_id].to_circle().unwrap().center);
        
    }
    

    
}

pub fn crop_image(crop: &mut Crop, texture : &mut Option<TextureHandle>,image: &mut RgbaImage, painter: &Painter,ui: &mut Ui, last_actions: &mut Vec<Last_Action>,story_image : &mut Vec<RgbaImage>, story_texture : &mut Vec<Option<TextureHandle>>,  draws: &mut Vec<Draws>, last_crop: &mut Crop) {

    let input = ui.input(|i| i.events.clone() /*i.key_pressed(egui::Key::A)*/);
    input.iter().for_each(|event| {
        match event {
            Event::Key { key: Key::Enter, pressed: true, repeat:false, modifiers: Modifiers::NONE } => {
                // let cropped_image: ::image::SubImage<::image::ImageBuffer<::image::Rgba<u8>, Vec<u8>>> = ::image::SubImage::new(image.clone(), crop.rectangle.min.x as u32,
                //     crop.rectangle.min.y as u32,
                //     crop.rectangle.width() as u32,
                //     crop.rectangle.height() as u32);
                //println!("1");
                //println!("crop che crasha tutto: {:?}",crop);
                //println!("crop che crasha tutto: left top {:?}, right top {:?} width {:?} height {:?} image width {:?} height {:?}",crop.rectangle.left_top().x as u32,
                //crop.rectangle.left_top().y as u32,
                //crop.rectangle.width() as u32,
                //crop.rectangle.height() as u32,
                //image.width() as u32,
                //image.height() as u32);

                *last_crop = crop.clone();

                let cropped_image = image.view( crop.rectangle.left_top().x as u32,
                    crop.rectangle.left_top().y as u32,
                    crop.rectangle.width() as u32,
                    crop.rectangle.height() as u32);
                    //println!("2");
                let new_image = cropped_image.to_image();
                //println!("3");
                let flat_image: ::image::FlatSamples<&[u8]> = new_image.as_flat_samples();
                ///println!("4");
                let color_image2 = egui::ColorImage::from_rgba_unmultiplied([new_image.width() as usize, new_image.height() as usize],flat_image.samples);
                //println!("5");
                let image_data = egui::ImageData::from(color_image2);
                story_image.push(image.clone());
                story_texture.push(texture.clone());
                *texture = Some(ui.ctx().load_texture("screen", image_data, Default::default()));
                //println!("6");
                *image = new_image;
                //println!("7");
                last_actions.push(Last_Action::Crop(crop.left_top.clone()));
                crop.left_top = crop.rectangle_logical.left_top();
                //println!("8");
                crop.rectangle = Rect::NAN;
                crop.rectangle_logical = Rect::NAN;
                
                //println!("{:?}",last_actions);
                
            }
            _ =>{

            }
        }
    }

    )

}
