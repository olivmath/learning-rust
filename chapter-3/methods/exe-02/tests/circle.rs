use exe_02::Circle;
use exe_02::Point;
use std::f64::consts::PI;

fn round_two_digits(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

#[test]
fn test_circle_new() {
    let center = Point::new(1, 1);
    let radius = 5.0;
    let circle = Circle::new(center, radius);
    assert_eq!(circle.center, center);
    assert_eq!(circle.radius, radius);
}

#[test]
fn test_circle_perimeter() {
    let center = Point::new(1, 1);
    let radius = 5.0;
    let circle = Circle::new(center, radius);
    let expected_perimeter = 2.0 * PI * radius;
    assert_eq!(round_two_digits(circle.perimeter()), round_two_digits(expected_perimeter));
}
