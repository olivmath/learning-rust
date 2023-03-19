use crate::{Circle, Polygon};

/// An enum representing different shapes, such as `Polygon` and `Circle`.
pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    /// Calculate the perimeter of the shape.
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(p) => p.perimeter(),
            Shape::Circle(c) => c.perimeter(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}
