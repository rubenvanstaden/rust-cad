mod shape {

    use std::f32::consts::PI;

    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    pub struct Edge {
        pub p1: Point,
        pub p2: Point,
    }

    pub struct Rectangle {
        pub origin: Point,
        pub width: f32,
        pub height: f32,
    }

    pub struct Circle {
        pub origin: Point,
        pub radius: f32,
    }

    fn deg2rad(angle: f32) -> f32 {
        angle * PI/180.0
    }

    impl Point {
        pub fn new(x: f32, y: f32) -> Point {
            Point { x, y }
        }

        pub fn rotate(&mut self, angle: f32) {
            let theta = deg2rad(angle);
            let (x, y) = (self.x, self.y);
            self.x = x*theta.cos() - y*theta.sin();
            self.y = y*theta.cos() + x*theta.sin();
        }

        pub fn translate(&mut self, x: f32, y: f32) {
            self.x += x;
            self.y += y;
        }
    }

    impl Rectangle {
        pub fn area(&self) -> f32 {
            self.width * self.height
        }

        pub fn translate(&mut self, x: f32, y: f32) {
            self.origin.x += x;
            self.origin.y += y;
        }
    }

    impl Circle {
        pub fn area(&self) -> f32 {
            PI * self.radius.powi(2)
        }

        pub fn translate(&mut self, x: f32, y: f32) {
            self.origin.x += x;
            self.origin.y += y;
        }
    }
}

fn main() {

    let mut p1 = shape::Point::new(1.0, 0.0);
    println!("Point - ({}, {})", p1.x, p1.y);
    p1.translate(2.0, 0.0);
    println!("Point - translated origin: ({}, {})", p1.x, p1.y);
    p1.rotate(90.0);
    println!("Point - rotated origin: ({}, {})", p1.x, p1.y);

    let e1 = shape::Edge{
        p1: shape::Point { x: 0.0, y: 0.0 },
        p2: shape::Point { x: 1.0, y: 1.0 },
    };
    println!("{}", e1.p1.x);
    println!("{}", e1.p2.x);
    
    let mut r1 = shape::Rectangle{
        origin: shape::Point { x: 0.0, y: 0.0 },
        width: 10.0,
        height: 10.0,
    };
    println!("Rectangle - origin: ({}, {})", r1.origin.x, r1.origin.y);
    println!("Rectangle - width: {}", r1.width);
    println!("Rectangle - height: {}", r1.height);
    println!("Rectangle - area: {}", r1.area());
    
    r1.translate(2.0, 0.0);
    println!("Rectangle - translated origin: ({}, {})", r1.origin.x, r1.origin.y);
    
    let mut c1 = shape::Circle{
        origin: shape::Point { x: 0.0, y: 0.0 },
        radius: 2.0,
    };
    println!("Circle - origin: ({}, {})", c1.origin.x, c1.origin.y);
    println!("Circle - radius: {}", c1.radius);
    println!("Circle - area: {}", c1.area());
    
    c1.translate(2.0, 0.0);
    println!("Circle - translated origin: ({}, {})", c1.origin.x, c1.origin.y);
}
