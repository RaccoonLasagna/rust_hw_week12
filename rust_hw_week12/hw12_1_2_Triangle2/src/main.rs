use std::f64::consts::PI;

trait Shape {
    fn rep_string(&self) -> String;
    fn area(&self) -> f64;
    fn clone_shape(&self) -> Box<dyn Shape>;
}

trait Clone {
    fn clone(&self) -> Vec<Box<dyn Shape>>;
}

#[derive(Clone)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

#[derive(Clone)]
struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[derive(Clone)]
struct Triangle {
    point1: (f64, f64),
    point2: (f64, f64),
    point3: (f64, f64),
}

impl Shape for Circle {
    fn rep_string(&self) -> String {
        return format!("<Circle: {}, {}, {}>", self.x, self.y, self.r);
    }
    fn area(&self) -> f64 {
        return self.r.powf(2.0) * PI;
    }
    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) // Create a clone of the Circle and box it
    }
}

impl Circle {
    fn new(x: f64, y: f64, r: f64) -> Box<dyn Shape> {
        Box::new(Circle {
            x: x,
            y: y,
            r: r,
        })
    }
}

impl Shape for Rectangle {
    fn rep_string(&self) -> String {
        return format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.w, self.h);
    }
    fn area(&self) -> f64 {
        return self.w * self.h;
    }
    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Rectangle {
    fn new(x: f64, y: f64, w: f64, h: f64) -> Box<dyn Shape> {
        Box::new(Rectangle {
            x: x,
            y: y,
            w: w,
            h: h,
        })
    }
}

impl Shape for Triangle {
    fn rep_string(&self) -> String {
        return format!("<Triangle: {}, {}, {}, {}, {}, {}>", self.point1.0, self.point1.1, self.point2.0, self.point2.1, self.point3.0, self.point3.1);
    }
    fn area(&self) -> f64 {
        return 0.5 * ((self.point1.0-self.point3.0)*(self.point2.1-self.point1.1)-(self.point1.0-self.point2.0)*(self.point3.1-self.point1.1));
    }
    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Triangle {
    fn new(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Box<dyn Shape> {
        Box::new(Triangle {
            point1: (x1, y1),
            point2: (x2, y2),
            point3: (x3, y3),
        })
    }
}

impl Clone for Vec<Box<dyn Shape>> {
    fn clone(&self) -> Vec<Box<dyn Shape>> {
        let mut return_vec = Vec::new();
        for boxes in self.iter() {
            return_vec.push(boxes.clone_shape());
        }
        return_vec
    }
}

fn main() {
    let testtriangle = Triangle::new(0., 0., 10., 0., 5., 5.);
    let testtriangleclone = testtriangle.clone_shape();
    let testtriangleclone2 = testtriangle.clone_shape();
    println!("{}, {}", testtriangle.rep_string(), testtriangle.area());
    let vectriangle = vec![testtriangle, testtriangleclone, testtriangleclone2];
    let vectriangleclone = vectriangle.clone();
}

