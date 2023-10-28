
mod gui;
mod screen;

use eframe::egui;
use eframe::egui::TextureHandle;
use eframe::egui::Pos2;
use eframe::egui::Stroke;
use image::RgbaImage;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};

fn main() {
    
    //let mut ctx = egui::Context::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([640.0, 360.0].into()),
        min_window_size: Some([400.0, 320.0].into()),
        resizable: true,
        
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Some([640.0, 360.0].into()),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        ..Default::default()
    };
    eframe::run_native("Cattura", native_options, Box::new(|cc| Box::new(Windows::new(cc))));
    
}

pub struct MyGlobalHotKeyManager(GlobalHotKeyManager);

impl Default for MyGlobalHotKeyManager {
    fn default() -> Self {
        MyGlobalHotKeyManager(GlobalHotKeyManager::new().unwrap())
    }
}

#[derive(Default)]
struct Windows {
    schermata: Schermata,
    image : RgbaImage,
    texture : Option<TextureHandle>,

    //gestione delle hotkeys
    is_popup_open: bool,
    manager: MyGlobalHotKeyManager,
    modifier: Modifiers,
    key: Code,

    //gestione editing
    stroke: Stroke,
    points: Vec<Vec<Pos2>>,
    modifiche: EditType,
}

#[derive(Default,Debug)]
pub enum Schermata {
    #[default]
    Home,
    Edit,
}
//indica il tipo di editing
#[derive(Default,Debug)]
pub enum EditType {
    #[default]
    Raw,
    Pennarello,
    Evidenziatore,
    Cerchi,
    Rettangoli,
    Gomma
}


impl Windows {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
         //cc.egui_ctx.set_pixels_per_point(1.0);
        //println!("{:?}",cc.egui_ctx.pixels_per_point());
        
        Self {
            stroke: Stroke::new(1.0, eframe::egui::Color32::from_rgba_premultiplied(200, 195, 25, 255)),
            ..Default::default()
        }
    }
}

impl eframe::App for Windows {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    eframe::egui::Context::set_pixels_per_point(ctx, 1.0);
    match self.schermata {
        Schermata::Home => gui::home(ctx,&mut self.schermata, &mut self.image, &mut self.texture, &mut self.is_popup_open, &mut self.manager, &mut self.modifier,  &mut self.key, frame, &mut self.stroke, &mut self.points),
        Schermata::Edit => gui::edit(ctx),
    }

    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        self.image = screen::screenshot().unwrap();
        let flat_image = self.image.as_flat_samples();
        let color_image2 = egui::ColorImage::from_rgba_unmultiplied([self.image.width() as usize, self.image.height() as usize],flat_image.samples);
        let image_data = egui::ImageData::from(color_image2);
        self.texture = Some(ctx.load_texture("screen", image_data, Default::default()));
    }
        //println!("{:?}",frame.info().window_info.size);
        println!("proporzione: {:?}",egui::Context::pixels_per_point(ctx));
   }
}






    

