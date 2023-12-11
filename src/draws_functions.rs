


use egui::*;





pub enum Draws {
    Line(Single_Line),
    Circle(Circle),
    Rect(Rectangle)
}


impl Draws{
    pub fn line(line: Single_Line) -> Self {
        Self::Line(line)
    }

    pub fn circle(circle: Circle) -> Self {
        Self::Circle(circle)
    }

    pub fn rect(rect: Rectangle) -> Self {
        Self::Rect(rect)
    }

    pub fn to_circle(&mut self) -> Option<&mut Circle> {
        match self {
            Draws::Circle(c) => Some(c),
            _ => None
        }
    }

    pub fn to_rect(&mut self) -> Option<&mut Rectangle> {
        match self {
            Draws::Rect(r) => Some(r),
            _ => None
        }
    }

    pub fn to_line(&mut self) -> Option<&mut Single_Line> {
        match self {
            Draws::Line(l) => Some(l),
            _ => None
        }
    }
}





#[derive(Default,Debug)]
pub struct Single_Line {
    pub points: Vec<Pos2>,
    pub stroke: Stroke,
}

impl Single_Line {
    pub fn new() -> Self {
        Self { points: Vec::new(),
             stroke: Stroke::new(0.0,Color32::default())}
    }
}

#[derive(Default,Debug)]
pub struct Circle{
    pub center: Pos2,
    pub radius: f32,
    pub stroke: Stroke
}

impl Circle {
    pub fn new() -> Self {
        Self { center: Pos2::default(),
                radius: f32::default(),
             stroke: Stroke::new(0.0,Color32::default())}
    }    
}

#[derive(Default,Debug)]
pub struct Rectangle{
    pub points: [Pos2;2],
    pub stroke: Stroke
}

impl Rectangle {
    pub fn new() -> Self {
        Self { points: [Pos2::default();2],
             stroke: Stroke::new(0.0,Color32::default())}
    }    
}