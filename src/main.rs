
mod gui;
mod screen;
mod edit;
mod draws_functions;
mod icons;
mod wrapper_functions;


use draws_functions::{Draws, Crop, Last_Action};
use gui::{String_to_hotkey, hotkey_to_String};
use screenshots::display_info;
use std::{path::PathBuf, io::Write};
use std::sync::Arc;
use egui::*;
use image::RgbaImage;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};
use std::ptr;
use std::thread;
use std::time::{Duration, Instant};
use arboard::{Clipboard, ImageData as OtherImageData};
use image::DynamicImage;
use std::fs::File;
use std::io::{self, BufRead};
use std::fs::OpenOptions;
use std::io::LineWriter;

fn main() {
    
    //let mut ctx = egui::Context::default();
    let native_options = eframe::NativeOptions {
        //initial_window_size: Some([500.0, 400.0].into()),
        //min_window_size: Some([500.0, 400.0].into()),
        //resizable: true,
        viewport: egui::ViewportBuilder::default(),
         //.with_inner_size([400.0, 400.0])
         //.with_resizable(false)
         //.with_maximize_button(false),
         
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
    eframe::run_native("Cattura", native_options, Box::new(|cc| Box::new(Windows::new(cc)))).unwrap();
    
}

pub struct MyGlobalHotKeyManager(GlobalHotKeyManager);

impl Default for MyGlobalHotKeyManager {
    fn default() -> Self {
        MyGlobalHotKeyManager(GlobalHotKeyManager::new().unwrap())
    }
}

pub struct MyInstant(Instant);

impl Default for MyInstant {
    fn default() -> Self {
        MyInstant(Instant::now())
    }
}

#[derive(Default)]
struct Windows {
    schermata: Schermata,
    image : RgbaImage,
    texture : Option<TextureHandle>,
    change_size : bool, //usato per gestire il cambio di dimensione della finestra (quando è piccola non si deve poter ridimensionare !)

    //gestione delle hotkeys
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
    hotkeys_list: Vec<(Modifiers, Code, String, u32)>,

    //gestione editing
    draws: Vec<Draws>,
    mode: EditType,
    stroke: Stroke,
    last_index: Option<usize>,
    crop: Crop,
    last_actions: Vec<Last_Action>,
    story_image : Vec<RgbaImage>, 
    story_texture : Vec<Option<TextureHandle>>,
    garbage: Vec<Draws>,

    //gestione del salvataggio
    file_format_tmp: String,
    save_path_tmp: PathBuf,
    name_convention_tmp: String,
    file_format: String,
    save_path: PathBuf,
    name_convention: String,
    update_file: bool,
    monitor_used: usize,
    monitor_used_tmp: usize,
    num_monitors: usize,
    last_crop: Crop,

    //screenshot
    free_to_screenshots: bool,

    //gestione delay timer
    start_time: MyInstant,
    delay_duration: Duration,
    timer_expired: bool,
    start_timer: bool,
    delay_tmp: u64,
}

#[derive(Default,Debug,PartialEq)]
pub enum Schermata {
    #[default]
    Home,
    Edit,
    Setting_Hotkey,
    Setting_Saving,
    Setting_Timer,
}

//indica il tipo di editing
#[derive(Default,Debug,PartialEq)]
pub enum EditType {
    #[default]
    Cursor,
    Segment,
    Free,
    Circle,
    Rectangle,
    Crop,
    Eraser,
    Back,
    Text
}


impl Windows {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
         cc.egui_ctx.set_pixels_per_point(1.0);
         egui_extras::install_image_loaders(&cc.egui_ctx);
         
        //println!("{:?}",cc.egui_ctx.pixels_per_point());
        
        let file = File::open("src/default.txt").unwrap();
        let reader = io::BufReader::new(file);
        let manager = MyGlobalHotKeyManager::default();
        let mut hotkeys_list = Vec::<(Modifiers, Code, String, u32)>::new();
        let mut format = String::new();
        let mut path = PathBuf::new();
        let mut name_convention = String::new();
        let mut modifier_copy = Modifiers::default();
        let mut modifier_screen = Modifiers::default();
        let mut modifier_save = Modifiers::default();
        let mut key_copy = Code::default();
        let mut key_screen = Code::default();
        let mut key_save = Code::default();
        let mut start_time = MyInstant::default();
        let display_infos: Vec<display_info::DisplayInfo> = screenshots::display_info::DisplayInfo::all().unwrap();

        //set_font_style(&cc.egui_ctx);
        
        for (index, line) in reader.lines().enumerate() {

            match index {
                0 => {
                    let tmp = String_to_hotkey(line.unwrap());
                    let hotkey_copy = HotKey::new(Some(tmp.0), tmp.1);
                    manager.0.register(hotkey_copy).unwrap();
                    hotkeys_list.push((tmp.0, tmp.1, "Copy".to_string(), hotkey_copy.id()));
                    modifier_copy = tmp.0;
                    key_copy = tmp.1;
                },
                1 => {
                    let tmp = String_to_hotkey(line.unwrap());
                    let hotkey_screen = HotKey::new(Some(tmp.0), tmp.1);
                    manager.0.register(hotkey_screen).unwrap();
                    hotkeys_list.push((tmp.0, tmp.1, "Screen".to_string(), hotkey_screen.id()));
                    modifier_screen = tmp.0;
                    key_screen = tmp.1;
                },
                2 => {
                    let tmp = String_to_hotkey(line.unwrap());
                    let hotkey_save = HotKey::new(Some(tmp.0), tmp.1);
                    manager.0.register(hotkey_save).unwrap();
                    hotkeys_list.push((tmp.0, tmp.1, "Save".to_string(), hotkey_save.id()));
                    modifier_save = tmp.0;
                    key_save = tmp.1;
                }
                3 => {
                    format = line.unwrap();
                }
                4 => {
                    path.push(line.unwrap());
                }
                5 => {
                    name_convention = line.unwrap();
                }
                _ => break, // Break out of the loop if all variables are assigned
            }
        }

        let mut style = (*cc.egui_ctx.style()).clone();

        style.visuals.panel_fill = eframe::egui::Color32::from_rgb(45, 70, 110); 
        cc.egui_ctx.set_style(style);


        Self {
            //text_focused: Box::new(Text::new()),
            //draws: vec![Draws::Text(Text::new())],
            free_to_screenshots: false,
            stroke: Stroke::new(3.0, egui::Color32::from_rgba_premultiplied(200, 195, 25, 255)),
            manager: manager,
            change_size: false, 
            hotkeys_list: hotkeys_list,
            file_format: format.clone(),
            file_format_tmp: format.clone(),
            save_path: path.clone(),
            save_path_tmp: path.clone(),
            name_convention: name_convention.clone(),
            name_convention_tmp: name_convention.clone(),
            modifier_copy: modifier_copy,
            modifier_copy_tmp: modifier_copy,
            modifier_screen: modifier_screen,
            modifier_screen_tmp: modifier_screen,
            modifier_save: modifier_save,
            modifier_save_tmp: modifier_save,
            key_copy: key_copy,
            key_copy_tmp: key_copy,
            key_screen: key_screen,
            key_screen_tmp: key_screen,
            key_save: key_save,
            key_save_tmp: key_save,
            start_time: start_time,
            delay_duration: Duration::from_secs(0), // 0 seconds delay
            timer_expired: true,
            start_timer: false,
            delay_tmp: 0,
            monitor_used: 0, //uso solo il primo monitor di default
            monitor_used_tmp: 0, //uso solo il primo monitor di default
            num_monitors: display_infos.len(),

            ..Default::default()
        }
    }

    fn update_file_default_setting(&self){
        let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("src/default.txt"); // Open the file with write and truncate mode to erase its contents
        
        //file.unwrap().try_clone().unwrap().seek(SeekFrom::Start(0)).unwrap();
        let mut new_line = LineWriter::new(file.unwrap());
        let mut lines = Vec::<String>::new();

        lines.push(hotkey_to_String(self.modifier_copy, self.key_copy));
        lines.push(hotkey_to_String(self.modifier_screen, self.key_screen));
        lines.push(hotkey_to_String(self.modifier_save, self.key_save));
        lines.push(self.file_format.clone());
        lines.push(self.save_path.clone().into_os_string().to_str().unwrap().to_string());
        lines.push(self.name_convention.clone());

        for line in lines.iter_mut(){
            line.push_str("\n");
            new_line.write_all(line.as_bytes());
        }
    }
}

impl eframe::App for Windows {
   fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
    //eframe::egui::Context::set_pixels_per_point(ctx, 1.0);

    if self.start_timer {
        std::thread::sleep(Duration::from_millis(200));
        let handle = timing(self.start_time.0, self.delay_duration);
        self.timer_expired = handle.join().unwrap();
        self.start_timer = false;
    }

    if !{self.timer_expired} {
        self.timer_expired=true;
        self.free_to_screenshots = true;
    }
    
    if self.free_to_screenshots {
        if self.delay_duration.is_zero() {std::thread::sleep(Duration::from_millis(400));}

        self.free_to_screenshots = false;
        screen::make_screenshot(ctx, &mut self.image, &mut self.texture, &mut self.schermata, self.monitor_used,&mut self.story_image, &mut self.story_texture, &mut self.draws, &mut self.last_crop, &mut self.last_actions, &mut self.garbage);

        ctx.send_viewport_cmd(viewport::ViewportCommand::Minimized(false.into()));
        ctx.send_viewport_cmd(viewport::ViewportCommand::InnerSize([900.0, 500.0].into())); //set_window_size substituted by ctx.send....
        ctx.send_viewport_cmd(viewport::ViewportCommand::Resizable(true.into())); //set_window_size substituted by ctx.send....
        ctx.send_viewport_cmd(viewport::ViewportCommand::EnableButtons { close: true.into(), minimized: true.into(), maximize: true.into() });
        self.change_size = false;
        //ctx.send_viewport_cmd(viewport::ViewportCommand::WindowLevel(crate::WindowLevel::Normal));
    }

    if self.schermata == Schermata::Home || self.schermata == Schermata::Setting_Hotkey || self.schermata == Schermata::Setting_Saving || self.schermata == Schermata::Setting_Timer && !self.change_size{
        //ctx.send_viewport_cmd(viewport::ViewportCommand::InnerSize([400.0, 400.0].into())); //set_window_size substituted by ctx.send....
        ctx.send_viewport_cmd(viewport::ViewportCommand::Resizable(false.into())); //set_window_size substituted by ctx.send....
        ctx.send_viewport_cmd(viewport::ViewportCommand::EnableButtons { close: true.into(), minimized: true.into(), maximize: false.into() });
        self.change_size = true;
    }

    if self.schermata == Schermata::Edit && self.change_size{
        ctx.send_viewport_cmd(viewport::ViewportCommand::InnerSize([900.0, 500.0].into())); //set_window_size substituted by ctx.send....
        ctx.send_viewport_cmd(viewport::ViewportCommand::Resizable(true.into())); //set_window_size substituted by ctx.send....
        ctx.send_viewport_cmd(viewport::ViewportCommand::EnableButtons { close: true.into(), minimized: true.into(), maximize: true.into() });
        self.change_size = false;
    }

    match self.schermata {
        Schermata::Home => { // frame.info.window_size substituted by ctx.screen_rect().size()
            gui::home(ctx, &mut self.schermata, &mut self.image, &mut self.texture, &mut self.hotkeys_list, &mut self.file_format, &mut self.save_path, &mut self.name_convention, &mut self.monitor_used,&mut self.story_image,&mut self.story_texture,&mut self.free_to_screenshots, &mut self.start_time.0, &mut self.delay_duration, &mut self.start_timer);
        },
        Schermata::Edit => {
            gui::edit(ctx, &mut self.draws, &mut self.texture, frame, &mut self.stroke, &mut self.schermata, &mut self.image, &mut self.file_format, &mut self.save_path, &mut self.name_convention, &mut self.last_index, &mut self.mode,&mut self.crop, &mut self.last_actions, &mut self.story_image,&mut self.story_texture,&mut self.garbage, &mut self.last_crop);
        },
        Schermata::Setting_Hotkey => {
            gui::setting_hotkey(ctx, &mut self.schermata, &mut self.manager, &mut self.modifier_copy, &mut self.key_copy, &mut self.modifier_screen, &mut self.key_screen, &mut self.modifier_save, &mut self.key_save, &mut self.hotkeys_list, &mut self.modifier_copy_tmp, &mut self.key_copy_tmp, &mut self.modifier_screen_tmp, &mut self.key_screen_tmp, &mut self.modifier_save_tmp, &mut self.key_save_tmp, &mut self.update_file, &self.texture);
        },
        Schermata::Setting_Saving => {
            gui::setting_saving(ctx, &mut self.schermata, &mut self.file_format, &mut self.save_path, &mut self.file_format_tmp, &mut self.save_path_tmp, &mut self.name_convention, &mut self.name_convention_tmp, &mut self.update_file, &mut self.monitor_used, &mut self.monitor_used_tmp, &self.texture);
        },
        Schermata::Setting_Timer=> {
            gui::setting_timer(ctx, &mut self.schermata, &mut self.delay_duration, &mut self.delay_tmp , &self.texture);

        },
    }

    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        println!("{:?}",event.id());

        for el in self.hotkeys_list.iter(){
            if event.id() == el.3 && el.2 == "Screen".to_string() {
                //screen::make_screenshot(ctx, &mut self.image, &mut self.texture, &mut self.schermata, self.monitor_used,&mut self.story_image, &mut self.story_texture)
                ctx.send_viewport_cmd(viewport::ViewportCommand::Minimized(true.into()));
                self.start_time = MyInstant(Instant::now());
                self.start_timer = true;
                self.free_to_screenshots = true;
            }
            else if event.id() == el.3 && el.2 == "Copy".to_string() && !(self.texture.is_none()) {
                // Copy the image to the clipboard
                wrapper_functions::copy_to_clipboard(&self.story_image[0], &self.draws, self.last_crop.clone())
            }
            else if event.id() == el.3 && el.2 == "Save".to_string() && !(self.texture.is_none()) {
                wrapper_functions::save_image(&mut self.story_image[0], &self.save_path, &self.name_convention, &self.file_format, &mut self.draws, &mut self.last_crop);
            }
        }
    }
    if self.update_file{
        self.update_file_default_setting();
        self.update_file = false;
    }

    let display_infos: Vec<display_info::DisplayInfo> = screenshots::display_info::DisplayInfo::all().unwrap();

    if self.num_monitors != display_infos.len(){
        self.monitor_used = 0; //nel caso si "stacchi" il cavo all'imporvviso del secondo schermo, si riporta lo schermo principale al primo
        self.monitor_used_tmp = 0;
        self.num_monitors = display_infos.len();
    }
   }
}

fn timing(start_time : Instant, delay_duration : Duration) -> std::thread::JoinHandle<bool> {
    let handle = std::thread::spawn(move || {
        let mut stop : bool = true;
        loop {
            // Your timer logic goes here
            // Check if the timer has expired and perform the necessary actions
            if (Instant::now() - start_time) > delay_duration {
                stop = false;
                break;
            }
            // Sleep for a short duration to avoid unnecessary CPU usage
            std::thread::sleep(Duration::from_millis(100)); // Adjust as needed
        }
        return stop;
    });
    return handle;
}

