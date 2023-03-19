use exe_02::Circle;
use exe_02::Point;
use exe_02::Polygon;
use exe_02::Shape;
use std::f64::consts::PI;

fn round_two_digits(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

#[test]
fn test_shape_perimeter_polygon() {
    let mut poly = Polygon::new();
    poly.add_point(Point::new(12, 13));
    poly.add_point(Point::new(17, 11));
    poly.add_point(Point::new(16, 16));
    let shape = Shape::Polygon(poly);
    let expected_perimeter = 15.48;
    assert_eq!(round_two_digits(shape.perimeter()), round_two_digits(expected_perimeter));
}

#[test]
fn test_shape_perimeter_circle() {
    let center = Point::new(10, 20);
    let radius = 5.0;
    let circle = Circle::new(center, radius);
    let shape = Shape::Circle(circle);
    let expected_perimeter = 2.0 * PI * radius;
    assert_eq!(round_two_digits(shape.perimeter()), round_two_digits(expected_perimeter));
}

#[test]
fn test_shape_from_polygon() {
    let mut poly = Polygon::new();
    poly.add_point(Point::new(12, 13));
    poly.add_point(Point::new(17, 11));
    poly.add_point(Point::new(16, 16));
    let shape = Shape::from(poly.clone());
    if let Shape::Polygon(p) = shape {
        assert_eq!(p, poly);
    } else {
        panic!("Shape should be a Polygon");
    }
}

#[test]
fn test_shape_from_circle() {
    let center = Point::new(10, 20);
    let radius = 5.0;
    let circle = Circle::new(center, radius);
    let shape = Shape::from(circle.clone());
    if let Shape::Circle(c) = shape {
        assert_eq!(c, circle);
    } else {
        panic!("Shape should be a Circle");
    }
}
