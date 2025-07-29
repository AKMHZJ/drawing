use rand::Rng;
use raster::{ Color, Image };

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

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
        // println!("{x}, {y}");
        Point {
            x,
            y,
        }
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
        Line {
            start: s,
            end: e,
        }
    }
}

// impl Drawable for Line {
//     fn draw(&self, image: &mut Image) {
//         let dx = (self.end.x - self.start.x).abs().max(self.start.x);
//         let dy = (self.end.y - self.start.y).abs();
//         let mut m = 0;
//         if dx == 0 {
//             m = 0
//         } else {
//             m = dy / dx;
//         }
        
//         let coef = self.start.y - (m * self.start.x);
//         if dx > dy {
//             for i in 0..=dx {
//                 let y = m * (self.start.x + i) + coef;
//                 image.display(self.start.x + i, y, self.color());
//             }
//         } else {
//             for i in 0..=dy {
//                 let x = ((self.start.y+i)-coef)/m;
//                 image.display(x,self.start.y+i,self.color());
//             }
//         }
//     }
// }


impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();

        let dx_f = (self.end.x - self.start.x) as f64;
        let dy_f = (self.end.y - self.start.y) as f64;
        let m = if dx != 0 { dy_f / dx_f } else { 0.0 };
        let coef = self.start.y as f64 - m * self.start.x as f64;

        let step_x = if self.end.x >= self.start.x { 1 } else { -1 };
        let step_y = if self.end.y >= self.start.y { 1 } else { -1 };

        if dx > dy {
            for i in 0..=dx {
                let x = self.start.x + i * step_x;
                let y = (m * x as f64 + coef).round() as i32;
                image.display(x, y, self.color());
            }
        } else {
            for i in 0..=dy {
                let y = self.start.y + i * step_y;
                let x = if m != 0.0 {
                    ((y as f64 - coef) / m).round() as i32
                } else {
                    self.start.x
                };
                image.display(x, y, self.color());
            }
        }
    }
}