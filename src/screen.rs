
use screenshots::{self, Screen, display_info};
use image::{RgbaImage, Rgb, ImageBuffer, imageops};
use std::error::Error;
use std::ptr;
use std::thread::sleep;
use std::time::{Instant, Duration};
use egui::*;


use crate::Schermata;
#[derive(Debug)]
struct ErrorScreenshot(String);

pub fn screenshot (used_monitor: usize) -> Result <RgbaImage, Box<dyn Error>> { 
    let start = Instant::now();
    let display_infos: Vec<display_info::DisplayInfo> = screenshots::display_info::DisplayInfo::all().unwrap();
    let screen = Screen::new(&display_infos[used_monitor]);
    println!("capturer {screen:?}");
    let mut raw_image = screen.capture().unwrap();
    return Ok( raw_image);
}


pub fn ui_content( ui: &mut Ui, lines: &mut Vec<Vec<Pos2>>, stroke: &mut Stroke, resp: Response, dim: Vec2) -> egui::Response {

    let (mut response, painter) =
        //ui.allocate_painter(dim , Sense::drag());
        ui.allocate_painter(ui.available_size_before_wrap() , Sense::click_and_drag());
        
    let to_screen = emath::RectTransform::from_to(
        Rect::from_min_size(response.rect.min, response.rect.square_proportions()),
        response.rect,
    );
    let from_screen = to_screen.inverse();

    if lines.is_empty() {
        lines.push(vec![]);
    }

    let current_line = lines.last_mut().unwrap();

    if let Some(pointer_pos) = response.interact_pointer_pos() {
        let canvas_pos = from_screen * pointer_pos;
        if current_line.last() != Some(&canvas_pos) {
            current_line.push(canvas_pos);
            response.mark_changed();
        }
    } else if !current_line.is_empty() {
        lines.push(vec![]);
        response.mark_changed();
    }

    let shapes = 
        lines
        .iter()
        .filter(|line| line.len() >= 2)
        .map(|line| {
            let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
            egui::Shape::line(points, *stroke)
        });

    painter.extend(shapes);
        
    response
    }

    pub fn make_screenshot(ctx: &Context, image: &mut RgbaImage, texture : &mut Option<TextureHandle>, schermata: &mut Schermata, used_monitor: usize){
        if used_monitor == 9999{
            *image = screenshot_all_monitors().unwrap();
        }
        else{
            *image = screenshot(used_monitor).unwrap();
        }

        let flat_image = image.as_flat_samples();
        let color_image2 = egui::ColorImage::from_rgba_unmultiplied([image.width() as usize, image.height() as usize],flat_image.samples);
        let image_data = egui::ImageData::from(color_image2);
        *texture = Some(ctx.load_texture("screen", image_data, Default::default()));
        *schermata = Schermata::Edit;
    }

    pub fn screenshot_all_monitors() -> Result<RgbaImage, Box<dyn Error>> {
        let start = Instant::now();
        let display_infos = screenshots::display_info::DisplayInfo::all().unwrap();
        let mut combined_width = 0;
        let mut max_height = 0;
    
        // Capture the screens and calculate the combined width and maximum height
        let mut screen_images = vec![];
        for display_info in &display_infos {
            let screen = Screen::new(display_info);
            let raw_image = screen.capture().unwrap();
            let raw_image_clone = raw_image.clone();
            let image = image::ImageBuffer::from_raw(raw_image.width(), raw_image.height(), raw_image.into_raw()).unwrap();
            screen_images.push(image);
            combined_width += raw_image_clone.width();
            if raw_image_clone.height() > max_height {
                max_height = raw_image_clone.height();
            }
        }
    
    // Create a new image with the combined width and maximum height
    let mut combined_image = RgbaImage::new(combined_width, max_height);

    // Combine the captured images into a single image
    let mut current_x = 0;
    for screen_image in screen_images {
        imageops::overlay(&mut combined_image, &screen_image, current_x, 0);
        current_x += (screen_image.width() as i64);
    }

    Ok(combined_image)
}
