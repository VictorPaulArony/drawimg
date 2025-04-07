use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// POINT
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0)
    }
}

// LINE
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line::new(Point::random(width, height), Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let (mut x0, mut y0) = (self.start.x, self.start.y);
        let (x1, y1) = (self.end.x, self.end.y);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            image.display(x0, y0, self.color());
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0)
    }
}

// RECTANGLE
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: &Point, bottom_right: &Point) -> Self {
        Self { top_left: top_left.clone(), bottom_right: bottom_right.clone() }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let tl = &self.top_left;
        let tr = &Point::new(self.bottom_right.x, self.top_left.y);
        let br = &self.bottom_right;
        let bl = &Point::new(self.top_left.x, self.bottom_right.y);

        Line::new(tl.clone(), tr.clone()).draw(image);
        Line::new(tr.clone(), br.clone()).draw(image);
        Line::new(br.clone(), bl.clone()).draw(image);
        Line::new(bl.clone(), tl.clone()).draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 255)
    }
}

// TRIANGLE
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self { a: a.clone(), b: b.clone(), c: c.clone() }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.a.clone(), self.b.clone()).draw(image);
        Line::new(self.b.clone(), self.c.clone()).draw(image);
        Line::new(self.c.clone(), self.a.clone()).draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 0)
    }
}

// CIRCLE
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(10..50);
        Circle::new(center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let Point { x: cx, y: cy } = self.center;
        let r = self.radius;

        let mut x = r;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            let points = [
                (cx + x, cy + y),
                (cx + y, cy + x),
                (cx - y, cy + x),
                (cx - x, cy + y),
                (cx - x, cy - y),
                (cx - y, cy - x),
                (cx + y, cy - x),
                (cx + x, cy - y),
            ];

            for (px, py) in points {
                image.display(px, py, self.color());
            }

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 255)
    }
}