
use screenshots::{self, Screen, display_info};
use image::{RgbaImage, Rgb, ImageBuffer};
use std::error::Error;
use std::time::{Instant};
use eframe::egui::*;
#[derive(Debug)]
struct ErrorScreenshot(String);

pub fn screenshot () -> Result <RgbaImage, Box<dyn Error>> { 
    let start = Instant::now();
    let display_infos: Vec<display_info::DisplayInfo> = screenshots::display_info::DisplayInfo::all().unwrap();
    let screens = Screen::all().unwrap();
    if display_infos.len() == 1 {
        let screen = Screen::new(&display_infos[0]);
        println!("capturer {screen:?}");
        let mut raw_image = screen.capture().unwrap();
        return Ok( raw_image);
    }
    else{
        let screen = Screen::new(&display_infos[0]);
        for screen in screens {
            println!("capturer {screen:?}");
            let mut raw_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = screen.capture().unwrap();
            return Ok( raw_image);
        }
        return Err("Nessuno Schermo disponibile")?;
    }
}


    pub fn ui_content( ui: &mut Ui, lines: &mut Vec<Vec<Pos2>>, stroke: &mut Stroke, resp: Response, dim: Vec2) -> eframe::egui::Response {
        
        let (mut response, painter) =
            ui.allocate_painter(dim , Sense::drag());
            
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
                eframe::egui::Shape::line(points, *stroke)
            });

        painter.extend(shapes);
            
        response
    }
