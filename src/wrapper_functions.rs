use std::path::PathBuf;

use arboard::{Clipboard, ImageData as OtherImageData};
use chrono::Utc;
use egui::{Ui};
use global_hotkey::hotkey::Code;
use image::{DynamicImage, RgbImage, RgbaImage, ImageFormat, Rgba};
use rfd::FileDialog;

use crate::draws_functions::{Circle, Draws};


pub fn copy_to_clipboard(image: &RgbaImage){
    let mut ctx_clip = Clipboard::new().unwrap();
    let clipboard_image = DynamicImage::ImageRgba8(image.clone());
    let image_bytes = clipboard_image.into_bytes();
    #[rustfmt::skip]
    let img_data = OtherImageData { width: image.width() as usize, height: image.height() as usize, bytes: image_bytes.into() };
    ctx_clip.set_image(img_data).unwrap();
}

pub fn save_image(rgba_image: &mut RgbaImage, save_path: &PathBuf, name_convention: &String, file_format: &String, draws: &mut Vec<Draws>){
    let now = Utc::now();
    let ts = now.timestamp(); //add timestamp in the name convention, in order to have unique files

    // Save the DynamicImage to a file
    draw_on_image(draws, rgba_image);
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


pub fn show_combo_box(ui: &mut Ui, key: &mut Code, id_combo_box: String){
    egui::ComboBox::from_id_source(id_combo_box)
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
}

fn draw_circle_on_image(circle: &Circle, image: &mut RgbaImage) {
    // Implement drawing a stroked circumference (outline) of a circle on the image
    let center = circle.center.round();
    let radius = circle.radius;
    let stroke_width = circle.stroke.width as f64;

    // Draw only the pixels along the circumference
    let mut theta = 0.0;
    while theta <= 2.0 * std::f64::consts::PI {
        let x = (center.x + radius * theta.cos() as f32).round() as i32;
        let y = (center.y + radius * theta.sin() as f32).round() as i32;

        for i in -(stroke_width / 2.0).ceil() as i32..=(stroke_width / 2.0).floor() as i32 {
            let x_stroke = (center.x + (radius + i as f32) * theta.cos() as f32).round() as i32;
            let y_stroke = (center.y + (radius + i as f32) * theta.sin() as f32).round() as i32;

            if x_stroke >= 0 && x_stroke < image.width() as i32 && y_stroke >= 0 && y_stroke < image.height() as i32 {
                image.put_pixel(x_stroke as u32, y_stroke as u32, Rgba(circle.stroke.color.to_array()));
            }
        }

        theta += 0.0001; // Adjust this value for a smoother or coarser circumference
    }
}



fn draw_on_image(draws: &Vec<Draws>, image: &mut RgbaImage) {
    for el in draws.iter(){
        match el{
            Draws::Circle(circle) => draw_circle_on_image(circle, image),
            _ => println!("miao"),
        }
        
    }
}
