
mod gui;
mod screen;

use std::path::PathBuf;
use std::sync::Arc;
use ::egui::Color32;
use egui::style::Style;
use eframe::egui;
use eframe::egui::TextureHandle;
use eframe::egui::Pos2;
use eframe::egui::Stroke;
use ::egui::{Image, ImageData};
use image::RgbaImage;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};
use egui::epaint::image::ColorImage;
use std::ptr;
use winapi::um::winuser::{GetForegroundWindow, ShowWindow, SW_HIDE, SW_SHOW};
use std::thread::sleep;
use std::time::Duration;
use arboard::{Clipboard, ImageData as OtherImageData};
use image::DynamicImage;

fn main() {
    
    //let mut ctx = egui::Context::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([500.0, 400.0].into()),
        min_window_size: Some([500.0, 400.0].into()),
        resizable: true,
        
        ..Default::default()
    };
    /*
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
    */
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
    change_size : bool, //usato per gestire il cambio di dimensione della finestra (quando è piccola non si deve poter ridimensionare !)

    //gestione delle hotkeys
    is_popup_open: bool,
    manager: MyGlobalHotKeyManager,
    modifier_copy: Modifiers,
    key_copy: Code,
    modifier_screen: Modifiers,
    key_screen: Code,
    modifier_save: Modifiers,
    key_save: Code,
    modifier_copy_tmp: Modifiers,
    key_copy_tmp: Code,
    modifier_screen_tmp: Modifiers,
    key_screen_tmp: Code,
    modifier_save_tmp: Modifiers,
    key_save_tmp: Code,
    hotkeys_list: Vec<(Modifiers, Code, String)>,

    //gestione editing
    stroke: Stroke,
    points: Vec<Vec<Pos2>>,
    modifiche: EditType,

    //gestione del salvataggio
    is_popup_open2: bool,
    file_format_tmp: String,
    save_path_tmp: PathBuf,
    name_convention_tmp: String,
    file_format: String,
    save_path: PathBuf,
    name_convention: String,

}

#[derive(Default,Debug)]
pub enum Schermata {
    #[default]
    Home,
    Edit,
    Setting_Hotkey,
    Setting_Saving,
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
        let manager = MyGlobalHotKeyManager::default();
        let mut hotkey_copy = HotKey::new(Some(Modifiers::CONTROL), Code::KeyA);
        let mut hotkey_screen = HotKey::new(Some(Modifiers::CONTROL), Code::KeyB);
        let mut hotkey_save = HotKey::new(Some(Modifiers::CONTROL), Code::KeyD);

        manager.0.register(hotkey_copy);
        manager.0.register(hotkey_screen);
        manager.0.register(hotkey_save);

        let mut hotkeys_list = Vec::<(Modifiers, Code, String)>::new();
        hotkeys_list.push((Modifiers::CONTROL, Code::KeyA, "Copy".to_string()));
        hotkeys_list.push((Modifiers::CONTROL, Code::KeyB, "Screen".to_string()));
        hotkeys_list.push((Modifiers::CONTROL, Code::KeyD, "Save".to_string()));

        let mut style = (*cc.egui_ctx.style()).clone();

        style.visuals.panel_fill = eframe::egui::Color32::from_rgb(0, 0, 139); // Dodger Blue color
        cc.egui_ctx.set_style(style);


        Self {
            stroke: Stroke::new(1.0, eframe::egui::Color32::from_rgba_premultiplied(200, 195, 25, 255)),
            manager: manager,
            change_size: false, 
            hotkeys_list: hotkeys_list,
            file_format: ".jpeg".to_string(),
            file_format_tmp: ".jpeg".to_string(),
            modifier_copy: Modifiers::CONTROL,
            modifier_copy_tmp: Modifiers::CONTROL,
            modifier_screen: Modifiers::CONTROL,
            modifier_screen_tmp: Modifiers::CONTROL,
            modifier_save: Modifiers::CONTROL,
            modifier_save_tmp: Modifiers::CONTROL,
            key_copy: Code::KeyA,
            key_copy_tmp: Code::KeyA,
            key_screen: Code::KeyB,
            key_screen_tmp: Code::KeyB,
            key_save: Code::KeyD,
            key_save_tmp: Code::KeyD,
            name_convention: "Screen".to_string(),
            name_convention_tmp: "Screen".to_string(),
            ..Default::default()
        }
    }
}

impl eframe::App for Windows {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    eframe::egui::Context::set_pixels_per_point(ctx, 1.0);
    

    match self.schermata {
        Schermata::Home => {
            if frame.info().window_info.size != [400.0, 300.0].into(){
                frame.set_window_size([400.0, 300.0].into());
                self.change_size = true;
            }
            gui::home(ctx, &mut self.schermata, &mut self.image, &mut self.texture, &mut self.hotkeys_list, &mut self.file_format, &mut self.save_path, &mut self.name_convention);
        },
        Schermata::Edit => {
            if frame.info().window_info.size != [800.0, 620.0].into() && self.change_size{
                frame.set_window_size([800.0, 620.0].into());
                self.change_size = false;
            }
            gui::edit(ctx, &mut self.stroke, &mut self.texture, frame, &mut self.points, &mut self.schermata, &mut self.image, &mut self.file_format, &mut self.save_path, &mut self.name_convention);
        },
        Schermata::Setting_Hotkey => {
            if frame.info().window_info.size != [400.0, 300.0].into(){
                frame.set_window_size([400.0, 300.0].into());
                self.change_size = true;
            }
            gui::setting_hotkey(ctx, &mut self.schermata, &mut self.manager, &mut self.modifier_copy, &mut self.key_copy, &mut self.modifier_screen, &mut self.key_screen, &mut self.modifier_save, &mut self.key_save, &mut self.hotkeys_list, &mut self.modifier_copy_tmp, &mut self.key_copy_tmp, &mut self.modifier_screen_tmp, &mut self.key_screen_tmp, &mut self.modifier_save_tmp, &mut self.key_save_tmp);
        },
        Schermata::Setting_Saving => {
            if frame.info().window_info.size != [400.0, 300.0].into(){
                frame.set_window_size([400.0, 300.0].into());
                self.change_size = true;
            }
            gui::setting_saving(ctx, &mut self.schermata, &mut self.file_format, &mut self.save_path, &mut self.file_format_tmp, &mut self.save_path_tmp, &mut self.name_convention, &mut self.name_convention_tmp)

        },
    }

    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        println!("{:?}",event.id());
        //println!("{:?}",self.image.width());
        //println!("{:?}",self.image.height());
            if(event.id() == 869406661) {
                unsafe {
                    // Find the window by class name
                    let hwnd = GetForegroundWindow();
                    // Hide the window if it is found
                    if hwnd != ptr::null_mut() {
                        ShowWindow(hwnd, SW_HIDE);
                        sleep(Duration::from_millis(500));
                        self.image = screen::screenshot().unwrap();
                        ShowWindow(hwnd, SW_SHOW);
                    }
                }
                let flat_image = self.image.as_flat_samples();
                let color_image2 = egui::ColorImage::from_rgba_unmultiplied([self.image.width() as usize, self.image.height() as usize],flat_image.samples);
                let image_data: egui::ImageData = egui::ImageData::from(color_image2);
                self.texture = Some(ctx.load_texture("screen", image_data, Default::default()));
            }
            else if(event.id() == 538883802 && !(self.texture.is_none())) {
                // Copy the image to the clipboard
                let mut ctx_clip = Clipboard::new().unwrap();
                let clipboard_image = DynamicImage::ImageRgba8(self.image.clone());
                let image_bytes = clipboard_image.into_bytes();
                #[rustfmt::skip]
                let img_data = OtherImageData { width: self.image.width() as usize, height: self.image.height() as usize, bytes: image_bytes.into() };
                ctx_clip.set_image(img_data).unwrap();
            }

        if (!self.is_popup_open && !self.is_popup_open2){
            self.schermata = Schermata::Edit;
        }
    }
        //println!("{:?}",frame.info().window_info.size);
        //println!("proporzione: {:?}",egui::Context::pixels_per_point(ctx));
   }
}
