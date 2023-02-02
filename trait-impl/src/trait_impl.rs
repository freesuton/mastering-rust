
pub trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
}

pub struct Circle {
    pub radius: f32,
}

impl Geometry for Circle{
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        3.14 * 2.0 * self.radius
    }
}

