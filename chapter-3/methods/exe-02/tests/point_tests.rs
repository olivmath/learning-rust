use exe_02::Point;

#[test]
fn test_point_addition() {
    let p1 = Point::new(2, 3);
    let p2 = Point::new(4, 5);
    let result = p1 + p2;
    let expected = Point::new(6, 8);
    assert_eq!(result, expected);
}

#[test]
fn test_point_origin() {
    let origin = Point::origin();
    let expected = Point::new(0, 0);
    assert_eq!(origin, expected);
}

#[test]
fn test_point_new() {
    let point = Point::new(2, 3);
    assert_eq!(point.x, 2);
    assert_eq!(point.y, 3);
}

#[test]
fn test_point_magnitude() {
    let p1 = Point::new(3, 4);
    let p1_magnitude = p1.magnitude();
    assert_eq!(p1_magnitude, 5.0);

    let p2 = Point::new(0, 0);
    let p2_magnitude = p2.magnitude();
    assert_eq!(p2_magnitude, 0.0);
}

#[test]
fn test_point_dist() {
    let p1 = Point::new(2, 3);
    let p2 = Point::new(4, 5);
    let dist = p1.dist(p2);
    let expected = ((4u32 - 2u32).pow(2) + (5u32 - 3u32).pow(2)) as f64;
    let expected = expected.sqrt();
    assert_eq!(dist, expected);
}
