/// A struct representing a point in a 2D plane.
#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    /// Add two points together component-wise.
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    /// Return the origin point (0, 0).
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Create a new point with the given x and y coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Calculate the magnitude (Euclidean distance) of the point from the origin.
    pub fn magnitude(&self) -> f64 {
        // pitagoras
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }

    /// Calculate the Euclidean distance between two points.
    pub fn dist(&self, other: Point) -> f64 {
        // distance between two points
        let x = self.x - other.x;
        let y = self.y - other.y;

        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    }
}
