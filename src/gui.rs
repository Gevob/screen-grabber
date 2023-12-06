
use std::collections::HashSet;
use std::path::PathBuf;
use egui::Image;
use egui::{menu, Button, Color32};
use image::DynamicImage;
use egui::*;
use image::ImageBuffer;
use image::ImageOutputFormat;
use image::RgbaImage;
use image::GenericImageView;
//use eframe::egui;
//use eframe::egui::TextureHandle;
use crate::Schermata;
use crate::screen;
use crate::MyGlobalHotKeyManager;
use global_hotkey::hotkey::{HotKey, Code, Modifiers};
use egui::{Grid, Stroke, Ui, Visuals, Label};
use image::{save_buffer, ImageFormat, ColorType};
use rfd::FileDialog;
use chrono::prelude::*;
use std::io::Cursor;
use image::io::Reader as ImageReader;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use arboard::{Clipboard, ImageData};


pub fn home(ctx: &egui::Context, schermata: &mut Schermata, image: &mut RgbaImage, texture : &mut Option<TextureHandle>, hotkeys_list: &mut Vec<(Modifiers, Code, String)>, file_format: &mut String, save_path: &mut PathBuf, name_convention: &mut String){
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            menu::bar(ui, |ui| {

                ui.menu_button("Settings", |ui| {
                    if ui.button("Custom Hotkey").on_hover_text("Customize your Hotkeys").clicked() {
                        *schermata = Schermata::Setting_Hotkey;
                    }

                    if ui.button("Saving settings").on_hover_text("Customize default saving options").clicked() {
                        *schermata = Schermata::Setting_Saving;
                    }
                }).response.on_hover_text("Change your Settings");; //.on_hover_text("Take a Screenshot");



                if ui.button("Screenshots").on_hover_text("Take a Screenshot").clicked() {
                    *image = screen::screenshot().unwrap();
                    let flat_image = image.as_flat_samples();
                    let color_image2 = egui::ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize],flat_image.samples);
                    let image_data = egui::ImageData::from(color_image2);
                    *texture = Some(ui.ctx().load_texture("screen", image_data, Default::default()));
                    *schermata = Schermata::Edit;
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
                    

                });
            });
    });    
}

pub fn edit(ctx: &egui::Context, stroke: &mut Stroke, texture : &mut Option<TextureHandle>, frame: &mut eframe::Frame, points: &mut Vec<Vec<Pos2>>, schermata: &mut Schermata, rgba_image: &mut RgbaImage, file_format: &mut String, save_path: &mut PathBuf, name_convention: &mut String){
    //sleep(Duration::from_millis(200));
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {  
        menu::bar(ui, |ui| { 
            ui.color_edit_button_srgba(&mut stroke.color);
            ui.add(egui::Slider::new(&mut stroke.width, 1.0..=8.0).integer());   

            if ui.button("Discard").clicked() {
                *schermata = Schermata::Home;
                //elimina anche gli edit
                *texture = None; //e setta a null la textureHandle
            }

            if ui.button("Save").clicked(){
                let now = Utc::now();
                let ts = now.timestamp(); //add timestamp in the name convention, in order to have unique files

                // Save the DynamicImage to a file
                let dynamic_image = DynamicImage::ImageRgba8(rgba_image.clone());                
                if(*save_path != PathBuf::default()) {
                    let output_path = format!("{}\\{}_{}{}", save_path.clone().into_os_string().into_string().unwrap(), name_convention, ts, file_format);
                    dynamic_image.save_with_format(output_path, ImageFormat::Jpeg).expect("Failed to save image");
                }
                else {
                    let p = FileDialog::new().set_directory("/").pick_folder();
                    if(p.is_none()) { }
                    else{
                        let mut path_tmp = p.unwrap();
                        let output_path = format!("{}\\{}_{}{}", path_tmp.clone().into_os_string().into_string().unwrap(), name_convention, ts, file_format);
                        dynamic_image.save_with_format(output_path, ImageFormat::Jpeg).expect("Failed to save image");
                    }   
                }
            }

            if ui.button("Copy").on_hover_text("Copy the Screenshot to Clipboard").clicked() {
                // Copy the image to the clipboard
                let mut ctx_clip = Clipboard::new().unwrap();
                let clipboard_image = DynamicImage::ImageRgba8(rgba_image.clone());
                let image_bytes = clipboard_image.into_bytes();
                #[rustfmt::skip]
                let img_data = ImageData { width: rgba_image.width() as usize, height: rgba_image.height() as usize, bytes: image_bytes.into() };
                ctx_clip.set_image(img_data).unwrap();
            }
        });

        ui.add_space(30.0);
        if !(texture.is_none()) { 
            ui.centered_and_justified(|ui| {
                let mut edited_image = Image::new(texture.as_ref().unwrap()).max_size(ui.available_size()).maintain_aspect_ratio(true).ui(ui);
                    let texture_rect = egui::Rect::from_min_size(Pos2::ZERO, texture.clone().unwrap().size_vec2()); //rettangolo della dimensione dell'immagine
                    let screen_rect = eframe::emath::RectTransform::from_to(texture_rect,edited_image.rect);
                    //let response = ui.add(edited_image.);
                    //let image_center = response.rect.center();
                    let area_pos = egui::pos2(0.0, 32.0);
                    egui::Area::new("my_area")
                        .default_pos(area_pos)
                        .show(ui.ctx(), |ui| {
                            screen::ui_content(ui, points, stroke, edited_image.clone(), edited_image.rect.size());
                        });
            });
        }
    });
}

pub fn setting_hotkey(ctx: &egui::Context, schermata: &mut Schermata, manager: &mut MyGlobalHotKeyManager, modifier_copy: &mut Modifiers, key_copy: &mut Code, modifier_screen: &mut Modifiers, key_screen: &mut Code, modifier_save: &mut Modifiers, key_save: &mut Code, hotkeys_list: &mut Vec<(Modifiers, Code, String)>, modifier_copy_tmp: &mut Modifiers, key_copy_tmp: &mut Code, modifier_screen_tmp: &mut Modifiers, key_screen_tmp: &mut Code, modifier_save_tmp: &mut Modifiers, key_save_tmp: &mut Code){
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

                egui::ComboBox::from_id_source("Choose Key copy")
                .selected_text(format!("{:?}", key_copy_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(key_copy_tmp, Code::KeyA, "KeyA");
                    ui.selectable_value(key_copy_tmp, Code::KeyB, "KeyB");
                    ui.selectable_value(key_copy_tmp, Code::KeyC, "KeyC");
                    ui.selectable_value(key_copy_tmp, Code::KeyD, "KeyD");
                    ui.selectable_value(key_copy_tmp, Code::KeyE, "KeyE");
                    ui.selectable_value(key_copy_tmp, Code::KeyF, "KeyF");
                    ui.selectable_value(key_copy_tmp, Code::KeyG, "KeyG");
                    ui.selectable_value(key_copy_tmp, Code::KeyH, "KeyH");
                    ui.selectable_value(key_copy_tmp, Code::KeyI, "KeyI");
                    ui.selectable_value(key_copy_tmp, Code::KeyJ, "KeyJ");
                    ui.selectable_value(key_copy_tmp, Code::KeyK, "KeyK");
                    ui.selectable_value(key_copy_tmp, Code::KeyL, "KeyL");
                    ui.selectable_value(key_copy_tmp, Code::KeyM, "KeyM");
                    ui.selectable_value(key_copy_tmp, Code::KeyN, "KeyN");
                    ui.selectable_value(key_copy_tmp, Code::KeyO, "KeyO");
                    ui.selectable_value(key_copy_tmp, Code::KeyP, "KeyP");
                    ui.selectable_value(key_copy_tmp, Code::KeyQ, "KeyQ");
                    ui.selectable_value(key_copy_tmp, Code::KeyR, "KeyR");
                    ui.selectable_value(key_copy_tmp, Code::KeyS, "KeyS");
                    ui.selectable_value(key_copy_tmp, Code::KeyT, "KeyT");
                    ui.selectable_value(key_copy_tmp, Code::KeyU, "KeyU");
                    ui.selectable_value(key_copy_tmp, Code::KeyV, "KeyV");
                    ui.selectable_value(key_copy_tmp, Code::KeyW, "KeyW");
                    ui.selectable_value(key_copy_tmp, Code::KeyX, "KeyX");
                    ui.selectable_value(key_copy_tmp, Code::KeyY, "KeyY");
                    ui.selectable_value(key_copy_tmp, Code::KeyZ, "KeyZ");
                    ui.selectable_value(key_copy_tmp, Code::F1, "F1");
                    ui.selectable_value(key_copy_tmp, Code::F2, "F2");
                    ui.selectable_value(key_copy_tmp, Code::F3, "F3");
                    ui.selectable_value(key_copy_tmp, Code::F5, "F5");
                    ui.selectable_value(key_copy_tmp, Code::F6, "F6");
                    ui.selectable_value(key_copy_tmp, Code::F7, "F7");
                    ui.selectable_value(key_copy_tmp, Code::F8, "F8");
                    ui.selectable_value(key_copy_tmp, Code::F9, "F9");
                    ui.selectable_value(key_copy_tmp, Code::F10, "F10");
                    ui.selectable_value(key_copy_tmp, Code::F11, "F11");
                    ui.selectable_value(key_copy_tmp, Code::F12, "F12");
                    //... aggiungere altre keys nel caso sia necessario ...
                });

                ui.end_row();

                ui.label("SCREEN ");

                egui::ComboBox::from_id_source("Choose modifier screen")
                .selected_text(format!("{:?}", modifier_screen_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(modifier_screen_tmp, Modifiers::CONTROL, "Ctrl");
                    ui.selectable_value(modifier_screen_tmp, Modifiers::SHIFT, "Shift");
                    ui.selectable_value(modifier_screen_tmp, Modifiers::ALT, "Alt");
                });

                egui::ComboBox::from_id_source("Choose Key screen")
                .selected_text(format!("{:?}", key_screen_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(key_screen_tmp, Code::KeyA, "KeyA");
                    ui.selectable_value(key_screen_tmp, Code::KeyB, "KeyB");
                    ui.selectable_value(key_screen_tmp, Code::KeyC, "KeyC");
                    ui.selectable_value(key_screen_tmp, Code::KeyD, "KeyD");
                    ui.selectable_value(key_screen_tmp, Code::KeyE, "KeyE");
                    ui.selectable_value(key_screen_tmp, Code::KeyF, "KeyF");
                    ui.selectable_value(key_screen_tmp, Code::KeyG, "KeyG");
                    ui.selectable_value(key_screen_tmp, Code::KeyH, "KeyH");
                    ui.selectable_value(key_screen_tmp, Code::KeyI, "KeyI");
                    ui.selectable_value(key_screen_tmp, Code::KeyJ, "KeyJ");
                    ui.selectable_value(key_screen_tmp, Code::KeyK, "KeyK");
                    ui.selectable_value(key_screen_tmp, Code::KeyL, "KeyL");
                    ui.selectable_value(key_screen_tmp, Code::KeyM, "KeyM");
                    ui.selectable_value(key_screen_tmp, Code::KeyN, "KeyN");
                    ui.selectable_value(key_screen_tmp, Code::KeyO, "KeyO");
                    ui.selectable_value(key_screen_tmp, Code::KeyP, "KeyP");
                    ui.selectable_value(key_screen_tmp, Code::KeyQ, "KeyQ");
                    ui.selectable_value(key_screen_tmp, Code::KeyR, "KeyR");
                    ui.selectable_value(key_screen_tmp, Code::KeyS, "KeyS");
                    ui.selectable_value(key_screen_tmp, Code::KeyT, "KeyT");
                    ui.selectable_value(key_screen_tmp, Code::KeyU, "KeyU");
                    ui.selectable_value(key_screen_tmp, Code::KeyV, "KeyV");
                    ui.selectable_value(key_screen_tmp, Code::KeyW, "KeyW");
                    ui.selectable_value(key_screen_tmp, Code::KeyX, "KeyX");
                    ui.selectable_value(key_screen_tmp, Code::KeyY, "KeyY");
                    ui.selectable_value(key_screen_tmp, Code::KeyZ, "KeyZ");
                    ui.selectable_value(key_screen_tmp, Code::F1, "F1");
                    ui.selectable_value(key_screen_tmp, Code::F2, "F2");
                    ui.selectable_value(key_screen_tmp, Code::F3, "F3");
                    ui.selectable_value(key_screen_tmp, Code::F5, "F5");
                    ui.selectable_value(key_screen_tmp, Code::F6, "F6");
                    ui.selectable_value(key_screen_tmp, Code::F7, "F7");
                    ui.selectable_value(key_screen_tmp, Code::F8, "F8");
                    ui.selectable_value(key_screen_tmp, Code::F9, "F9");
                    ui.selectable_value(key_screen_tmp, Code::F10, "F10");
                    ui.selectable_value(key_screen_tmp, Code::F11, "F11");
                    ui.selectable_value(key_screen_tmp, Code::F12, "F12");
                    //... aggiungere altre keys nel caso sia necessario ...
                });

                ui.end_row();

                ui.label("SAVE ");

                egui::ComboBox::from_id_source("Choose modifier save")
                .selected_text(format!("{:?}", modifier_save_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(modifier_save_tmp, Modifiers::CONTROL, "Ctrl");
                    ui.selectable_value(modifier_save_tmp, Modifiers::SHIFT, "Shift");
                    ui.selectable_value(modifier_save_tmp, Modifiers::ALT, "Alt");
                });

                egui::ComboBox::from_id_source("Choose Key save")
                .selected_text(format!("{:?}", key_save_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(key_save_tmp, Code::KeyA, "KeyA");
                    ui.selectable_value(key_save_tmp, Code::KeyB, "KeyB");
                    ui.selectable_value(key_save_tmp, Code::KeyC, "KeyC");
                    ui.selectable_value(key_save_tmp, Code::KeyD, "KeyD");
                    ui.selectable_value(key_save_tmp, Code::KeyE, "KeyE");
                    ui.selectable_value(key_save_tmp, Code::KeyF, "KeyF");
                    ui.selectable_value(key_save_tmp, Code::KeyG, "KeyG");
                    ui.selectable_value(key_save_tmp, Code::KeyH, "KeyH");
                    ui.selectable_value(key_save_tmp, Code::KeyI, "KeyI");
                    ui.selectable_value(key_save_tmp, Code::KeyJ, "KeyJ");
                    ui.selectable_value(key_save_tmp, Code::KeyK, "KeyK");
                    ui.selectable_value(key_save_tmp, Code::KeyL, "KeyL");
                    ui.selectable_value(key_save_tmp, Code::KeyM, "KeyM");
                    ui.selectable_value(key_save_tmp, Code::KeyN, "KeyN");
                    ui.selectable_value(key_save_tmp, Code::KeyO, "KeyO");
                    ui.selectable_value(key_save_tmp, Code::KeyP, "KeyP");
                    ui.selectable_value(key_save_tmp, Code::KeyQ, "KeyQ");
                    ui.selectable_value(key_save_tmp, Code::KeyR, "KeyR");
                    ui.selectable_value(key_save_tmp, Code::KeyS, "KeyS");
                    ui.selectable_value(key_save_tmp, Code::KeyT, "KeyT");
                    ui.selectable_value(key_save_tmp, Code::KeyU, "KeyU");
                    ui.selectable_value(key_save_tmp, Code::KeyV, "KeyV");
                    ui.selectable_value(key_save_tmp, Code::KeyW, "KeyW");
                    ui.selectable_value(key_save_tmp, Code::KeyX, "KeyX");
                    ui.selectable_value(key_save_tmp, Code::KeyY, "KeyY");
                    ui.selectable_value(key_save_tmp, Code::KeyZ, "KeyZ");
                    ui.selectable_value(key_save_tmp, Code::F1, "F1");
                    ui.selectable_value(key_save_tmp, Code::F2, "F2");
                    ui.selectable_value(key_save_tmp, Code::F3, "F3");
                    ui.selectable_value(key_save_tmp, Code::F5, "F5");
                    ui.selectable_value(key_save_tmp, Code::F6, "F6");
                    ui.selectable_value(key_save_tmp, Code::F7, "F7");
                    ui.selectable_value(key_save_tmp, Code::F8, "F8");
                    ui.selectable_value(key_save_tmp, Code::F9, "F9");
                    ui.selectable_value(key_save_tmp, Code::F10, "F10");
                    ui.selectable_value(key_save_tmp, Code::F11, "F11");
                    ui.selectable_value(key_save_tmp, Code::F12, "F12");
                    //... aggiungere altre keys nel caso sia necessario ...
                });

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

            ui.set_enabled(all_distinct && ((*modifier_copy != *modifier_copy_tmp) || (*modifier_screen != *modifier_screen_tmp) || (*modifier_save != *modifier_save_tmp) || (*key_copy != *key_copy_tmp) || (*key_screen != *key_screen_tmp) || (*key_save != *key_save_tmp)));
            
            if ui.button("Salva modifiche").clicked() {
                *modifier_copy = *modifier_copy_tmp;
                *modifier_save = *modifier_save_tmp;
                *modifier_screen = *modifier_screen_tmp;
                *key_copy = *key_copy_tmp;
                *key_screen = *key_screen_tmp;
                *key_save = *key_save_tmp;

                //genera la hotkey
                if all_distinct {
                    for el in hotkeys_list.iter_mut(){
                        if el.2 == "Copy".to_string(){
                            if el.0 != *modifier_copy || el.1 != *key_copy{
                                let mut hotkey_copy = HotKey::new(Some(*modifier_copy), *key_copy);
                                let mut hotkey_to_delete = HotKey::new(Some(el.0), el.1);
                                ((*manager).0).unregister(hotkey_to_delete).unwrap();
                                ((*manager).0).register(hotkey_copy).unwrap(); //ho fatto in questo modo perchè GlobalHotKeyManager non aveva il tratto Default
    
                                el.0 = *modifier_copy;
                                el.1 = *key_copy;
                            }
                        }
                        else if el.2 == "Screen".to_string(){
                            if el.0 != *modifier_screen || el.1 != *key_screen{
                                let mut hotkey_screen = HotKey::new(Some(*modifier_screen), *key_screen);
                                let mut hotkey_to_delete = HotKey::new(Some(el.0), el.1);
                                ((*manager).0).unregister(hotkey_to_delete).unwrap();
                                ((*manager).0).register(hotkey_screen).unwrap(); //ho fatto in questo modo perchè GlobalHotKeyManager non aveva il tratto Default
    
                                el.0 = *modifier_screen;
                                el.1 = *key_screen;
                            }
                        }
                        else { //if el.2 == "Save".to_string()
                            if el.0 != *modifier_save || el.1 != *key_save{
                                let mut hotkey_save = HotKey::new(Some(*modifier_save), *key_save);
                                let mut hotkey_to_delete = HotKey::new(Some(el.0), el.1);
                                ((*manager).0).unregister(hotkey_to_delete).unwrap();
                                ((*manager).0).register(hotkey_save).unwrap(); //ho fatto in questo modo perchè GlobalHotKeyManager non aveva il tratto Default
    
                                el.0 = *modifier_save;
                                el.1 = *key_save;
                            }
                        }
                    }
                    *schermata = Schermata::Home; 
                }
            }
        });
}

pub fn setting_saving(ctx: &egui::Context, schermata: &mut Schermata, file_format: &mut String, save_path: &mut PathBuf, file_format_tmp: &mut String, save_path_tmp: &mut PathBuf, name_convention: &mut String, name_convention_tmp: &mut String){
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
            });
        });

        ui.add_space(20.0);
        
            egui::ComboBox::from_label("Choose format")
                .selected_text(format!("{}", file_format_tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(file_format_tmp, ".png".to_string(), "PNG");
                    ui.selectable_value(file_format_tmp, ".jpeg".to_string(), "JPEG");
                    ui.selectable_value(file_format_tmp, ".gif".to_string(), "GIF");
                    ui.selectable_value(file_format_tmp, ".webp".to_string(), "WEBP");
                    ui.selectable_value(file_format_tmp, ".pnm".to_string(), "PNM");
                    ui.selectable_value(file_format_tmp, ".tiff".to_string(), "TIFF");
                    ui.selectable_value(file_format_tmp, ".tga".to_string(), "TGA");
                    ui.selectable_value(file_format_tmp, ".dds".to_string(), "DDS");
                    ui.selectable_value(file_format_tmp, ".bmp".to_string(), "BMP");
                    ui.selectable_value(file_format_tmp, ".ico".to_string(), "ICO");
                    ui.selectable_value(file_format_tmp, ".hdr".to_string(), "HDR");
                    ui.selectable_value(file_format_tmp, ".openexr".to_string(), "OPENEXR");
                    ui.selectable_value(file_format_tmp, ".farbfeld".to_string(), "FARBFELD");
                    ui.selectable_value(file_format_tmp, ".avif".to_string(), "AVIF");
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
                    //aggiungere la parte relativa alle convenzioni sul nome del file da salvare (con auto incremento)
                });

                ui.add_space(30.0);

                if ui.button("Chiudi").clicked(){
                    *save_path_tmp = save_path.clone();
                    *file_format_tmp = file_format.clone();
                    *name_convention_tmp = name_convention.clone();
                    *schermata = Schermata::Home;
                }

                ui.set_enabled((*save_path != save_path_tmp.clone()) || (*file_format != file_format_tmp.clone()) || (*name_convention != *name_convention_tmp));

                if ui.button("Salva modifiche").clicked(){
                    *save_path = save_path_tmp.clone();
                    *file_format = file_format_tmp.clone(); 
                    *name_convention = name_convention_tmp.clone();
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

fn hotkey_to_String(modifier: Modifiers, key: Code) -> String{
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

