use rand::Rng;
use raster::{ Color, Image };

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color())
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn random(max_x: i32, max_y: i32) -> Point {
        let mut g = rand::rng();
        let x = g.random_range(0..max_x);
        let y = g.random_range(0..max_y);
        Point::new(x, y)
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(s: Point, e: Point) -> Line {
        Line { start: s, end: e }
    }

    pub fn random(max_x: i32, max_y: i32) -> Line {
        let s = Point::random(max_x, max_y);
        let e = Point::random(max_x, max_y);
        Line::new(s, e)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        let steps = dx.abs().max(dy.abs());

        let x_inc = (dx as f32) / (steps as f32);
        let y_inc = (dy as f32) / (steps as f32);

        let mut x: f32 = self.start.x as f32;
        let mut y: f32 = self.start.y as f32;

        for _i in 0..=steps {
            image.display(x.round() as i32, y.round() as i32, self.color().clone());
            x += x_inc;
            y += y_inc;
        }
    }
}

/*******************************Triangle******************************/
pub struct Triangle {
    start: Point,
    middle: Point,
    end: Point,
}

impl Triangle {
    pub fn new(s: &Point, m: &Point, e: &Point) -> Triangle {
        Triangle { start: *s, middle: *m, end: *e }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.start, self.middle).draw(image);
        Line::new(self.middle, self.end).draw(image);
        Line::new(self.end, self.start).draw(image);
    }
}

/*******************************Rectangle******************************/
pub struct Rectangle {
    point1: Point,
    point2: Point,
}

impl Rectangle {
    pub fn new(point1: &Point, point2: &Point) -> Rectangle {
        Rectangle {
            point1: Point::new(point1.x, point1.y),
            point2: Point::new(point2.x, point2.y),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let point3 = Point::new(self.point2.x, self.point1.y);
        let point4 = Point::new(self.point1.x, self.point2.y);

        Line::new(self.point1, point3).draw(image);
        Line::new(point3, self.point2).draw(image);
        Line::new(self.point2, point4).draw(image);
        Line::new(point4, self.point1).draw(image);
    }
}

/*******************************Circle******************************/
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(x_rng: i32, y_rng: i32) -> Self {
        let mut g = rand::rng();
        Self::new(Point::random(x_rng, y_rng), g.random_range(0..x_rng.min(y_rng) / 2))
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius;

        let mut x = cx;
        let mut y = cy - r;
        let color = self.color();

        while y <= cy {
            image.display(x, y, color.clone());
            image.display(2 * cx - x, y, color.clone());
            image.display(x, 2 * cy - y, color.clone());
            image.display(2 * cx - x, 2 * cy - y, color.clone());

            let a = distance((cx, cy), (x + 1, y));
            let b = distance((cx, cy), (x, y + 1));
            let c = distance((cx, cy), (x + 1, y + 1));

            let closest = closest_to_target(a, b, c, r as f64);

            if closest == a {
                x += 1;
            } else if closest == b {
                y += 1;
            } else {
                x += 1;
                y += 1;
            }
        }
    }

    fn color(&self) -> Color {
        let mut g = rand::rng();
        Color {
            r: g.random_range(0..=255),
            g: g.random_range(0..=255),
            b: g.random_range(0..=255),
            a: g.random_range(100..=255),
        }
    }
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    ((dx.pow(2) + dy.pow(2)) as f64).sqrt()
}

fn closest_to_target(a: f64, b: f64, c: f64, target: f64) -> f64 {
    let diff_a = (a - target).abs();
    let diff_b = (b - target).abs();
    let diff_c = (c - target).abs();

    if diff_a <= diff_b && diff_a <= diff_c {
        a
    } else if diff_b <= diff_c {
        b
    } else {
        c
    }
}