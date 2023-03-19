#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

fn main() {
    let p1: Point = Point(1, 10);
    let p2: Point = Point(20, 20);
    let p3: &Point;
    {
        p3 = left_most(&p1, &p2);
    }
    println!("left-most point: {:?}", p3);
}
