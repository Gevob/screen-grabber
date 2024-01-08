use crate::wrapper_functions;

use std::collections::HashSet;
use std::path::PathBuf;
use egui::Image;
use egui::epaint::TextShape;
use egui::{menu, Button, Color32};
use egui::emath::RectTransform;
use image::DynamicImage;
use egui::*;
use image::ImageBuffer;
use image::ImageOutputFormat;
use image::RgbaImage;
use image::GenericImageView;
//use eframe::egui;
//use eframe::egui::TextureHandle;
use crate::draws_functions::{Draws, Last_Action};
use crate::draws_functions::Crop;
use crate::{Schermata, edit, EditType};
use crate::screen;
use crate::MyGlobalHotKeyManager;
use global_hotkey::hotkey::{HotKey, Code, Modifiers};
use egui::{Grid, Stroke, Ui, Visuals, Label};
use image::{save_buffer, ImageFormat, ColorType};
use rfd::FileDialog;
use chrono::prelude::*;
use std::io::{Cursor, Write};
use image::io::Reader as ImageReader;
use std::ptr;
use std::thread;
use std::time::{Duration, Instant};
use arboard::{Clipboard, ImageData};
use std::io::stdout;
use screenshots::{Screen, display_info};
use crate::icons::*;

pub fn home(ctx: &egui::Context, schermata: &mut Schermata, image: &mut RgbaImage, texture : &mut Option<TextureHandle>, hotkeys_list: &mut Vec<(Modifiers, Code, String, u32)>, file_format: &mut String, save_path: &mut PathBuf, name_convention: &mut String, monitor_used: &mut usize,story_image : &mut Vec<RgbaImage>, story_texture : &mut Vec<Option<TextureHandle>>, free_to_screenshot: &mut bool, start_time : &mut Instant, delay_duration : &mut Duration, start_timer: &mut bool){
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            menu::bar(ui, |ui| {

                ui.menu_button("Settings", |ui| {
                    if ui.button("Custom Hotkey").on_hover_text("Customize your Hotkeys").clicked() {
                        *schermata = Schermata::Setting_Hotkey;
                    }

                    if ui.button("Saving settings").on_hover_text("Customize default saving options").clicked() {
                        *schermata = Schermata::Setting_Saving;
                    }

                    if ui.button("Timer settings").on_hover_text("Set a timer").clicked() {
                        *schermata = Schermata::Setting_Timer;
                    }
                }).response.on_hover_text("Change your Settings");; //.on_hover_text("Take a Screenshot");

                if ui.button("Screenshots").on_hover_text("Take a Screenshot").clicked() {
                    if delay_duration.is_zero() {
                        ctx.send_viewport_cmd(viewport::ViewportCommand::Minimized(true.into()));
                        *free_to_screenshot = true;
                    }
                    else {
                        ctx.send_viewport_cmd(viewport::ViewportCommand::Minimized(true.into()));
                        //thread::sleep(Duration::from_millis(5000));
                        *start_time = Instant::now();
                        *start_timer = true;
                    }
                    
                }
                    
            });

            ui.centered_and_justified(|ui| {
            //mostro le hotkeys registrate
                Grid::new("some_unique_id").show(ui, |ui| {
                    ui.label("REGISTERED KEYS");
                    ui.end_row();
        
                    for curr_hotkey in hotkeys_list.iter(){
                        //ui.label(hotkey_to_String(curr_hotkey.0, curr_hotkey.1)); 
                        ui.label(hotkey_to_String(curr_hotkey.0, curr_hotkey.1));
                        ui.label(curr_hotkey.2.clone());
                        ui.end_row();
                    }

                    ui.add_space(20.0);

                    ui.end_row();                
                    ui.label("CUSTOM SAVING");

                    ui.end_row();
                    ui.label("File Format: ");
                    ui.label(file_format.clone());

                    ui.end_row();
                    ui.label("Default Path :");

                    if *save_path == PathBuf::default(){
                        ui.label("Go to settings...");
                    }
                    else {
                        ui.label(save_path.clone().into_os_string().into_string().unwrap());
                    }

                    ui.end_row();
                    ui.label("File name:");
                    ui.label(name_convention.clone());

                    ui.end_row();
                    ui.end_row();

                    if *monitor_used == 9999{
                        let text = format!("All monitors are being used");
                        ui.label(text);
                    }
                    else{
                        let text = format!("Monitor {} is being used", (*monitor_used + 1));
                        ui.label(text);
                    }

                    ui.end_row();
                    ui.end_row();

                    if (*delay_duration).is_zero() {
                        let text = format!("No timer selected");
                        ui.label(text);
                    }
                    else{
                        let text = format!("Timer: {} seconds", ((*delay_duration).as_secs() as u64));
                        ui.label(text);
                    }
                    

                });
            });
    });    
}


pub fn edit(ctx: &egui::Context, draws: &mut Vec<Draws>, texture : &mut Option<TextureHandle>, frame: &mut eframe::Frame, stroke: &mut Stroke, schermata: &mut Schermata, rgba_image: &mut RgbaImage, file_format: &mut String, save_path: &mut PathBuf, name_convention: &mut String, last_index: &mut Option<usize>, mode: &mut EditType,crop: &mut Crop, last_actions: &mut  Vec<Last_Action>,story_image : &mut Vec<RgbaImage>, story_texture : &mut Vec<Option<TextureHandle>>, garbage: &mut Vec<Draws>, last_crop: &mut Crop){
    //sleep(Duration::from_millis(200));
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {  
        menu::bar(ui, |ui| {
            add_edits_buttons(ui, stroke, mode,last_index,draws,last_actions,rgba_image,texture,story_image,story_texture,crop,garbage,last_crop);
            if ui.button("Discard").clicked() {
                *schermata = Schermata::Home;
                //elimina anche gli edit
                *texture = None; //e setta a null la textureHandle
                crop.left_top = Pos2::ZERO;
                draws.clear();
                story_image.clear();
                story_texture.clear();
                last_actions.clear();
            }

            if ui.button("Save").clicked(){
                wrapper_functions::save_image(&mut story_image[0], save_path, name_convention, file_format, draws, last_crop);
            }

            if ui.button("Copy").on_hover_text("Copy the Screenshot to Clipboard").clicked() {
                wrapper_functions::copy_to_clipboard(&story_image[0], draws, last_crop.clone());
            }
        });

        ui.add_space(30.0);

        if !(texture.is_none()) { 
            ui.vertical_centered(|ui| {
                let mut padding = ui.max_rect();
                let x = texture.clone().unwrap().aspect_ratio();
                let y: f32 = padding.aspect_ratio();
                if x > y {
                    padding.set_bottom(ui.max_rect().top()+(ui.max_rect().height()/2.0 - (ui.max_rect().width() / x)/2.0 ));
                    ui.advance_cursor_after_rect(padding);
                }                
                let mut edited_image = Image::new(texture.as_ref().unwrap()).max_size(ui.available_size()).maintain_aspect_ratio(true).shrink_to_fit().ui(ui);
                let texture_rect2 = egui::Rect::from_min_size(Pos2::ZERO, texture.clone().unwrap().size_vec2()); //rettangolo della dimensione dell'immagine
                println!("crop first point: {:?}",crop.left_top);
                //println!("number of actions: {:?}",last_actions.len());
                let texture_rect = egui::Rect::from_min_size(crop.left_top, texture.clone().unwrap().size_vec2()); //rettangolo della dimensione dell'immagine
                let screen_rect = eframe::emath::RectTransform::from_to(texture_rect,edited_image.rect);
                let painter = Painter::new(ctx.clone(),edited_image.layer_id,edited_image.rect);
                match mode {
                    EditType::Circle => {
                        edit::write_circles(draws, ui,screen_rect.inverse(),stroke,last_actions);
                        
                    }
                    EditType::Rectangle => {
                        edit::write_rects(draws, ui, screen_rect.inverse(),stroke,last_actions);
                        
                    }
                    EditType::Free => {
                        edit::write_lines( draws, ui,screen_rect.inverse(),stroke,last_actions);
                        
                    }
                    EditType::Text => {
                        edit::write_text(&painter, draws, ui, screen_rect.inverse(),last_index,stroke,last_actions);
                        if last_index.is_some()  {
                            edit::read_keyboard_input(ui, draws[last_index.unwrap()].to_text().unwrap(),last_index);
                        }
                    }
                    EditType::Segment => {
                        edit::write_segments(draws, ui,screen_rect.inverse(),stroke,last_actions);
                        
                    }
                    EditType::Eraser => {
                        edit::erase_edit(draws, ui, screen_rect.inverse(),&painter,garbage,last_actions);

                    }
                    EditType::Crop => {
                        let screen_rect2 = eframe::emath::RectTransform::from_to(texture_rect2,edited_image.rect);
                        edit::crop_rectangle(crop,ui,screen_rect.inverse(),screen_rect2.inverse());
                        let min = screen_rect2.transform_pos(crop.rectangle.left_top());
                        let max = screen_rect2.transform_pos(crop.rectangle.right_bottom());
                        let shape = egui::Shape::rect_stroke(Rect::from_min_max(min,max), epaint::Rounding::ZERO, Stroke::new(3.0, Color32::from_rgb(255,255,255)));
                        let shape_filled = egui::Shape::rect_filled(Rect::from_min_max(min,max), epaint::Rounding::ZERO, Color32::from_rgba_unmultiplied(90, 90, 82, 60));
                        print_draws3(&painter, draws, screen_rect,last_index);
                        painter.add(shape);
                        painter.add(shape_filled);
                        edit::crop_image(crop, texture, rgba_image, &painter, ui,last_actions,story_image,story_texture, draws, last_crop);
                    }
                    _ => {

                    }
                }
                if *mode != EditType::Crop {
                print_draws3(&painter, draws, screen_rect,last_index);
                }
            });
        }
    });
}

fn add_edits_buttons(ui: &mut Ui, stroke: &mut Stroke, mode: &mut EditType,last_index: &mut Option<usize>, draws: &mut Vec<Draws>, last_actions: &mut  Vec<Last_Action>,rgba_image: &mut RgbaImage,texture : &mut Option<TextureHandle>,story_image : &mut Vec<RgbaImage>,story_texture : &mut Vec<Option<TextureHandle>>,crop: &mut Crop, garbage: &mut Vec<Draws>, last_crop: &mut Crop) {
    color_picker_and_width(ui, stroke);
    if edit_single_button(ui,&CURSOR,mode,&EditType::Cursor).clicked(){
        *mode = EditType::Cursor;
        *last_index = None;
    }
    if edit_single_button(ui,&ERASER,mode,&EditType::Eraser).clicked(){
        *mode = EditType::Eraser;
        *last_index = None;

    }
    if edit_single_button(ui,&CIRCLE,mode,&EditType::Circle).clicked(){
        *mode = EditType::Circle;
        *last_index = None;
    }
    if edit_single_button(ui,&RECTANGLE,mode,&EditType::Rectangle).clicked(){
        *mode = EditType::Rectangle;
        *last_index = None;
    }
    if edit_single_button(ui,&SEGMENT,mode,&EditType::Segment).clicked(){
        *mode = EditType::Segment;
        *last_index = None;
    }
    if edit_single_button(ui,&FREE,mode,&EditType::Free).clicked(){
        *mode = EditType::Free;
        *last_index = None;
    }
    if edit_single_button(ui,&TEXT,mode,&EditType::Text).clicked(){
        *mode = EditType::Text;
    }
    if edit_single_button(ui,&SCISSOR,mode,&EditType::Crop).clicked(){
        *mode = EditType::Crop;
        *last_index = None;
    }
    if edit_single_button(ui,&BACK,mode,&EditType::Back).clicked(){
        *last_index = None;
        if last_actions.last().is_some() {
            let last_action = last_actions.last().unwrap();
            match last_action {
                Last_Action::Annotation => {
                    if draws.len() > 0 {
                        draws.pop();
                        
                    }
                    last_actions.pop();
                }
                Last_Action::Crop(begin) => {
                    *texture = story_texture.last().unwrap().clone();
                    *rgba_image = story_image.last().unwrap().clone();
                    //println!("crop before stack: {:?}",crop.left_top);
                    crop.left_top = *begin;
                    //println!("crop after stack: {:?}",crop.left_top);

                    last_crop.rectangle_logical.set_left(begin.x);
                    last_crop.rectangle_logical.set_top(begin.y);
                    last_crop.rectangle_logical.set_width(rgba_image.width() as f32);
                    last_crop.rectangle_logical.set_height(rgba_image.height() as f32);

                    story_image.pop();
                    story_texture.pop();
                    last_actions.pop();
                    
                }
                Last_Action::Erase => {
                    draws.push(garbage.last().unwrap().clone());
                    garbage.pop();
                    last_actions.pop();
                }
            }
        }
    }

}

fn color_picker_and_width(ui: &mut Ui, stroke: &mut Stroke) {
    let size_points = egui::Vec2::new(128.0,32.0);
    let (id, rect) = ui.allocate_space(size_points);
    ui.allocate_ui_at_rect(rect, |ui| {
        ui.color_edit_button_srgba(&mut stroke.color);
    });
    let (id, rect2) = ui.allocate_space(size_points);
    ui.allocate_ui_at_rect(rect2, |ui| {
        ui.add(egui::Slider::new(&mut stroke.width, 1.0..=8.0).integer());   
    });
    
}

fn edit_single_button(ui: &mut Ui, image: &Image<'_>, mode: &EditType, current_mode: &EditType) -> Response {
    let size_points = egui::Vec2::splat(32.0);
    let (id, rect) = ui.allocate_space(size_points);
    let response = ui.interact(rect, id, Sense::click());
    if response.hovered() || mode == current_mode  {
        ui.painter().rect_filled(
            rect,
            Rounding::same(4.0),
            Color32::from_rgb(83,83,83)
        );
        //ui.visuals().widgets.active.fg_stroke.color
    }
    let image = image
    .clone()
    .maintain_aspect_ratio(true)
    //.tint(tint)
    .fit_to_exact_size(size_points);
    image.paint_at(ui, rect);
    //ui.add(Button::image(image));

    response
}

// pub fn print_draws(painter: &Painter, draws: &Vec<Draws>,screen_rect: RectTransform) {
//                     println!("Testo {:?}",draws);
//                     //print_text(painter);
//                     let shapes = 
//                     draws
//                     .iter()
//                     .map(|draw| {
                        
//                         match draw {
//                             Draws::Line(single_line) => {
//                                 let points: Vec<Pos2> = single_line.points.iter().map(|p| screen_rect.transform_pos_clamped(*p)).collect();
//                                 egui::Shape::line(points, Stroke::new(5.0,Color32::RED))
//                             }
//                             Draws::Circle(circle) => {
//                                 // Gestisci il caso Circle
//                                 let center = screen_rect.transform_pos_clamped(circle.center);
//                                 let modify = screen_rect.from().width() / screen_rect.to().width();
//                                 let radius = circle.radius / modify;
//                                 egui::Shape::circle_stroke(center, radius,Stroke::new(5.0,Color32::RED))
//                             }
//                             Draws::Rect(rectange) => {
//                                 // Gestisci il caso Circle
//                                 let min = screen_rect.transform_pos_clamped(rectange.rect.min);
//                                 let max = screen_rect.transform_pos_clamped(rectange.rect.max);
//                                 egui::Shape::rect_stroke(Rect::from_min_max(min, max), epaint::Rounding::ZERO, Stroke::new(5.0,Color32::RED))
//                             }
//                             Draws::Text(text) => {
//                                 // Gestisci il caso Circle
//                                 println!("Testo {:?}",text);
//                                 //let point = screen_rect.transform_pos_clamped(text.point);
//                                 //println!("Punto: {:?}",point);
//                                 println!("prima");
//                                 //let point_1 = screen_rect.transform_pos_clamped(text.points[0]);
//                                 //let point_2 = screen_rect.transform_pos_clamped(text.points[1]);
//                                 //print_text(painter);
//                                 //let galley = painter.layout_no_wrap(text.letters.clone(), FontId::monospace(32.0), Color32::RED);
//                                 //stdout().flush();
//                                 let galley = painter.fonts(|f|f.layout("Ciao bella\n".into(), FontId::proportional(1.0), Color32::RED, f32::INFINITY));
//                                 println!("dopo");
//                                 //egui::Shape::Text(TextShape::new(point, galley))
//                                 //egui::Shape::line_segment([point_1,point_2],Stroke::new(5.0,Color32::RED))
//                                 //text.render(painter, screen_rect)
//                                 egui::Shape::Noop
//                             }
//                             Draws::Segment(segment) => {
//                                 // Gestisci il caso Circle
//                                 let point_1 = screen_rect.transform_pos_clamped(segment.points[0]);
//                                 let point_2 = screen_rect.transform_pos_clamped(segment.points[1]);
//                                 egui::Shape::line_segment([point_1,point_2],Stroke::new(5.0,Color32::RED))
//                             }
//                             // Utilizza l'underscore per trattare tutti gli altri casi
//                             _ => {
//                                 egui::Shape::Noop
//                             }
//                         }
//                     });
//                     painter.extend(shapes);
// }

// pub fn print_draws2(painter: &Painter, draws: &mut Vec<Draws>,screen_rect: RectTransform) {
//     println!("Testo {:?}",draws);
//     println!("prima2");
//     let shapes = draws.iter().for_each(|dr| {
//         println!("Testo2 {:?}",dr);
//         // print_text(painter);
//         match dr {
//             Draws::Line(single_line) => {
//                 let points: Vec<Pos2> = single_line.points.iter().map(|p| screen_rect.transform_pos_clamped(*p)).collect();
//                 //egui::Shape::line(points, Stroke::new(5.0,Color32::RED))
//             }
//             Draws::Circle(circle) => {
//                 // Gestisci il caso Circle
//                 let center = screen_rect.transform_pos_clamped(circle.center);
//                 let modify = screen_rect.from().width() / screen_rect.to().width();
//                 let radius = circle.radius / modify;
//                 //egui::Shape::circle_stroke(center, radius,Stroke::new(5.0,Color32::RED))
//             }
//             Draws::Rect(rectange) => {
//                 // Gestisci il caso Circle
//                 let min = screen_rect.transform_pos_clamped(rectange.rect.min);
//                 let max = screen_rect.transform_pos_clamped(rectange.rect.max);
//                 //egui::Shape::rect_stroke(Rect::from_min_max(min, max), epaint::Rounding::ZERO, Stroke::new(5.0,Color32::RED))
//             }
//             Draws::Text(text) => {
//                 println!("dentro text================================");
//                 print_text(painter,text.letters.clone());
//                 //egui::Shape::Noop
//             }
//             _ => {
//                 //print_text(painter);
//                 println!("tutto");
//                 //egui::Shape::Noop
//             }
//         }
        
//     });
//     println!("dopo2");
// }

pub fn print_draws3(painter: &Painter, draws: &mut Vec<Draws>,screen_rect: RectTransform,last_index: &mut Option<usize>) {
    let mut shape: Vec<Shape> = Vec::new();
    //println!("Testo {:?}",draws);
    //print_text(painter);
    //let shapes = 
    draws
    .iter_mut().enumerate()
    .for_each(|(index,draw)| {
        match draw {
            Draws::Line(single_line) => {
                let points: Vec<Pos2> = single_line.points.iter().map(|p| screen_rect.transform_pos(*p)).collect();
                let mut proportional_stroke = single_line.stroke;
                proportional_stroke.width = proportional_stroke.width * screen_rect.scale()[0];
                shape.push(egui::Shape::line(points, /*single_line.stroke*/proportional_stroke));
            }
            Draws::Circle(circle) => {
                let center = screen_rect.transform_pos(circle.center);
                let modify = screen_rect.from().width() / screen_rect.to().width();
                let radius = circle.radius / modify;
                let mut proportional_stroke = circle.stroke;
                proportional_stroke.width = proportional_stroke.width * screen_rect.scale()[0];
                shape.push(egui::Shape::circle_stroke(center, radius,proportional_stroke));
            }
            Draws::Rect(rectangle) => {
                let min = screen_rect.transform_pos(rectangle.rect.min);
                let max = screen_rect.transform_pos(rectangle.rect.max);
                let mut proportional_stroke = rectangle.stroke;
                proportional_stroke.width = proportional_stroke.width * screen_rect.scale()[0];
                shape.push(egui::Shape::rect_stroke(Rect::from_min_max(min, max), epaint::Rounding::ZERO, proportional_stroke));
            }
            Draws::Text(text) => {
                let galley = painter.layout_no_wrap(text.letters.clone(), FontId::monospace(32.0), text.stroke.color);
                let real_rect = Align2::CENTER_CENTER.anchor_rect(Rect::from_min_size(text.point, galley.size()));
                text.real_pos = real_rect.left_top();
                let point = screen_rect.transform_pos(text.point);
                let rect = Align2::CENTER_CENTER.anchor_rect(Rect::from_min_size(point, galley.size()));
                if last_index.is_some() && last_index.unwrap() == index {
                let path = [Pos2::new(rect.left(),rect.top()),
                            Pos2::new(rect.right(),rect.top()),
                            Pos2::new(rect.right(),rect.bottom()),
                            Pos2::new(rect.left(),rect.bottom()),
                            Pos2::new(rect.left(),rect.top())];
                let dotted_line = Shape::dotted_line(&path, Color32::GRAY, 12.0, 4.0);
                shape.extend(dotted_line);
                }
                shape.push(Shape::galley(rect.min, galley));
            }
            Draws::Segment(segment) => {
                let point_1 = screen_rect.transform_pos(segment.points[0]);
                let point_2 = screen_rect.transform_pos(segment.points[1]);
                let mut proportional_stroke = segment.stroke;
                proportional_stroke.width = proportional_stroke.width * screen_rect.scale()[0];
                shape.push(egui::Shape::line_segment([point_1,point_2],proportional_stroke));
            }
            // Utilizza l'underscore per trattare tutti gli altri casi
            _ => {
                shape.push(egui::Shape::Noop);
            }
        }
    });
    
    painter.extend(shape);
}

pub fn setting_hotkey(ctx: &egui::Context, schermata: &mut Schermata, manager: &mut MyGlobalHotKeyManager, modifier_copy: &mut Modifiers, key_copy: &mut Code, modifier_screen: &mut Modifiers, key_screen: &mut Code, modifier_save: &mut Modifiers, key_save: &mut Code, hotkeys_list: &mut Vec<(Modifiers, Code, String, u32)>, modifier_copy_tmp: &mut Modifiers, key_copy_tmp: &mut Code, modifier_screen_tmp: &mut Modifiers, key_screen_tmp: &mut Code, modifier_save_tmp: &mut Modifiers, key_save_tmp: &mut Code, update_file: &mut bool){
    let window_size = egui::vec2(0.0, 0.0);

    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("Settings", |ui| {
                if ui.button("Custom Hotkey").clicked() {
                    *schermata = Schermata::Setting_Hotkey;
                }
                if ui.button("Saving settings").clicked() {
                    *schermata = Schermata::Setting_Saving;
                }
                if ui.button("Timer settings").clicked() {
                    *schermata = Schermata::Setting_Timer;
                }
            });
        });

        ui.add_space(20.0);

            Grid::new("miao").show(ui, |ui| {
                ui.label("COPY ");

                egui::ComboBox::from_id_source("Choose modifier copy")
                .selected_text(format!("{:?}", modifier_copy_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(modifier_copy_tmp, Modifiers::CONTROL, "Ctrl");
                    ui.selectable_value(modifier_copy_tmp, Modifiers::SHIFT, "Shift");
                    ui.selectable_value(modifier_copy_tmp, Modifiers::ALT, "Alt");
                });

                wrapper_functions::show_combo_box(ui, key_copy_tmp, "Copy key".to_string());

                ui.end_row();

                ui.label("SCREEN ");

                egui::ComboBox::from_id_source("Choose modifier screen")
                .selected_text(format!("{:?}", modifier_screen_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(modifier_screen_tmp, Modifiers::CONTROL, "Ctrl");
                    ui.selectable_value(modifier_screen_tmp, Modifiers::SHIFT, "Shift");
                    ui.selectable_value(modifier_screen_tmp, Modifiers::ALT, "Alt");
                });

                wrapper_functions::show_combo_box(ui, key_screen_tmp, "Screen key".to_string());

                ui.end_row();

                ui.label("SAVE ");

                egui::ComboBox::from_id_source("Choose modifier save")
                .selected_text(format!("{:?}", modifier_save_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(modifier_save_tmp, Modifiers::CONTROL, "Ctrl");
                    ui.selectable_value(modifier_save_tmp, Modifiers::SHIFT, "Shift");
                    ui.selectable_value(modifier_save_tmp, Modifiers::ALT, "Alt");
                });

                wrapper_functions::show_combo_box(ui, key_save_tmp, "Save key".to_string());

                ui.end_row();
            });


            ui.add_space(30.0);

            if ui.button("Chiudi").clicked(){

                for el in hotkeys_list.iter(){
                    //non lascio che un utente modifichi i valori delle caselle e poi lasci il casino...
                    //RIMETTO A POSTO...
                    if el.2 == "Copy".to_string(){
                        *modifier_copy_tmp = el.0.clone();
                        *key_copy_tmp = el.1.clone();
                    }
                    else if el.2 == "Screen".to_string(){
                        *modifier_screen_tmp = el.0.clone();
                        *key_screen_tmp = el.1.clone();
                    }
                    else{ // el.2 == "Save".to_string()
                        *modifier_save_tmp = el.0.clone();
                        *key_save_tmp = el.1.clone();
                    }
                }
                *schermata = Schermata::Home; 
            }

           //fai un check per verificare che tutte le hotkeys siano diverse
           let mut set = HashSet::<(Modifiers, Code)>::new();
           let curr_hotkey_list = vec![(*modifier_copy_tmp, *key_copy_tmp, "Copy"), (*modifier_screen_tmp, *key_screen_tmp, "Screen"), (*modifier_save_tmp, *key_save_tmp, "Save")];
           let all_distinct = curr_hotkey_list.iter().all(|x| set.insert((x.0,x.1)));
           
           let mut hotkeys_to_save = Vec::<HotKey>::new();
           let mut hotkeys_to_delete = Vec::<HotKey>::new();

           ui.set_enabled(all_distinct && ((*modifier_copy != *modifier_copy_tmp) || (*modifier_screen != *modifier_screen_tmp) || (*modifier_save != *modifier_save_tmp) || (*key_copy != *key_copy_tmp) || (*key_screen != *key_screen_tmp) || (*key_save != *key_save_tmp)));
            
           if ui.button("Salva modifiche").clicked() {
            *modifier_copy = *modifier_copy_tmp;
            *modifier_save = *modifier_save_tmp;
            *modifier_screen = *modifier_screen_tmp;
            *key_copy = *key_copy_tmp;
            *key_screen = *key_screen_tmp;
            *key_save = *key_save_tmp;

            //genera la hotkey modificata
            if all_distinct {
                for el in hotkeys_list.iter_mut(){
                    if el.2 == "Copy".to_string(){
                        if el.0 != *modifier_copy || el.1 != *key_copy{
                            let mut hotkey_copy = HotKey::new(Some(*modifier_copy), *key_copy);
                            let mut hotkey_to_delete = HotKey::new(Some(el.0), el.1);

                            hotkeys_to_save.push(hotkey_copy);
                            hotkeys_to_delete.push(hotkey_to_delete);

                            el.0 = *modifier_copy;
                            el.1 = *key_copy;
                            el.3 = hotkey_copy.id();
                        }
                    }
                    else if el.2 == "Screen".to_string(){
                        if el.0 != *modifier_screen || el.1 != *key_screen{
                            let mut hotkey_screen = HotKey::new(Some(*modifier_screen), *key_screen);
                            let mut hotkey_to_delete = HotKey::new(Some(el.0), el.1);

                            hotkeys_to_save.push(hotkey_screen);
                            hotkeys_to_delete.push(hotkey_to_delete);

                            el.0 = *modifier_screen;
                            el.1 = *key_screen;
                            el.3 = hotkey_screen.id();
                        }
                    }
                    else { //if el.2 == "Save".to_string()
                        if el.0 != *modifier_save || el.1 != *key_save{
                            let mut hotkey_save = HotKey::new(Some(*modifier_save), *key_save);
                            let mut hotkey_to_delete = HotKey::new(Some(el.0), el.1);

                            hotkeys_to_save.push(hotkey_save);
                            hotkeys_to_delete.push(hotkey_to_delete);

                            el.0 = *modifier_save;
                            el.1 = *key_save;
                            el.3 = hotkey_save.id();

                        }
                    }
                }

                ((*manager).0).unregister_all(&hotkeys_to_delete).unwrap();
                ((*manager).0).register_all(&hotkeys_to_save).unwrap(); //ho fatto in questo modo perchè GlobalHotKeyManager non aveva il tratto Default

                *update_file = true;
                *schermata = Schermata::Home; 
            }
        }
    });
}

pub fn setting_saving(ctx: &egui::Context, schermata: &mut Schermata, file_format: &mut String, save_path: &mut PathBuf, file_format_tmp: &mut String, save_path_tmp: &mut PathBuf, name_convention: &mut String, name_convention_tmp: &mut String, update_file: &mut bool, monitor_used: &mut usize, monitor_used_tmp: &mut usize){
    let window_size = egui::vec2(0.0, 0.0);

    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {

        menu::bar(ui, |ui| {
            ui.menu_button("Settings", |ui| {
                if ui.button("Custom Hotkey").clicked() {
                    *schermata = Schermata::Setting_Hotkey;
                }

                if ui.button("Saving settings").clicked() {
                    *schermata = Schermata::Setting_Saving;
                }

                if ui.button("Timer settings").clicked() {
                    *schermata = Schermata::Setting_Timer;
                }
            });
        });

        ui.add_space(20.0);
        
            egui::ComboBox::from_label("Choose format")
                .selected_text(format!("{}", file_format_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(file_format_tmp, ".png".to_string(), "PNG");
                    ui.selectable_value(file_format_tmp, ".jpeg".to_string(), "JPEG");
                    ui.selectable_value(file_format_tmp, ".gif".to_string(), "GIF");
                    ui.selectable_value(file_format_tmp, ".tiff".to_string(), "TIFF");
                    ui.selectable_value(file_format_tmp, ".tga".to_string(), "TGA");
                    ui.selectable_value(file_format_tmp, ".bmp".to_string(), "BMP");
                    ui.selectable_value(file_format_tmp, ".qoi".to_string(), "QOI");
                });

                ui.add_space(10.0);

                Grid::new("123").show(ui, |ui| {
                    ui.label("DEFAULT PATH");
                    ui.end_row();

                    let button_text1 = "Choose default path";
                    let button_text2 = "Change default path";
                    //let button_text1 = "Choose file name";
                    //let button_text2 = "Change file name";
                    let button_text = if *save_path_tmp == PathBuf::default() {button_text1} else {button_text2};

                    if ui.button(button_text).clicked(){
                        let p = FileDialog::new().set_directory("/").pick_folder();
                        if(p.is_none()) { }
                        else{
                            *save_path_tmp=p.unwrap();
                        }                         
                    }

                    ui.end_row();
                    ui.end_row();

                    ui.label("CHOOSE FILE NAME");
                    ui.end_row();
                    ui.add(egui::TextEdit::singleline(name_convention_tmp));
                    
                    ui.end_row();
                    ui.end_row();

                    let display_infos: Vec<display_info::DisplayInfo> = screenshots::display_info::DisplayInfo::all().unwrap();

                    if display_infos.len() == 1{
                        let text = format!("Monitor {} is being used", (*monitor_used + 1));
                        ui.label(text);
                    }

                    else{                        
                        egui::ComboBox::from_label("Choose monitor")
                        .selected_text(
                            if *monitor_used_tmp != 9999 {
                                format!("{}", (*monitor_used_tmp + 1))
                            } else {
                                String::from("All")
                            }
                        )
                        .show_ui(ui, |ui| {
                            for (i, _)  in display_infos.iter().enumerate(){
                                ui.selectable_value(monitor_used_tmp, i, (i+1).to_string());
                            }
                            //the following one is used in case i want a screenshot af all the screens
                            ui.selectable_value(monitor_used_tmp, 9999, "All".to_string());
                        });
                    }
                });

                ui.add_space(30.0);

                if ui.button("Chiudi").clicked(){
                    *save_path_tmp = save_path.clone();
                    *file_format_tmp = file_format.clone();
                    *name_convention_tmp = name_convention.clone();
                    *monitor_used_tmp = *monitor_used;
                    *schermata = Schermata::Home;
                }

                ui.set_enabled((*save_path != save_path_tmp.clone()) || (*file_format != file_format_tmp.clone()) || (*name_convention != *name_convention_tmp) || (*monitor_used != *monitor_used_tmp));

                if ui.button("Salva modifiche").clicked(){
                    *save_path = save_path_tmp.clone();
                    *file_format = file_format_tmp.clone(); 
                    *name_convention = name_convention_tmp.clone();
                    *monitor_used = *monitor_used_tmp;

                    *update_file = true; //in order to update the default initial settings
                    *schermata = Schermata::Home; 
                }
            });
}


fn set_image_gui_visible (window_size :egui::Vec2, prop :f32) -> egui::Vec2 {
    let mut  size = egui::Vec2::new(0.0, 0.0);
    size.x = window_size.x * 0.8;
    size.y = size.x / prop;
    if size.y >= window_size.y * 0.8 {
        size.y = window_size.y * 0.8;
        size.x = size.y * prop;
    }
    size
}

pub fn hotkey_to_String(modifier: Modifiers, key: Code) -> String{
    let mut mystr = String::from("");

    match modifier {
        Modifiers::ALT => mystr.push_str("ALT + "),
        Modifiers::CONTROL => mystr.push_str("CONTROL + "), 
        Modifiers::SHIFT => mystr.push_str("SHIFT + "),
        _ => mystr.push_str(""),
    }

    match key {
        Code::KeyA => mystr.push_str("A"),
        Code::KeyB => mystr.push_str("B"),
        Code::KeyC => mystr.push_str("C"),
        Code::KeyD => mystr.push_str("D"),
        Code::KeyE => mystr.push_str("E"),
        Code::KeyF => mystr.push_str("F"),
        Code::KeyG => mystr.push_str("G"),
        Code::KeyH => mystr.push_str("H"),
        Code::KeyI => mystr.push_str("I"),
        Code::KeyJ => mystr.push_str("J"),
        Code::KeyK => mystr.push_str("K"),
        Code::KeyL => mystr.push_str("L"),
        Code::KeyM => mystr.push_str("M"),
        Code::KeyN => mystr.push_str("N"),
        Code::KeyO => mystr.push_str("O"),
        Code::KeyP => mystr.push_str("P"),
        Code::KeyQ => mystr.push_str("Q"),
        Code::KeyR => mystr.push_str("R"),
        Code::KeyS => mystr.push_str("S"),
        Code::KeyT => mystr.push_str("T"),
        Code::KeyU => mystr.push_str("U"),
        Code::KeyV => mystr.push_str("V"),
        Code::KeyW => mystr.push_str("W"),
        Code::KeyX => mystr.push_str("X"),
        Code::KeyY => mystr.push_str("Y"),
        Code::KeyZ => mystr.push_str("Z"),
        Code::F1 => mystr.push_str("F1"),
        Code::F2 => mystr.push_str("F2"),
        Code::F3 => mystr.push_str("F3"),
        Code::F5 => mystr.push_str("F5"),
        Code::F6 => mystr.push_str("F6"),
        Code::F7 => mystr.push_str("F7"),
        Code::F8 => mystr.push_str("F8"),
        Code::F9 => mystr.push_str("F9"),
        Code::F10 => mystr.push_str("F10"),
        Code::F11 => mystr.push_str("F11"),
        Code::F12 => mystr.push_str("F12"),
        _ => mystr.push_str(""),
    }

    return mystr;
}

pub fn String_to_hotkey(my_string: String) -> (Modifiers, Code){
    let mod_and_key: Vec<&str> = my_string.split("+").collect();
    let mut result : (Modifiers, Code) = (Modifiers::default(), Code::default());

    match mod_and_key[0].trim() {
        "ALT" => result.0 = Modifiers::ALT,
        "CONTROL" => result.0 = Modifiers::CONTROL, 
        "SHIFT" => result.0 = Modifiers::SHIFT,
        _ => panic!("miao"),
    }

    match mod_and_key[1].trim() {
        "A" => result.1 = Code::KeyA,
        "B" => result.1 = Code::KeyB,
        "C" => result.1 = Code::KeyC,
        "D" => result.1 = Code::KeyD,
        "E" => result.1 = Code::KeyE,
        "F" => result.1 = Code::KeyF,
        "G" => result.1 = Code::KeyG,
        "H" => result.1 = Code::KeyH,
        "I" => result.1 = Code::KeyI,
        "J" => result.1 = Code::KeyJ,
        "K" => result.1 = Code::KeyK,
        "L" => result.1 = Code::KeyL,
        "M" => result.1 = Code::KeyM,
        "N" => result.1 = Code::KeyN,
        "O" => result.1 = Code::KeyO,
        "P" => result.1 = Code::KeyP,
        "Q" => result.1 = Code::KeyQ,
        "R" => result.1 = Code::KeyR,
        "S" => result.1 = Code::KeyS,
        "T" => result.1 = Code::KeyT,
        "U" => result.1 = Code::KeyU,
        "V" => result.1 = Code::KeyV,
        "W" => result.1 = Code::KeyW,
        "X" => result.1 = Code::KeyX,
        "Y" => result.1 = Code::KeyY,
        "Z" => result.1 = Code::KeyZ,
        "F1" => result.1 = Code::KeyA,
        "F2" => result.1 = Code::KeyB,
        "F3" => result.1 = Code::KeyC,
        "F5" => result.1 = Code::KeyE,
        "F6" => result.1 = Code::KeyF,
        "F7" => result.1 = Code::KeyG,
        "F8" => result.1 = Code::KeyH,
        "F9" => result.1 = Code::KeyI,
        "F10" => result.1 = Code::KeyA,
        "F11" => result.1 = Code::KeyB,
        "F12" => result.1 = Code::KeyC,
        _ => panic!("miao"),
    }  

    return result;
}


pub fn setting_timer(ctx: &egui::Context, schermata: &mut Schermata, delay_duration : &mut Duration, delay_tmp : &mut u64){
    let window_size = egui::vec2(0.0, 0.0);

    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {

        menu::bar(ui, |ui| {
            ui.menu_button("Settings", |ui| {
                if ui.button("Custom Hotkey").clicked() {
                    *schermata = Schermata::Setting_Hotkey;
                }

                if ui.button("Saving settings").clicked() {
                    *schermata = Schermata::Setting_Saving;
                }

                if ui.button("Timer settings").clicked() {
                    *schermata = Schermata::Setting_Timer;
                }
            });
        });

        ui.add_space(20.0);
        
            egui::ComboBox::from_label("Choose a delay")
                .selected_text(format!("{}", delay_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(delay_tmp, 0, "0 sec");
                    ui.selectable_value(delay_tmp, 3, "3 sec");
                    ui.selectable_value(delay_tmp, 5, "5 sec");
                    ui.selectable_value(delay_tmp, 10, "10 sec");
                });

                ui.add_space(30.0);

                if ui.button("Chiudi").clicked(){
                    *delay_tmp = (*delay_duration).as_secs() as u64;
                    *schermata = Schermata::Home;
                }

                ui.set_enabled(*delay_duration != Duration::from_secs(*delay_tmp));

                if ui.button("Salva modifiche").clicked(){
                    *delay_duration = Duration::from_secs(*delay_tmp);
                    *schermata = Schermata::Home; 
                }
            });
}
