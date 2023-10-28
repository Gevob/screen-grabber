
use eframe::egui;
use ::egui::Vec2;

use egui::{menu, Button};
use image::RgbaImage;
use eframe::egui::Pos2;
use eframe::egui::TextureHandle;
use crate::Schermata;
use crate::screen;
use crate::MyGlobalHotKeyManager;
use global_hotkey::hotkey::{HotKey, Code, Modifiers};
use egui::{Color32, Stroke, Ui, Visuals};


pub fn home(ctx: &egui::Context, schermata: &mut Schermata, image: &mut RgbaImage, texture : &mut Option<TextureHandle>,  is_popup_open: &mut bool, manager: &mut MyGlobalHotKeyManager, modifier: &mut Modifiers, key: &mut Code, frame: &mut eframe::Frame, stroke: &mut Stroke, points: &mut Vec<Vec<Pos2>>){
    //let mut texture_data : eframe::epaint::TextureHandle;
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
        ui.set_enabled(!(*is_popup_open)); 
        menu::bar(ui, |ui| {
            ui.menu_button("Settings", |ui| {
                if ui.button("Custom Hotkey").clicked() {
                    *is_popup_open = true;
                }
            });
            /*    
            let image_data = include_bytes!("./images/marker.png");
            let image2 = image::load_from_memory(image_data).expect("Failed to load image");
            let image_buffer = image2.to_rgba8();
            let flat_image2 = image_buffer.as_flat_samples();
            let color_image2 = egui::ColorImage::from_rgba_unmultiplied([image2.width() as usize, image2.height() as usize],flat_image2.samples);
            let image_data2 = egui::ImageData::from(color_image2);
            let texture2 = ui.ctx().load_texture("screen", image_data2, Default::default());
            ui.add(eframe::egui::Button::image_and_text(texture2.id(), [16.0,16.0], ""));
                                */
            if texture.is_some() {
                           ui.color_edit_button_srgba(&mut stroke.color);
                           ui.add(eframe::egui::Slider::new(&mut stroke.width, 1.0..=8.0).integer());
                           
            }
            //ui.add_space(frame.info().window_info.size.x * 0.45);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                ui.button("Salva");
                ui.vertical_centered(|ui| {
                    if ui.button("Screenshots").clicked() {
                        *image = screen::screenshot().unwrap();
                        let flat_image = image.as_flat_samples();
                        let color_image2 = egui::ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize],flat_image.samples);
                        let image_data = egui::ImageData::from(color_image2);
                        *texture = Some(ui.ctx().load_texture("screen", image_data, Default::default()));
                    }
                });
            });
        });    
    //ui.vertical_centered(|ui| {
      //  let (_id,rect) = ui.allocate_space(egui::vec2(70.0, 40.0));
        
            //if ui.button("Nuovo").clicked() {
                // Azione da intraprendere quando il pulsante viene premuto
            //}
       // if ui.put(rect, egui::Button::new("Screenshot")).clicked() {
         //   *image = screen::screenshot().unwrap();
           // let flat_image = image.as_flat_samples();
           // let color_image2 = egui::ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize],flat_image.samples);
            //let color_image = egui::ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize], image.as_ref());
            //let image_data = egui::ImageData::from(color_image2);
            //let option = egui::TextureOptions::LINEAR;
            //let texture_data  = eframe::egui::Context::default().load_texture( "image",image_data, option);
            //*texture = Some(texture_data);
            //ui.image(texture_data.id(), texture_data.size_vec2());
            //*texture = Some(ui.ctx().load_texture("screen", image_data, Default::default()));
        //}
        
    //});

    if *is_popup_open {
        show_popup(is_popup_open, manager, modifier, key, ctx);
    }

    if texture.is_some() {
        
        
        ui.centered_and_justified(|ui| {
                        ui.add(egui::Image::new(texture.as_ref().unwrap().id(), set_image_gui_visible(frame.info().window_info.size,texture.as_ref().unwrap().size_vec2().x / texture.as_ref().unwrap().size_vec2().y))); 
                        egui::Area::new("my_area")
                        .default_pos(egui::pos2(0.0, 32.0))
                        .show(ctx, |ui| {
                        screen::ui_content(ui, points, stroke);
                        });
                
        });
        
        //ui.image(texture.as_ref().unwrap().id(),egui::vec2(900.0, 600.0));// per qualche strano motivo non si vede
        
       
        
    }
    if texture.is_none() {
        ui.centered_and_justified(|ui| {
            ui.label("per i dev,aggiungere hotkey list al posto di quello che stai leggendo");
        });
        //ui.label("per i dev,aggiungere hotkey list al posto di quello che stai leggendo");
        
    }
});
}


fn set_image_gui_visible (window_size :eframe::egui::Vec2, prop :f32) -> eframe::egui::Vec2 {
    let mut  size = eframe::egui::Vec2::new(0.0, 0.0);
    size.x = window_size.x * 0.8;
    size.y = size.x / prop;
    //println!("BEFORE SECOND IF: {:?}",size);
    if size.y >= window_size.y * 0.8 {
        size.y = window_size.y * 0.8;
        size.x = size.y * prop;
        //println!("AFTERl SECOND IF: {:?}",size);
    }
    size
}

pub fn edit(ctx: &egui::Context){
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {      
    
});
}

fn show_popup(is_window_open: &mut bool, manager: &mut MyGlobalHotKeyManager, modifier: &mut Modifiers, key: &mut Code, ctx: &egui::Context) {
    let window_size = egui::vec2(0.0, 0.0);

    if *modifier == Modifiers::default() || *key == Code::default() { //setta la shortcut de default (scelta da noi)
        *modifier = Modifiers::CONTROL;
        *key = Code::KeyA;
    }

    egui::Window::new("Custom Window")
        .anchor(egui::Align2::CENTER_CENTER, window_size)
        .show(ctx, |ui| {
            egui::ComboBox::from_label("Choose modifier")
                .selected_text(format!("{:?}", modifier))
                .show_ui(ui, |ui| {
                    ui.selectable_value(modifier, Modifiers::CONTROL, "Ctrl");
                    ui.selectable_value(modifier, Modifiers::SHIFT, "Shift");
                    ui.selectable_value(modifier, Modifiers::ALT, "Alt");
                });

            egui::ComboBox::from_label("Choose Key")
                .selected_text(format!("{:?}", key))
                .show_ui(ui, |ui| {
                    ui.selectable_value(key, Code::KeyA, "KeyA");
                    ui.selectable_value(key, Code::KeyB, "KeyB");
                    ui.selectable_value(key, Code::KeyC, "KeyC");
                    ui.selectable_value(key, Code::KeyD, "KeyD");
                    ui.selectable_value(key, Code::KeyE, "KeyE");
                    ui.selectable_value(key, Code::KeyF, "KeyF");
                    ui.selectable_value(key, Code::KeyG, "KeyG");
                    ui.selectable_value(key, Code::KeyH, "KeyH");
                    ui.selectable_value(key, Code::KeyI, "KeyI");
                    ui.selectable_value(key, Code::KeyJ, "KeyJ");
                    ui.selectable_value(key, Code::KeyK, "KeyK");
                    ui.selectable_value(key, Code::KeyL, "KeyL");
                    ui.selectable_value(key, Code::KeyM, "KeyM");
                    ui.selectable_value(key, Code::KeyN, "KeyN");
                    ui.selectable_value(key, Code::KeyO, "KeyO");
                    ui.selectable_value(key, Code::KeyP, "KeyP");
                    ui.selectable_value(key, Code::KeyQ, "KeyQ");
                    ui.selectable_value(key, Code::KeyR, "KeyR");
                    ui.selectable_value(key, Code::KeyS, "KeyS");
                    ui.selectable_value(key, Code::KeyT, "KeyT");
                    ui.selectable_value(key, Code::KeyU, "KeyU");
                    ui.selectable_value(key, Code::KeyV, "KeyV");
                    ui.selectable_value(key, Code::KeyW, "KeyW");
                    ui.selectable_value(key, Code::KeyX, "KeyX");
                    ui.selectable_value(key, Code::KeyY, "KeyY");
                    ui.selectable_value(key, Code::KeyZ, "KeyZ");
                    ui.selectable_value(key, Code::F1, "F1");
                    ui.selectable_value(key, Code::F2, "F2");
                    ui.selectable_value(key, Code::F3, "F3");
                    ui.selectable_value(key, Code::F4, "F4");
                    ui.selectable_value(key, Code::F5, "F5");
                    ui.selectable_value(key, Code::F6, "F6");
                    ui.selectable_value(key, Code::F7, "F7");
                    ui.selectable_value(key, Code::F8, "F8");
                    ui.selectable_value(key, Code::F9, "F9");
                    ui.selectable_value(key, Code::F10, "F10");
                    ui.selectable_value(key, Code::F11, "F11");
                    ui.selectable_value(key, Code::F12, "F12");
                    //... aggiungere altre keys nel caso sia necessario ...
                });
            
            let mut save_button = Button::new("Click me").fill(Color32::LIGHT_BLUE);

            if ui.add(save_button).clicked() {
                //genera la hotkey
                let hotkey = HotKey::new(Some(*modifier), *key);
                //e poi la registri
                ((*manager).0).register(hotkey).unwrap(); //ho fatto in questo modo perchè GlobalHotKeyManager didn't have the Default trait
                *is_window_open = false;             
            }
        });
}
