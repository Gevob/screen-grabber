
use screenshots::{self, Screen, display_info};
use image::{RgbaImage, Rgb, ImageBuffer, imageops};
use std::error::Error;
use std::time::{Instant};
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
