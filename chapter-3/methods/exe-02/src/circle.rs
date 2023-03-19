use crate::Point;
use std::f64::consts::PI;

/// A struct representing a circle.
#[derive(Clone, PartialEq, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    /// Create a new circle with the given center point and radius.
    ///
    /// # Arguments
    ///
    /// * `center` - A `Point` that represents the center of the circle.
    /// * `radius` - A `f64` that represents the radius of the circle.
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }

    /// Calculate the perimeter (circumference) of the circle.
    pub fn perimeter(&self) -> f64 {
        PI * self.radius * 2 as f64
    }
}
