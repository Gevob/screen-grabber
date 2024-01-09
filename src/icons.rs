use egui::{Image};


lazy_static::lazy_static! {
    pub static ref BACK :Image<'static>  = Image::new(egui::include_image!("./icons/back.png"));
    pub static ref CIRCLE :Image<'static> = Image::new(egui::include_image!("./icons/circle.png"));
    pub static ref SCISSOR :Image<'static> = Image::new(egui::include_image!("./icons/scissor.png"));
    pub static ref CURSOR :Image<'static> = Image::new(egui::include_image!("./icons/cursor.png"));
    pub static ref ERASER :Image<'static> = Image::new(egui::include_image!("./icons/eraser.png"));
    //pub static ref HIGHLIGHT :Image<'static> = Image::new(egui::include_image!(".../icons/circle.png"));
    pub static ref SEGMENT :Image<'static> = Image::new(egui::include_image!("./icons/segment.png"));
    pub static ref FREE :Image<'static> = Image::new(egui::include_image!("./icons/free.png"));
    pub static ref RECTANGLE :Image<'static> = Image::new(egui::include_image!("./icons/rectangle.png"));
    //pub static ref REDO :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/redo.png"));
    pub static ref TEXT :Image<'static> = Image::new(egui::include_image!("./icons/text.png"));
    pub static ref SCREEN :Image<'static> = Image::new(egui::include_image!("./icons/screen.png"));
    pub static ref SETTING :Image<'static> = Image::new(egui::include_image!("./icons/setting.png"));
    pub static ref TRASH :Image<'static> = Image::new(egui::include_image!("./icons/bin.png"));
    pub static ref COPY :Image<'static> = Image::new(egui::include_image!("./icons/copy.png"));
    pub static ref SAVE :Image<'static> = Image::new(egui::include_image!("./icons/save.png"));
}
