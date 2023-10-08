
use eframe::egui;
use ::egui::Vec2;

use egui::{menu, Button};
use image::RgbaImage;
use eframe::egui::TextureHandle;
use crate::Schermata;
use crate::screen;


pub fn home(ctx: &egui::Context, schermata: &mut Schermata, image: &mut RgbaImage, texture : &mut Option<TextureHandle>, frame: &mut eframe::Frame){
    //let mut texture_data : eframe::epaint::TextureHandle;
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {  
        menu::bar(ui, |ui| {
            ui.button("Setting");
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

    if texture.is_some() {
        
        
        ui.centered_and_justified(|ui| {
            println!("FIN {:?}",frame.info().window_info);
            println!("IMM {:?}",texture.as_ref().unwrap().size_vec2());
            //ui.add(egui::Image::new(texture.as_ref().unwrap().id(), texture.as_ref().unwrap().size_vec2()));
            println!("PROP finestra: {}, PROP Imm: {}",frame.info().window_info.size.x / frame.info().window_info.size.y,texture.as_ref().unwrap().size_vec2().x / texture.as_ref().unwrap().size_vec2().y);
            //if frame.info().window_info.size.x / frame.info().window_info.size.y == texture.as_ref().unwrap().size_vec2().x / texture.as_ref().unwrap().size_vec2().y {
                ui.add(egui::Image::new(texture.as_ref().unwrap().id(), set_image_gui_visible(frame.info().window_info.size,texture.as_ref().unwrap().size_vec2().x / texture.as_ref().unwrap().size_vec2().y)));
            //}
                
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