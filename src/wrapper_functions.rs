use std::{path::PathBuf, sync::Arc, os::windows::io::OwnedHandle};

use arboard::{Clipboard, ImageData as OtherImageData};
use chrono::Utc;
use egui::{Ui, Color32, Painter, Pos2, Image};
use global_hotkey::hotkey::Code;
use image::{DynamicImage, RgbImage, RgbaImage, ImageFormat, Rgba, Pixel, GenericImageView, ImageBuffer};
use rfd::FileDialog;
use imageproc::drawing::{draw_line_segment_mut, draw_hollow_circle_mut, draw_hollow_rect_mut, draw_text_mut, draw_antialiased_line_segment_mut};
use rusttype::{Scale, Font};
use screenshots::Screen;

use crate::draws_functions::{Circle, Draws, Segment, Rectangle, Text, Single_Line, Crop};

pub fn copy_to_clipboard(image: &RgbaImage, draws: &Vec<Draws>, last_crop: Crop){
    let mut img_tmp = image.clone();
    draw_on_image(draws, &mut img_tmp);
    let mut img_to_copy = ImageBuffer::default();

    if last_crop.rectangle_logical.width().is_nan() || last_crop.rectangle_logical.height().is_nan(){
        img_to_copy = img_tmp;
    }
    else{
        println!("{}", last_crop.rectangle_logical.left_top().x);
        println!("{}", last_crop.rectangle_logical.left_top().y);
        println!("{}", last_crop.rectangle_logical.width());
        println!("{}", last_crop.rectangle_logical.height());

        let cropped_image = img_tmp.view(last_crop.rectangle_logical.left_top().x as u32,
                                                                        last_crop.rectangle_logical.left_top().y as u32,
                                                                        last_crop.rectangle_logical.width() as u32,
                                                                        last_crop.rectangle_logical.height() as u32)
                                                                    .to_image();
        img_to_copy = cropped_image;
    }

    let mut ctx_clip = Clipboard::new().unwrap();
    let clipboard_image = DynamicImage::ImageRgba8(img_to_copy.clone());
    let image_bytes = clipboard_image.into_bytes();
    #[rustfmt::skip]
    let img_data = OtherImageData { width: img_to_copy.width() as usize, height: img_to_copy.height() as usize, bytes: image_bytes.into() };
    ctx_clip.set_image(img_data).unwrap();
}

pub fn save_image(rgba_image: &mut RgbaImage, save_path: &PathBuf, name_convention: &String, file_format: &String, draws: &mut Vec<Draws>, last_crop: &mut Crop){
    let now = Utc::now();
    let ts = now.timestamp(); //add timestamp in the name convention, in order to have unique files

    //disegna sulla prima immagine della story_image con tutti i disegni
    let mut img_tmp = rgba_image.clone();
    draw_on_image(draws, &mut img_tmp);
    //dopodicchè croppa il tutto a mestiere...
    //println!("2");
    let mut dynamic_image = DynamicImage::default();
    
    if last_crop.rectangle_logical.width().is_nan() || last_crop.rectangle_logical.height().is_nan(){
        dynamic_image = DynamicImage::ImageRgba8(img_tmp);    
    }
    else{
        let text = format!("{} - {}", last_crop.rectangle_logical.width(), last_crop.rectangle_logical.height());
        println!("{}", text);
        
        let cropped_image = img_tmp.view(last_crop.rectangle_logical.left_top().x as u32,
                                                                        last_crop.rectangle_logical.left_top().y as u32,
                                                                        last_crop.rectangle_logical.width() as u32,
                                                                        last_crop.rectangle_logical.height() as u32)
                                                                    .to_image();
        dynamic_image = DynamicImage::ImageRgba8(cropped_image);    
    }

    //let format = retrieve_format(file_format.clone());
    let mut output_path = String::new();
    
    if *save_path != PathBuf::default() {
        output_path = format!("{}\\{}_{}{}", save_path.clone().into_os_string().into_string().unwrap(), name_convention, ts, file_format);
    }
    else {
        let p = FileDialog::new().set_directory("/").pick_folder();
            if p.is_none() { }
            else{
                let mut path_tmp = p.unwrap();
                output_path = format!("{}\\{}_{}{}", path_tmp.clone().into_os_string().into_string().unwrap(), name_convention, ts, file_format);
            }   
    }

    dynamic_image.save(output_path).expect("Failed to save image");
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

/*
fn retrieve_format(format: String) -> ImageFormat {
    match format.as_str() {
        ".png" => ImageFormat::Png,
        ".jpeg" => ImageFormat::Jpeg,
        ".gif" => ImageFormat::Gif,
        ".webp" => ImageFormat::WebP,
        ".pnm" => ImageFormat::Pnm,
        ".tiff" => ImageFormat::Tiff,
        ".tga" => ImageFormat::Tga,
        ".dds" => ImageFormat::Dds,
        ".bmp" => ImageFormat::Bmp,
        ".ico" => ImageFormat::Ico,
        ".hdr" => ImageFormat::Hdr,
        ".openexr" => ImageFormat::OpenExr,
        ".farbfeld" => ImageFormat::Farbfeld,
        ".avif" => ImageFormat::Avif,
        ".qoi" => ImageFormat::Qoi,
        _ => ImageFormat::Png,
    }
}
*/

fn draw_circle_on_image(circle: &Circle, image: &mut RgbaImage) {
    let center = circle.center.round();
    let radius = circle.radius as i32;
    let stroke_width = circle.stroke.width as f64;

    let color = image::Rgba([circle.stroke.color.r(), circle.stroke.color.g(), circle.stroke.color.b(), circle.stroke.color.a()]);

    for offset in -(stroke_width / 2.0).ceil() as i32..=(stroke_width / 2.0).floor() as i32{
        draw_hollow_circle_mut(image, (center.x as i32, center.y as i32), radius + offset, color);
    } 
}

fn draw_segment_on_image(segment: &Segment, image: &mut RgbaImage){
    let color = image::Rgba([segment.stroke.color.r(), segment.stroke.color.g(), segment.stroke.color.b(), segment.stroke.color.a()]);

    for i in 0..segment.stroke.width as i32 {
        let offset = i as i32 - segment.stroke.width as i32 / 2;
        draw_line_segment_mut(
            image,
            (segment.points[0].x + offset as f32, segment.points[0].y as f32),
            (segment.points[1].x + offset as f32, segment.points[1].y as f32),
            color,
        );

        draw_line_segment_mut(
            image,
            (segment.points[0].x as f32, segment.points[0].y + offset as f32),
            (segment.points[1].x as f32, segment.points[1].y + offset as f32),
            color,
        );
    }
}

fn draw_rect_on_image(rectangle: &Rectangle, image: &mut RgbaImage){
    let color = image::Rgba([rectangle.stroke.color.r(), rectangle.stroke.color.g(), rectangle.stroke.color.b(), rectangle.stroke.color.a()]);

    for new_offset in -(rectangle.stroke.width / 2.0).floor() as i32..=(rectangle.stroke.width / 2.0).floor() as i32{
        //let new_offset = offset - rectangle.stroke.width as i32 / 2;

        if rectangle.rect.width().abs()>0.0 && rectangle.rect.height().abs()>0.0{
            //let offset = offset as i32 - rectangle.stroke.width as i32 / 2;

            //println!("width rect: {}", rectangle.rect.width());
            //println!("width rect: {}", offset);
            let rect_to_draw_width = imageproc::rect::Rect::at(rectangle.rect.left() as i32 + new_offset, rectangle.rect.top() as i32)
                .of_size(rectangle.rect.width() as u32, rectangle.rect.height() as i32 as u32);

            let rect_to_draw_height = imageproc::rect::Rect::at(rectangle.rect.left() as i32, rectangle.rect.top() as i32 + new_offset)
                .of_size(rectangle.rect.width() as u32, rectangle.rect.height() as u32);

                
            let rect_to_draw_diag = imageproc::rect::Rect::at(rectangle.rect.left() as i32  + new_offset, rectangle.rect.top() as i32 + new_offset)
                .of_size(rectangle.rect.width() as u32, rectangle.rect.height() as u32);
            
    
            draw_hollow_rect_mut(image, rect_to_draw_width, color);
            draw_hollow_rect_mut(image, rect_to_draw_height, color);
            draw_hollow_rect_mut(image, rect_to_draw_diag, color);

        }
    }
}

fn draw_text_on_image(text: &Text, image: &mut RgbaImage, ){ 
    let color = image::Rgba([text.stroke.color.r(), text.stroke.color.g(), text.stroke.color.b(), text.stroke.color.a()]);
    let font_data: &[u8] = include_bytes!("SpaceMono-Regular.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
    println!("{}", text.letters);
    draw_text_mut(image, color, text.real_pos.x as i32, text.real_pos.y as i32, Scale::uniform(80.0), &font, text.letters.as_str())
}

fn blend_pixels<P>(start: P, end: P, weight: f32) -> P
where
    P: Pixel,
    P::Subpixel: std::ops::Add<Output = P::Subpixel> + std::ops::Mul<u8, Output = P::Subpixel> + Copy,
{
    let start_channels = start.channels();
    let end_channels = end.channels();

    let blended_channels: Vec<_> = start_channels
        .iter()
        .zip(end_channels.iter())
        .map(|(&a, &b)| a * (1.0 as u8 - weight as u8) + b * weight as u8)
        .collect();

    return *P::from_slice(&blended_channels);
}

fn draw_line_on_image(line: &Single_Line, image: &mut RgbaImage){
    let bound = line.points.len() - 1;
    let color = image::Rgba([line.stroke.color.r(), line.stroke.color.g(), line.stroke.color.b(), line.stroke.color.a()]);
    println!("{}", line.points.len());
    
    for (i, _) in line.points.iter().enumerate(){
        if i != (bound){
            for offset in -(line.stroke.width / 2.0).floor() as i32..=(line.stroke.width / 2.0).floor() as i32{
                draw_antialiased_line_segment_mut(image, ((line.points[i].x + offset as f32) as  i32, (line.points[i].y as f32) as i32),
                                                           ((line.points[i+1].x + offset as f32) as i32, (line.points[i+1].y as f32) as i32),
                                                                color,
                                                                &blend_pixels);

                draw_antialiased_line_segment_mut(image, ((line.points[i].x as f32) as  i32, (line.points[i].y + offset as f32) as i32),
                                                                ((line.points[i+1].x as f32) as i32, (line.points[i+1].y + offset as f32) as i32),
                                                                color,
                                                                &blend_pixels);
            }
        }
    }
}

pub fn draw_on_image(draws: &Vec<Draws>, image: &mut RgbaImage) -> () {
    for el in draws.iter(){
        match el{
            Draws::Circle(circle) => draw_circle_on_image(circle, image),
            Draws::Segment(segment) => draw_segment_on_image(segment, image),
            Draws::Rect(rectangle) => draw_rect_on_image(rectangle, image),
            Draws::Text(text) => draw_text_on_image(text, image),
            Draws::Line(line) => draw_line_on_image(line, image),
            _ => println!("miao"),
        }
        
    }
}
