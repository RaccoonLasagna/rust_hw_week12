use std::f64::consts::PI;

#[derive(Debug)]
enum Shape {
    // x, y, r
    Circle (
        f64,
        f64,
        f64,
    ),
    // x, y, w, h
    Rectangle (
        f64,
        f64,
        f64,
        f64,
    ),
    Triangle (
        (f64, f64),
        (f64, f64),
        (f64, f64),
    )
}

impl Shape{
    fn rep_string(&self) -> String{
        match self{
            Shape::Circle(x, y, r) => {
                return format!("<Circle: {}, {}, {}>", x, y, r)
            },
            Shape::Rectangle(x, y, w, h) => {
                return format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h)
            },
            Shape::Triangle((x1, y1), (x2, y2), (x3, y3)) => {
                return format!("<Triangle: {}, {}, {}, {}, {}, {}", x1, y1, x2, y2, x3, y3)
            },
        }
    }
    fn area(&self) -> f64{
        match self{
            Shape::Circle(_, _, r) => {
                return PI * r.powf(2.)
            },
            Shape::Rectangle(_, _, w, h) => {
                return w*h
            },
            Shape::Triangle((x1, y1), (x2, y2), (x3, y3)) => {
                return 0.5 * ((x1-x3)*(y2-y1)-(x1-x2)*(y3-y1))
            },
        }
    }
}

fn main() {
    let testri = Shape::Triangle((0.,0.), (10.,0.), (5.,5.));
    println!("{}, {}", testri.rep_string(), testri.area());

}