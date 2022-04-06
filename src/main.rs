
struct Point {
    x: f32,
    y: f32,
}

struct Edge {
    p1: Point,
    p2: Point,
}

struct Rectangle {
    origin: Point,
    width: f32,
    height: f32,
}

struct Circle {
    origin: Point,
    radius: f32,
}

fn main() {

    let e1 = Edge{
        p1: Point { x: 0.0, y: 0.0 },
        p2: Point { x: 1.0, y: 1.0 },
    };
    println!("{}", e1.p1.x);
    println!("{}", e1.p2.x);

    let r1 = Rectangle{
        origin: Point { x: 0.0, y: 0.0 },
        width: 10.0,
        height: 10.0,
    };
    println!("Rectangle - origin: ({}, {})", r1.origin.x, r1.origin.y);
    println!("Rectangle - width: {}", r1.width);
    println!("Rectangle - height: {}", r1.height);

    let c1 = Circle{
        origin: Point { x: 0.0, y: 0.0 },
        radius: 2.0,
    };
    println!("Circle - origin: ({}, {})", c1.origin.x, c1.origin.y);
    println!("Circle - radius: {}", c1.radius);
}
