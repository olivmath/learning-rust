use exe_02::Point;
use exe_02::Polygon;

#[test]
fn test_polygon_new() {
    let poly = Polygon::new();
    assert!(poly.points.is_empty());
}

#[test]
fn test_polygon_add_point() {
    let mut poly = Polygon::new();
    poly.add_point(Point::new(2, 3));
    assert_eq!(poly.points.len(), 1);
    assert_eq!(poly.points[0], Point::new(2, 3));
}

#[test]
fn test_polygon_left_most_point() {
    let p1 = Point::new(12, 13);
    let p2 = Point::new(16, 16);

    let mut poly = Polygon::new();
    poly.add_point(p1);
    poly.add_point(p2);
    assert_eq!(poly.left_most_point(), Some(p1));
}

#[test]
fn test_polygon_right_most_point() {
    let p1 = Point::new(12, 13);
    let p2 = Point::new(16, 16);

    let mut poly = Polygon::new();
    poly.add_point(p1);
    poly.add_point(p2);
    assert_eq!(poly.right_most_point(), Some(p2));
}

#[test]
fn test_polygon_iter() {
    let p1 = Point::new(12, 13);
    let p2 = Point::new(16, 16);

    let mut poly = Polygon::new();
    poly.add_point(p1);
    poly.add_point(p2);

    let points = poly.iter().cloned().collect::<Vec<_>>();
    assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
}

#[test]
fn test_polygon_perimeter() {
    let p1 = Point::new(0, 0);
    let p2 = Point::new(0, 3);
    let p3 = Point::new(4, 0);

    let mut poly = Polygon::new();
    poly.add_point(p1);
    poly.add_point(p2);
    poly.add_point(p3);
    assert_eq!(poly.perimeter(), 12.0);
}
