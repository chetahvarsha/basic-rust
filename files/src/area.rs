// area.rs

pub enum Shape {
    Circle { radius: f64 },
    Triangle { width: f64, height: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Triangle { width, height } => 0.5 * (width * height),
        }
    }
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}



