use std::f32::consts::PI;

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

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.origin.x += x;
        self.origin.y += y;
    }
}

impl Circle {
    fn area(&self) -> f32 {
        PI * self.radius.powi(2)
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.origin.x += x;
        self.origin.y += y;
    }
}

fn main() {

    let mut p1 = Point::new(0.0, 0.0);
    println!("Point - ({}, {})", p1.x, p1.y);
    p1.translate(2.0, 0.0);
    println!("Point - translated origin: ({}, {})", p1.x, p1.y);

    let e1 = Edge{
        p1: Point { x: 0.0, y: 0.0 },
        p2: Point { x: 1.0, y: 1.0 },
    };
    println!("{}", e1.p1.x);
    println!("{}", e1.p2.x);

    let mut r1 = Rectangle{
        origin: Point { x: 0.0, y: 0.0 },
        width: 10.0,
        height: 10.0,
    };
    println!("Rectangle - origin: ({}, {})", r1.origin.x, r1.origin.y);
    println!("Rectangle - width: {}", r1.width);
    println!("Rectangle - height: {}", r1.height);
    println!("Rectangle - area: {}", r1.area());

    r1.translate(2.0, 0.0);
    println!("Rectangle - translated origin: ({}, {})", r1.origin.x, r1.origin.y);

    let mut c1 = Circle{
        origin: Point { x: 0.0, y: 0.0 },
        radius: 2.0,
    };
    println!("Circle - origin: ({}, {})", c1.origin.x, c1.origin.y);
    println!("Circle - radius: {}", c1.radius);
    println!("Circle - area: {}", c1.area());

    c1.translate(2.0, 0.0);
    println!("Circle - translated origin: ({}, {})", c1.origin.x, c1.origin.y);
}
