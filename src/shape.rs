use std::f32::consts::PI;

fn round(x: f32, decimal: i32) -> f32 {
    let f: f32 = 10.0;
    let s = f.powi(decimal);
    (x * s).round() / s 
}

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
        self.x = round(self.x, 3);
        self.y = y*theta.cos() + x*theta.sin();
        self.y = round(self.y, 3);
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn points() {
        let mut p1 = Point::new(1.0, 0.0);
        assert_eq!(p1.x, 1.0);
        assert_eq!(p1.y, 0.0);

        p1.translate(2.0, 0.0);
        assert_eq!(p1.x, 3.0);
        assert_eq!(p1.y, 0.0);

        p1.rotate(90.0);
        assert_eq!(p1.x, 0.0);
        assert_eq!(p1.y, 3.0);
    }

    #[test]
    fn edges() {
        let e1 = Edge{
            p1: Point { x: 0.0, y: 0.0 },
            p2: Point { x: 1.0, y: 1.0 },
        };
        assert_eq!(e1.p1.x, 0.0);
        assert_eq!(e1.p1.y, 0.0);
        assert_eq!(e1.p2.x, 1.0);
        assert_eq!(e1.p2.y, 1.0);
    }

    #[test]
    fn rectanlges() {
        let mut r1 = Rectangle{
            origin: Point { x: 0.0, y: 0.0 },
            width: 10.0,
            height: 10.0,
        };
        assert_eq!(r1.origin.x, 0.0);
        assert_eq!(r1.origin.y, 0.0);

        r1.translate(2.0, 0.0);
        assert_eq!(r1.origin.x, 2.0);
        assert_eq!(r1.origin.y, 0.0);
    }

    #[test]
    fn circles() {
        let mut c1 = Circle{
            origin: Point { x: 0.0, y: 0.0 },
            radius: 2.0,
        };
        assert_eq!(c1.origin.x, 0.0);
        assert_eq!(c1.origin.y, 0.0);

        c1.translate(2.0, 0.0);
        assert_eq!(c1.origin.x, 2.0);
        assert_eq!(c1.origin.y, 0.0);
    }
}
