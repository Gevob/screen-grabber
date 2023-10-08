
use screenshots::{self, Screen, display_info};
use image::{RgbaImage, Rgb, ImageBuffer};
use std::error::Error;
use std::time::{Instant};

#[derive(Debug)]
struct ErrorScreenshot(String);

pub fn screenshot () -> Result < RgbaImage, Box<dyn Error>> { 
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
            //image
              //  .save(format!("C:/Users/peppi/Downloads/{}.png", screen.display_info.id))
                //.unwrap();
    
            //image = screen.capture_area(300, 300, 300, 300).unwrap();
            //image
             //   .save(format!("C:/Users/peppi/Downloads/{}-2.png", screen.display_info.id))
              //  .unwrap();
        }
        return Err("Nessuno Schermo disponibile")?;
    }
}