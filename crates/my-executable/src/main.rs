use my_library::*;

#[derive(Debug, Clone, Copy)]
struct Point {
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

fn main() {
    println!("Hello, world!");

    let a = Point::new(0.0, 1.0);
    let b = Point::new(1.0, 0.0);

    let c = a.add(b);

    dbg!(a, b, c);
}
