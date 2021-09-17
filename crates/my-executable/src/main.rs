use my_library::*;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Triangle {
        a: Point,
        b: Point,
        c: Point,
    },

    Circle {
        center: Point,
        radius: f32,
    },
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Triangle { .. } => 1.0,
            Shape::Circle { .. } => 1.0,
        }
    }
}

pub trait Area {
    fn area(&self) -> f32;
}

impl Area for Shape {
    fn area(&self) -> f32 {
        self.area()
    }
}

fn main() {
    println!("Hello, world!");

    let a = Point::new(0.0, 1.0);
    let b = Point::new(1.0, 0.0);

    let c = a.add(b);

    dbg!(a, b, c);

    let triangle = Shape::Triangle {
        a: Point::new(0.0, 0.0),
        b: Point::new(1.0, 0.0),
        c: Point::new(0.5, 1.0),
    };

    let circle = Shape::Circle {
        center: Point::new(0.5, 0.5),
        radius: 1.0,
    };

    dbg!(triangle.area(), circle.area());

    print_area(&circle);
    print_area_dyn(&triangle);
    
    let boxed_area: Box<dyn Area> = Box::new(triangle);
    print_area_trait_object(boxed_area);
}

fn print_area<T: Area>(has_area: &T) {
    println!("T has an area of {}", has_area.area());
}

fn print_area_dyn(has_area: &dyn Area) {
    println!("dyn has an area of {}", has_area.area());
}

fn print_area_trait_object(boxed_area: Box<dyn Area>) {
    println!("box has an area of {}", boxed_area.area());
}
