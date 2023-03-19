use crate::Point;

/// An iterator over the points in a polygon.
pub struct PolygonIter<'a> {
    polygon: &'a Polygon,
    index: usize,
}

impl<'a> Iterator for PolygonIter<'a> {
    type Item = &'a Point;

    /// Return the next point in the polygon iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.polygon.points.len() {
            self.index += 1;
            Some(&self.polygon.points[self.index - 1])
        } else {
            None
        }
    }
}

/// A struct representing a polygon.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    /// Create a new empty polygon.
    pub fn new() -> Self {
        Self { points: vec![] }
    }

    /// Add a point to the polygon.
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    /// Find the left-most point of the polygon.
    pub fn left_most_point(&self) -> Option<Point> {
        self.points
            .iter()
            .reduce(|a, b| if a.x <= b.x { a } else { b })
            .copied()
    }

    /// Find the right-most point of the polygon.
    pub fn right_most_point(&self) -> Option<Point> {
        self.points
            .iter()
            .reduce(|a, b| if a.x >= b.x { a } else { b })
            .copied()
    }

    /// Create a new iterator over the points in the polygon.
    pub fn iter(&self) -> PolygonIter {
        PolygonIter {
            polygon: &self,
            index: 0,
        }
    }

    /// Calculate the perimeter of the polygon.
    pub fn perimeter(&self) -> f64 {
        let mut wraparound = self.points.clone();
        wraparound.push(self.points.first().unwrap().clone());
        wraparound
            .windows(2)
            .map(|pair| pair[0].dist(pair[1].clone()))
            .sum()
    }
}
