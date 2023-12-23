


use egui::{*, emath::RectTransform};

#[derive(Debug,Clone)]
pub enum Draws {
    Line(Single_Line),
    Circle(Circle),
    Rect(Rectangle),
    Segment(Segment),
    Text(Text)
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

    pub fn segment(rect: Rectangle) -> Self {
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

    pub fn to_segment(&mut self) -> Option<&mut Segment> {
        match self {
            Draws::Segment(l) => Some(l),
            _ => None
        }
    }

    pub fn to_text(&mut self) -> Option<&mut Text> {
        match self {
            Draws::Text(t) => Some(t),
            _ => None
        }
    }
}





#[derive(Default,Debug,Clone)]
pub struct Single_Line {
    pub points: Vec<Pos2>,
    pub stroke: Stroke,
}

impl Single_Line {
    pub fn new(stroke: &Stroke) -> Self {
        Self { points: Vec::new(),
             stroke: *stroke}
    }
}

#[derive(Default,Debug,Clone)]
pub struct Circle{
    pub center: Pos2,
    pub radius: f32,
    pub stroke: Stroke
}

impl Circle {
    pub fn new(stroke: &Stroke) -> Self {
        Self { center: Pos2::default(),
                radius: f32::default(),
             stroke: *stroke}
    }    
}

#[derive(Debug,Clone)]
pub struct Rectangle{
    pub rect: Rect,
    pub first_point: Pos2,
    pub stroke: Stroke
}

impl Rectangle {
    pub fn new(point: Pos2, stroke: &Stroke) -> Self {
        Self { //points: [Pos2::default();2]
             rect: Rect::NOTHING,
             first_point: point,
             stroke: *stroke}
    }  

    pub fn from_two_point(&self,point: Pos2)   -> Rect {
            Rect::from_two_pos(self.first_point,point)
    }
}


#[derive(Default,Debug,Clone)]
pub struct Segment{
    pub points: [Pos2;2],
    pub stroke: Stroke
}

impl Segment {
    pub fn new(stroke: &Stroke) -> Self {
        Self { points: [Pos2::default();2],
             stroke: *stroke}
    }    
}

#[derive(Default,Debug,Clone)]
pub struct Text{
    pub point: Pos2,
    pub letters: String,
    pub stroke: Stroke,
    pub focus: bool
}





impl Text {
    pub fn new(stroke: &Stroke) -> Self {
        Self { point: Pos2::default(),
             letters: String::from(""),
             focus: true,
             stroke: *stroke}
    } 

    pub fn add_input(&mut self, input: &String)  {
        self.letters.push_str(input);
    }

    pub fn remove_input(&mut self)  {
        if self.letters.len() > 0 {
            self.letters.pop();
        }
        
    }
}

#[derive(Debug,Clone)]
pub struct Crop {
    pub left_top: Pos2,
    pub first_point: Pos2,
    pub rectangle: Rect,
    pub rectangle_logical: Rect,
    pub first_point_logical: Pos2,
}

impl Default for Crop {
    fn default() -> Self {
        Crop {
            left_top: Pos2::ZERO,
            first_point: Pos2::ZERO,
            rectangle: Rect::NAN, // Assuming Rect has a NAN associated constant or constructor
            rectangle_logical: Rect::NAN,
            first_point_logical: Pos2::ZERO,
        }
    }
}

impl Crop {
    // pub fn new(point: Pos2) -> Self {
    //     Crop {
    //         left_top
    //         first_point: point,
    //         rectangle: Rect::NAN, // Assuming Rect has a NAN associated constant or constructor 
    //     }
    // }

    pub fn from_two_point(&mut self,point: Pos2){
        self.rectangle = Rect::from_two_pos(self.first_point,point);
}

    pub fn from_two_point_logical(&mut self,point: Pos2){
    self.rectangle_logical = Rect::from_two_pos(self.first_point_logical,point);
}
}



#[derive(Debug,Clone)]
pub enum Last_Action {
    Crop(Pos2),
    Annotation,
    Erase
}
