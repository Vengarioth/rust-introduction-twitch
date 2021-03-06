use std::{cell::Cell, rc::Rc, sync::Arc};

extern crate my_library;

use my_library::Point;

mod async_await;

#[derive(Debug, Clone)]
pub enum Shape {
    Triangle { a: Point, b: Point, c: Point },
    Circle { center: Point, radius: f32 },
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

    let d = a * b;
    dbg!(d);

    let mut c = a.add(b);

    dbg!(a, b, c);

    move_point(&mut c, 0.5, 0.5);

    dbg!(c);

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

    with_threads();

    async_await::main().unwrap();
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

fn move_point(point: &mut Point, x: f32, y: f32) {
    point.x += x;
    point.y += y;
}

struct Example<T> {
    owned: T,
    boxed: Box<T>,
    rc: Rc<T>,
    rc_mut: Rc<Cell<T>>,
    arc: Arc<T>,
    arc_mut: Arc<Cell<T>>,
}

#[derive(Debug)]
struct Item {
    items: Vec<Point>,
}

impl Item {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

fn with_threads() {
    use std::thread;

    let item = Item::new();

    thread::spawn(move || {
        dbg!(item);
    });

    thread::sleep(std::time::Duration::from_millis(1000));

    take_fn(|| {
        dbg!(1 + 2);
    });

    take_fn_once(|| {
        dbg!(1 + 2);
    });

    take_fn_mut(|| {
        dbg!(1 + 2);
    });
}

fn take_fn<F: Fn()>(f: F) {
    f();
    f();
    f();
    f();
}

fn take_fn_once<F: FnOnce()>(f: F) {
    f();
}

fn take_fn_mut_impl(mut f: impl FnMut()) {
    f();
}

fn take_fn_mut<F: FnMut()>(mut f: F) {
    f();
}

fn take_fn_mut_where<F>(mut f: F)
where
    F: FnMut(),
    Vec<F>: Send,
{
    f();
}
